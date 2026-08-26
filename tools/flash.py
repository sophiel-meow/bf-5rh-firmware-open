#!/usr/bin/env python3
import argparse
import struct
import sys
import time
from pathlib import Path

import serial

HEADER = 0xAA
FOOTER = 0xEF

CMD_HANDSHAKE = 0x01
CMD_UPDATE = 0x03
CMD_UPDATE_DATA_PACKAGES = 0x04
CMD_UPDATE_DATA_PACKAGES2 = 0x05
CMD_UPDATE_END = 0x45

ACK = 0x06

CHUNK_SIZE = 1024
REGION1_MAX_BYTES = 0xF000  # 60KB application area, bootloader-enforced
REGION2_MAX_BYTES = 0x1000  # 4KB system-bootloader scratch area
FRAME_TIMEOUT_S = 2.0
MAX_RETRIES = 5
READ_SLICE_TIMEOUT_S = 0.03

ERROR_NAMES = {
    0xE1: "Handshake/command-class error",
    0xE2: "Sequence number or length mismatch",
    0xE3: "Flash write failed",
    0xE5: "Unknown command",
}


def _error_name(code: int) -> str:
    return ERROR_NAMES.get(code, f"Unknown error code 0x{code:02X}")


def _crc16_xmodem(data: bytes) -> int:
    crc = 0
    for byte in data:
        crc ^= byte << 8
        for _ in range(8):
            crc = ((crc << 1) ^ 0x1021) if (crc & 0x8000) else (crc << 1)
        crc &= 0xFFFF
    return crc


def _pack(cmd: int, cmd_args: int, data: bytes) -> bytes:
    body = bytes([cmd, cmd_args, (len(data) >> 8) & 0xFF, len(data) & 0xFF]) + data
    crc = _crc16_xmodem(body)
    return bytes([HEADER]) + body + bytes([(crc >> 8) & 0xFF, crc & 0xFF, FOOTER])


def _read_exact_deadline(port: serial.Serial, n: int, deadline: float) -> bytes | None:
    buf = bytearray()
    while len(buf) < n:
        if time.monotonic() > deadline:
            return None
        chunk = port.read(n - len(buf))
        if chunk:
            buf.extend(chunk)
    return bytes(buf)


def _read_frame(port: serial.Serial, timeout_s: float):
    """Returns ("frame", cmd, cmd_args) | ("timeout", saw_bytes) | ("malformed", msg)."""
    deadline = time.monotonic() + timeout_s
    saw_bytes = False

    while True:
        b = _read_exact_deadline(port, 1, deadline)
        if b is None:
            return ("timeout", saw_bytes)
        saw_bytes = True
        if b[0] == HEADER:
            break

    header_body = _read_exact_deadline(port, 4, deadline)
    if header_body is None:
        return ("timeout", True)
    cmd, cmd_args, len_hi, len_lo = header_body
    data_len = (len_hi << 8) | len_lo

    data = b""
    if data_len:
        data = _read_exact_deadline(port, data_len, deadline)
        if data is None:
            return ("timeout", True)

    crc_bytes = _read_exact_deadline(port, 2, deadline)
    if crc_bytes is None:
        return ("timeout", True)
    footer = _read_exact_deadline(port, 1, deadline)
    if footer is None:
        return ("timeout", True)
    if footer[0] != FOOTER:
        print(f"    [INFO] frame footer was 0x{footer[0]:02X}")

    body = bytes([cmd, cmd_args, len_hi, len_lo]) + data
    expected_crc = _crc16_xmodem(body)
    got_crc = (crc_bytes[0] << 8) | crc_bytes[1]
    if expected_crc != got_crc:
        return (
            "malformed",
            f"CRC mismatch: expected {expected_crc:04X}, got {got_crc:04X}",
        )

    return ("frame", cmd, cmd_args)


def _send_command(
    port: serial.Serial,
    cmd: int,
    cmd_args: int,
    data: bytes,
    retries: int,
    tolerate_no_response: bool,
) -> tuple[int, int]:
    frame_bytes = _pack(cmd, cmd_args, data)
    last_error = "(no attempts made)"
    saw_any_bytes = False

    for _attempt in range(retries):
        port.reset_input_buffer()
        try:
            port.write(frame_bytes)
        except serial.SerialException as e:
            last_error = f"write failed: {e}"
            continue

        outcome = _read_frame(port, FRAME_TIMEOUT_S)
        kind = outcome[0]
        if kind == "frame":
            _, resp_cmd, resp_cmd_args = outcome
            saw_any_bytes = True
            if resp_cmd_args == ACK:
                return resp_cmd, resp_cmd_args
            err = _error_name(resp_cmd)
            if resp_cmd == 0xE2:
                last_error = err
                continue
            raise RuntimeError(f"denied: cmd=0x{cmd:02X}: {err}")
        elif kind == "timeout":
            _, saw_bytes = outcome
            if saw_bytes:
                saw_any_bytes = True
            last_error = "timed out waiting for response"
        else:  # "malformed"
            _, msg = outcome
            saw_any_bytes = True
            last_error = msg

    if tolerate_no_response and not saw_any_bytes:
        print("    (no ACK)")
        return cmd, ACK

    raise RuntimeError(f"cmd=0x{cmd:02X} failed after {retries} retries: {last_error}")


