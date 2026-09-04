#!/usr/bin/env python3
from __future__ import annotations

import struct
import time

import serial

HEADER = 0xA6

CMD_READ = 0x52
CMD_WRITE = 0x57
CMD_END = 0x45
CMD_READ_LOGO = 0x4C
CMD_WRITE_LOGO = 0x6C
CMD_APP_ERASE = 0x41
CMD_APP_WRITE = 0x61
CMD_SAT_WRITE = 0x53
CMD_READ_FLASH_RAW = 0x44
CMD_ERROR = 0xEE

ERR_NAMES = {
    0x01: "bad CRC",
    0x02: "bad address",
    0x03: "bad length",
}

HANDSHAKE = b"PROGRAMBF-5RH"
ACK = 0x06
HANDSHAKE_ATTEMPTS = 5
HANDSHAKE_TIMEOUT_S = 1.0
FRAME_TIMEOUT_S = 2.0
MAX_RETRIES = 5

ERASE_TIMEOUT_S = 20.0

# src/flash_map.rs
OVERLAY_APP_ADDR = 0x14C000
OVERLAY_SLOT_SIZE = 32 * 1024
OVERLAY_SLOT_COUNT = 8
BOOT_LOGO_WIDTH = 160
BOOT_LOGO_HEIGHT = 128
BOOT_LOGO_SIZE = BOOT_LOGO_WIDTH * BOOT_LOGO_HEIGHT * 2

# src/cps/frame.rs
APP_CHUNK = 512
SAT_CHUNK = 512
BOOT_LOGO_CHUNK = 512

# sgp4mini::TABLE_BYTES (RECORD_SIZE=256 * MAX_SATELLITES=20)
SAT_TABLE_SIZE = 5120


def crc16_xmodem(data: bytes) -> int:
    crc = 0
    for byte in data:
        crc ^= byte << 8
        for _ in range(8):
            crc = ((crc << 1) ^ 0x1021) if (crc & 0x8000) else (crc << 1)
        crc &= 0xFFFF
    return crc


def encode_frame(cmd: int, addr: int, data: bytes) -> bytes:
    body = struct.pack(">BHH", cmd, addr, len(data)) + bytes(data)
    crc = crc16_xmodem(body)
    return bytes([HEADER]) + body + struct.pack(">H", crc)


class CpsError(RuntimeError):
    pass


def _read_exact(port: serial.Serial, n: int, deadline: float) -> bytes | None:
    buf = bytearray()
    while len(buf) < n:
        if time.monotonic() > deadline:
            return None
        chunk = port.read(n - len(buf))
        if chunk:
            buf.extend(chunk)
    return bytes(buf)


class CpsSession:
    def __init__(self, port_name: str, baud: int = 115200, reboot: bool = True):
        self.reboot = reboot
        self.port = serial.Serial(port_name, baudrate=baud, timeout=0.03)
        self._handshake()

    def __enter__(self) -> "CpsSession":
        return self

    def __exit__(self, exc_type, exc, tb) -> None:
        try:
            if exc_type is None and self.reboot:
                self.end()
        finally:
            self.port.close()

    def _handshake(self) -> None:
        self.port.timeout = HANDSHAKE_TIMEOUT_S
        for _attempt in range(HANDSHAKE_ATTEMPTS):
            self.port.reset_input_buffer()
            self.port.write(HANDSHAKE)
            ack = self.port.read(1)
            if ack == bytes([ACK]):
                self.port.timeout = 0.03
                return
        raise CpsError(
            f"no handshake ACK after {HANDSHAKE_ATTEMPTS} attempts -- wrong "
            "port/baud, cable, or the radio isn't running bf5rh-fw"
        )

    def _recv(self, deadline: float):
        b = _read_exact(self.port, 1, deadline)
        while b is not None and b[0] != HEADER:
            b = _read_exact(self.port, 1, deadline)
        if b is None:
            return None
        head = _read_exact(self.port, 5, deadline)  # cmd(1) + addr(2) + len(2)
        if head is None:
            return None
        cmd, addr, length = struct.unpack(">BHH", head)
        data = _read_exact(self.port, length, deadline) if length else b""
        if data is None:
            return None
        tail = _read_exact(self.port, 2, deadline)
        if tail is None:
            return None
        expected = crc16_xmodem(head + data)
        got = struct.unpack(">H", tail)[0]
        if got != expected:
            return None
        return cmd, addr, data

    def send(
        self, cmd: int, addr: int, data: bytes = b"", timeout: float = FRAME_TIMEOUT_S
    ) -> tuple[int, int, bytes]:
        frame = encode_frame(cmd, addr, data)
        for _attempt in range(MAX_RETRIES):
            self.port.reset_input_buffer()
            self.port.write(frame)
            resp = self._recv(time.monotonic() + timeout)
            if resp is None:
                continue
            resp_cmd, resp_addr, resp_data = resp
            if resp_cmd == CMD_ERROR:
                code = resp_data[0] if resp_data else 0xFF
                name = ERR_NAMES.get(code, f"unknown 0x{code:02X}")
                raise CpsError(f"cmd=0x{cmd:02X} addr=0x{addr:04X} denied: {name}")
            return resp
        raise CpsError(f"no response for cmd=0x{cmd:02X} after {MAX_RETRIES} retries")

    # high-level operations

    def push_app(self, slot: int, payload: bytes, on_progress=None) -> None:
        if not 0 <= slot < OVERLAY_SLOT_COUNT:
            raise CpsError(f"slot {slot} out of range 0..{OVERLAY_SLOT_COUNT - 1}")
        if len(payload) > OVERLAY_SLOT_SIZE:
            raise CpsError(f"app is {len(payload)}B, slot holds {OVERLAY_SLOT_SIZE}B")
        self.send(CMD_APP_ERASE, slot)
        padded = payload.ljust(-(-len(payload) // APP_CHUNK) * APP_CHUNK, b"\xff")
        for off in range(0, len(padded), APP_CHUNK):
            self.send(CMD_APP_WRITE, off, padded[off : off + APP_CHUNK])
            if on_progress:
                on_progress(off + APP_CHUNK, len(padded))

    def push_sat(self, table: bytes, on_progress=None) -> None:
        if len(table) != SAT_TABLE_SIZE:
            raise CpsError(
                f"satellite table is {len(table)}B, expected {SAT_TABLE_SIZE}B"
            )
        for off in range(0, len(table), SAT_CHUNK):
            timeout = ERASE_TIMEOUT_S if off == 0 else FRAME_TIMEOUT_S
            self.send(CMD_SAT_WRITE, off, table[off : off + SAT_CHUNK], timeout=timeout)
            if on_progress:
                on_progress(off + SAT_CHUNK, len(table))

    def push_logo(self, rgb565: bytes, on_progress=None) -> None:
        if len(rgb565) != BOOT_LOGO_SIZE:
            raise CpsError(f"logo is {len(rgb565)}B, expected {BOOT_LOGO_SIZE}B")
        for off in range(0, len(rgb565), BOOT_LOGO_CHUNK):
            timeout = ERASE_TIMEOUT_S if off == 0 else FRAME_TIMEOUT_S
            self.send(
                CMD_WRITE_LOGO,
                off,
                rgb565[off : off + BOOT_LOGO_CHUNK],
                timeout=timeout,
            )
            if on_progress:
                on_progress(off + BOOT_LOGO_CHUNK, len(rgb565))

    def end(self) -> None:
        """Explicit `CMD_END`: radio ACKs, drains its TX buffer, reboots."""
        self.send(CMD_END, 0)
        self.port.flush()
