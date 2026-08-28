import argparse
import os
import re
import struct
import subprocess
import sys
import zlib
from pathlib import Path

MAGIC = b"OVL1"
NAME_LEN = 16
HEADER_FMT = f"<4sIIIII{NAME_LEN}s"  # magic, build_hash, image_len, bss_len, entry_off, crc32, name
HEADER_SIZE = struct.calcsize(HEADER_FMT)
assert HEADER_SIZE == 40

APP_RUSTFLAGS = "-C link-arg=-Tapp.x -C link-arg=--nmagic"


def run(cmd, **kw):
    return subprocess.run(cmd, check=True, capture_output=True, text=True, **kw)


def find_repo_root(app_dir: Path) -> Path:
    fw_root = app_dir.parent.parent
    if not (fw_root / "abi" / "src" / "lib.rs").exists():
        sys.exit(f"[ERR] missing abi/src/lib.rs")
    return fw_root


def build_hash(fw_root: Path) -> int:
    lib_rs = (fw_root / "abi" / "src" / "lib.rs").read_bytes()
    return zlib.crc32(lib_rs) & 0xFFFFFFFF


def arena_origin(app_dir: Path) -> int:
    app_x = (app_dir / "app.x").read_text()
    m = re.search(r"ORIGIN\s*=\s*(0x[0-9A-Fa-f]+)", app_x)
    if not m:
        sys.exit(f"[ERR] missing ORIGIN in {app_dir/'app.x'}")
    return int(m.group(1), 16)


def cargo_build(app_dir: Path, bin_name: str) -> Path:
    print(f"[..] cargo build --release (RUSTFLAGS={APP_RUSTFLAGS!r})")
    full_env = {**os.environ, "RUSTFLAGS": APP_RUSTFLAGS}
    subprocess.run(
        ["cargo", "build", "--release"], cwd=app_dir, env=full_env, check=True
    )
    elf = app_dir / "target" / "thumbv7em-none-eabihf" / "release" / bin_name
    if not elf.exists():
        sys.exit(f"[ERR] No elf generated")
    return elf


def extract_image(elf: Path, out_bin: Path) -> None:
    run(["rust-objcopy", "-O", "binary", str(elf), str(out_bin)])


def entry_address(elf: Path) -> int:
    out = run(["rust-nm", str(elf)]).stdout
    for line in out.splitlines():
        parts = line.split()
        if len(parts) >= 3 and parts[-1] == "app_entry":
            return int(parts[0], 16)
    sys.exit(f"[ERR] {elf} missing app_entry")


def bss_size(elf: Path) -> int:
    out = run(["rust-size", str(elf)]).stdout
    #    text    data     bss     dec     hex filename
    #     189       0       8     197      c5 .../hello
    lines = [l for l in out.splitlines() if l.strip()]
    cols = lines[-1].split()
    return int(cols[2])


def pack(app_dir: Path, out_path: Path, name: str | None = None) -> None:
    fw_root = find_repo_root(app_dir)
    bin_name = app_dir.name
    display_name = (name or bin_name).upper()
    name_bytes = display_name.encode("ascii", "replace")[:NAME_LEN]
    if len(name_bytes) < len(display_name.encode("ascii", "replace")):
        print(f"[WARN] app name {display_name!r} trunked at max {NAME_LEN} bytes")
    name_field = name_bytes.ljust(NAME_LEN, b"\0")

    elf = cargo_build(app_dir, bin_name)

    image_bin = out_path.with_suffix(".image.bin")
    extract_image(elf, image_bin)
    image_bytes = image_bin.read_bytes()
    image_bin.unlink()

    entry = entry_address(elf)
    bss_len = bss_size(elf)
    origin = arena_origin(app_dir)
    entry_off = entry - origin
    if entry_off < 0:
        sys.exit(f"[ERR] entry 0x{entry:X} smaller than ORIGIN 0x{origin:X}")

    hdr_build_hash = build_hash(fw_root)
    img_crc32 = zlib.crc32(image_bytes) & 0xFFFFFFFF

    header = struct.pack(
        HEADER_FMT,
        MAGIC,
        hdr_build_hash,
        len(image_bytes),
        bss_len,
        entry_off,
        img_crc32,
        name_field,
    )
    out_path.write_bytes(header + image_bytes)

    total = len(image_bytes) + bss_len
    print(
        f"[OK] {out_path}  name={display_name!r}  image={len(image_bytes)}B  bss={bss_len}B  "
        f"total={total}B  entry_off=0x{entry_off:X}  build_hash=0x{hdr_build_hash:08X}"
    )


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("app_dir", type=Path, help="app crate dir")
    ap.add_argument("-o", "--out", type=Path, help=".app path (default <app_dir>.app)")
    ap.add_argument(
        "--name",
        help=f"app name",
    )
    args = ap.parse_args()

    app_dir = args.app_dir.resolve()
    out_path = args.out or app_dir.with_suffix(".app")
    pack(app_dir, out_path, args.name)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