def _parse_bf(path: str) -> tuple[bytes, bytes]:
    """Returns (region1_bytes, region2_bytes); region2 is b"" if absent."""
    raw = Path(path).read_bytes()
    if len(raw) < 16:
        raise RuntimeError(f"{path}: too small to be a .BF file ({len(raw)} bytes)")
    cnt_parts, len_fw, len_data = struct.unpack_from(">BLL", raw, 0)
    if cnt_parts not in (1, 2):
        raise RuntimeError(
            f"{path}: unexpected region count {cnt_parts} in header (expected 1 or 2)"
        )
    region1 = raw[16 : 16 + len_fw]
    if len(region1) != len_fw:
        raise RuntimeError(
            f"{path}: header claims region1={len_fw}B but file only has {len(region1)}B"
        )
    region2 = b""
    if cnt_parts == 2:
        region2 = raw[16 + len_fw : 16 + len_fw + len_data]
        if len(region2) != len_data:
            raise RuntimeError(
                f"{path}: header claims region2={len_data}B but file only has {len(region2)}B"
            )
    return region1, region2


def _send_region(port: serial.Serial, label: str, payload: bytes) -> None:
    total_packages = -(-len(payload) // CHUNK_SIZE) if payload else 0
    print(f"{label}: {len(payload)} bytes, {total_packages} packages")
    last_printed_pct = -1
    for seq in range(total_packages):
        start = seq * CHUNK_SIZE
        end = min(start + CHUNK_SIZE, len(payload))
        chunk = payload[start:end]
        if len(chunk) < CHUNK_SIZE:
            chunk = chunk + bytes(CHUNK_SIZE - len(chunk))

        _send_command(port, CMD_UPDATE, seq & 0xFF, chunk, MAX_RETRIES, False)

        pct = (seq + 1) * 100 // total_packages
        if pct != last_printed_pct:
            print(f"\r    {pct}% ({seq + 1}/{total_packages})", end="", flush=True)
            last_printed_pct = pct
    if total_packages:
        print()


def flash(bf_path: str, port_name: str, baud: int) -> None:
    region1, region2 = _parse_bf(bf_path)

    if len(region1) > REGION1_MAX_BYTES:
        raise RuntimeError(
            f"region1 is {len(region1)} bytes, exceeds the 60KB application area "
            f"({REGION1_MAX_BYTES} bytes) -- bootloader will reject this with 0xE2"
        )
    if len(region2) > REGION2_MAX_BYTES:
        raise RuntimeError(
            f"region2 is {len(region2)} bytes, exceeds the 4KB system-bootloader area "
            f"({REGION2_MAX_BYTES} bytes) -- bootloader will reject this with 0xE2"
        )
    region1_packages = -(-len(region1) // CHUNK_SIZE)
    region2_packages = -(-len(region2) // CHUNK_SIZE) if region2 else 0
    if region1_packages > 255 or region2_packages > 255:
        raise RuntimeError(
            "more than 255 packages in one region -- package count is a single byte on the wire"
        )

    port = serial.Serial(
        port_name,
        baudrate=baud,
        bytesize=serial.EIGHTBITS,
        parity=serial.PARITY_NONE,
        stopbits=serial.STOPBITS_ONE,
        timeout=READ_SLICE_TIMEOUT_S,
    )
    port.dtr = True
    port.rts = True

    try:
        print("handshake...")
        _send_command(port, CMD_HANDSHAKE, 0, b"BOOTLOADER", MAX_RETRIES, False)

        print(
            f"region1 (application, 0x08001000+): declared packages: {region1_packages}"
        )
        _send_command(
            port,
            CMD_UPDATE_DATA_PACKAGES,
            0,
            bytes([region1_packages & 0xFF]),
            MAX_RETRIES,
            False,
        )
        _send_region(port, "sending region1", region1)

        if region2_packages:
            print(
                f"region2 (system bootloader, 0x1FFFE400+): declared packages: {region2_packages}"
            )
            _send_command(
                port,
                CMD_UPDATE_DATA_PACKAGES2,
                0,
                bytes([region2_packages & 0xFF]),
                MAX_RETRIES,
                False,
            )
            _send_region(port, "sending region2", region2)
        else:
            print(
                "region2: empty, skipping CMD_UPDATE_DATA_PACKAGES2 (unverified against real hardware)"
            )

        print("finished...")
        _send_command(port, CMD_UPDATE_END, 0, b"", 2, True)
        print("succeed!")
    finally:
        port.close()


def main() -> int:
    parser = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    parser.add_argument(
        "bf",
        help="wrapped+encrypted .BF firmware image (see uv5rm-wrap-tool / bao_fw_tool.py)",
    )
    parser.add_argument("port", help="serial port, e.g. /dev/ttyUSB0 or COM3")
    parser.add_argument("--baud", type=int, default=115200)
    args = parser.parse_args()

    try:
        flash(args.bf, args.port, args.baud)
    except Exception as e:
        print(f"\n[ERR] failed: {e}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
