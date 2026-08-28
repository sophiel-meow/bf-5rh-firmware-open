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
CMD_ERASE = 0x02
CMD_WRITE = 0x03
CMD_END = 0x45

ACK = 0x06
ERR_NAMES = {
    0xE1: "wrong length",
    0xE2: "not 4K",
    0xE5: "cmd error",
}

CHUNK_LEN = 512
SECTOR_SIZE = 4096
FRAME_TIMEOUT_S = 2.0
MAX_RETRIES = 5

OVERLAY_APP_ADDR = 0x14C000
OVERLAY_SLOT_SIZE = 16 * 1024
OVERLAY_SLOT_COUNT = 4


def crc16_xmodem(data: bytes) -> int:
    crc = 0
    for byte in data:
        crc ^= byte << 8
        for _ in range(8):
            crc = ((crc << 1) ^ 0x1021) if (crc & 0x8000) else (crc << 1)
        crc &= 0xFFFF
    return crc


def pack(cmd: int, data: bytes) -> bytes:
    body = bytes([cmd, 0, (len(data) >> 8) & 0xFF, len(data) & 0xFF]) + data
    crc = crc16_xmodem(body)
    return bytes([HEADER]) + body + bytes([(crc >> 8) & 0xFF, crc & 0xFF, FOOTER])


def read_exact(port: serial.Serial, n: int, deadline: float) -> bytes | None:
    buf = bytearray()
    while len(buf) < n:
        if time.monotonic() > deadline:
            return None
        chunk = port.read(n - len(buf))
        if chunk:
            buf.extend(chunk)
    return bytes(buf)


def read_response(port: serial.Serial, timeout_s: float) -> tuple[int, int] | None:
    deadline = time.monotonic() + timeout_s
    while True:
        b = read_exact(port, 1, deadline)
        if b is None:
            return None
        if b[0] == HEADER:
            break
    head = read_exact(port, 4, deadline)
    if head is None:
        return None
    cmd, cmdargs, len_hi, len_lo = head
    data_len = (len_hi << 8) | len_lo
    data = read_exact(port, data_len, deadline) if data_len else b""
    if data is None:
        return None
    tail = read_exact(port, 3, deadline)  # crc_hi crc_lo footer
    if tail is None:
        return None
    crc_hi, crc_lo, _footer = tail
    expected = crc16_xmodem(bytes([cmd, cmdargs, len_hi, len_lo]) + data)
    got = (crc_hi << 8) | crc_lo
    if got != expected:
        return None
    return cmd, cmdargs


def send(port: serial.Serial, cmd: int, data: bytes = b"") -> None:
    frame = pack(cmd, data)
    for attempt in range(MAX_RETRIES):
        port.reset_input_buffer()
        port.write(frame)
        resp = read_response(port, FRAME_TIMEOUT_S)
        if resp is None:
            continue
        resp_cmd, resp_cmdargs = resp
        if resp_cmdargs == ACK:
            return
        name = ERR_NAMES.get(resp_cmd, f"unknown 0x{resp_cmd:02X}")
        raise RuntimeError(f"cmd=0x{cmd:02X} denied: {name}")
    raise RuntimeError(f"NO ACK for cmd=0x{cmd:02X} after {MAX_RETRIES} retries")


def push(app_path: Path, port_name: str, baud: int, base_addr: int) -> None:
    payload = app_path.read_bytes()
    print(f"{app_path}: {len(payload)} Bytes, addr 0x{base_addr:06X}")

    port = serial.Serial(port_name, baudrate=baud, timeout=0.03)
    try:
        print("handshake...")
        send(port, CMD_HANDSHAKE)

        sector_start = base_addr - (base_addr % SECTOR_SIZE)
        sector_end = base_addr + len(payload)
        sector_end += (-sector_end) % SECTOR_SIZE
        n_sectors = (sector_end - sector_start) // SECTOR_SIZE
        print(
            f"earsing {n_sectors} sectors（0x{sector_start:06X}..0x{sector_end:06X}）"
        )
        for i in range(n_sectors):
            addr = sector_start + i * SECTOR_SIZE
            send(port, CMD_ERASE, struct.pack("<I", addr))
            print(f"\r    {i + 1}/{n_sectors}", end="", flush=True)
        print()

        n_chunks = -(-len(payload) // CHUNK_LEN)
        print(f"writing {len(payload)} Bytes, {n_chunks} chunks")
        for i in range(n_chunks):
            start = i * CHUNK_LEN
            chunk = payload[start : start + CHUNK_LEN]
            addr = base_addr + start
            send(port, CMD_WRITE, struct.pack("<I", addr) + chunk)
            print(f"\r    {i + 1}/{n_chunks}", end="", flush=True)
        print()

        print("done, rebooting")
        send(port, CMD_END)
        print("succeed.")
    finally:
        port.close()


def main() -> int:
    ap = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    ap.add_argument("app", type=Path, help=".app file")
    ap.add_argument("port", help="serial port eg. /dev/ttyUSB0 or COM3")
    ap.add_argument("--baud", type=int, default=115200)
    addr_group = ap.add_mutually_exclusive_group()
    addr_group.add_argument(
        "--slot",
        type=int,
        choices=range(OVERLAY_SLOT_COUNT),
        help=f"slot (0..{OVERLAY_SLOT_COUNT - 1})",
    )
    addr_group.add_argument(
        "--addr",
        type=lambda s: int(s, 0),
        help="SPI-NOR address",
    )
    args = ap.parse_args()

    if args.addr is not None:
        base_addr = args.addr
    else:
        slot = args.slot if args.slot is not None else 0
        base_addr = OVERLAY_APP_ADDR + slot * OVERLAY_SLOT_SIZE

    try:
        push(args.app, args.port, args.baud, base_addr)
    except Exception as e:
        print(f"\n[ERR] {e}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
