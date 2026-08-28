import argparse
import re
import struct
import subprocess
import sys
import zlib
from pathlib import Path

import mkapp

HEADER_FMT = mkapp.HEADER_FMT
HEADER_SIZE = mkapp.HEADER_SIZE


def run(cmd, **kw):
    return subprocess.run(cmd, check=True, capture_output=True, text=True, **kw)


def arena_size(fw_root: Path) -> int:
    lib_rs = (fw_root / "abi" / "src" / "lib.rs").read_text()
    m = re.search(r"pub const ARENA_SIZE:\s*usize\s*=\s*(\d+)", lib_rs)
    if not m:
        sys.exit("[ERR] missing ARENA_SIZE")
    return int(m.group(1))


def real_arena_address(fw_root: Path) -> int:
    memory_x = fw_root / "memory.x"
    original = memory_x.read_text()
    widened = re.sub(r"LENGTH\s*=\s*60K", "LENGTH = 64K", original)
    if widened == original:
        sys.exit("[ERR] missing `LENGTH = 60K`")
    memory_x.write_text(widened)
    try:
        subprocess.run(["cargo", "build", "--release"], cwd=fw_root, check=True)
        elf = fw_root / "target" / "thumbv7em-none-eabihf" / "release" / "bf5rh-fw"
        out = run(["rust-nm", str(elf)]).stdout
        for line in out.splitlines():
            if line.strip().endswith("overlay5ARENA") or "7overlay5ARENA" in line:
                return int(line.split()[0], 16)
        sys.exit("[ERR] missing overlay::ARENA")
    finally:
        memory_x.write_text(original)


def check_relocations(elf: Path) -> tuple[bool, str]:
    out = run(["rust-objdump", "-r", str(elf)]).stdout
    if "RELOCATION RECORDS" in out:
        return False, "residual relocation records"
    return True, "relocation ok"


def check_origin(app_dir: Path, fw_root: Path) -> tuple[bool, str]:
    declared = mkapp.arena_origin(app_dir)
    actual = real_arena_address(fw_root)
    if declared != actual:
        return (
            False,
            f"app.x wrong: ORIGIN=0x{declared:X}, should be: ARENA=0x{actual:X}",
        )
    return True, f"ORIGIN=0x{declared:X} correct"


def check_size(header: dict, fw_root: Path) -> tuple[bool, str]:
    limit = arena_size(fw_root)
    total = header["image_len"] + header["bss_len"]
    if total > limit:
        return False, f"image_len+bss_len={total}B over ARENA_SIZE={limit}B"
    return True, f"{total}B / {limit}B"


def check_build_hash(header: dict, fw_root: Path) -> tuple[bool, str]:
    expected = mkapp.build_hash(fw_root)
    if header["build_hash"] != expected:
        return False, (
            f"app header hash 0x{header['build_hash']:08X}"
            f"should be: 0x{expected:08X}"
        )
    return True, f"0x{expected:08X}"


def parse_header(app_path: Path) -> dict:
    data = app_path.read_bytes()[:HEADER_SIZE]
    magic, build_hash, image_len, bss_len, entry_off, crc32, name = struct.unpack(
        HEADER_FMT, data
    )
    return {
        "magic": magic,
        "build_hash": build_hash,
        "image_len": image_len,
        "bss_len": bss_len,
        "entry_off": entry_off,
        "crc32": crc32,
        "name": name.rstrip(b"\0").decode("ascii", "replace"),
    }


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument(
        "target",
        type=Path,
        help="app crate dir or .app file",
    )
    ap.add_argument(
        "--skip-origin",
        action="store_true",
        help="skip origin check",
    )
    ap.add_argument("--name", help="for mkapp.py")
    args = ap.parse_args()

    target = args.target.resolve()
    if target.is_dir():
        app_dir = target
        app_path = app_dir.with_suffix(".app")
        mkapp.pack(app_dir, app_path, args.name)
    else:
        app_path = target
        app_dir = None

    fw_root = mkapp.find_repo_root(app_dir) if app_dir else target.parent.parent
    header = parse_header(app_path)
    print(f"[..] name={header['name']!r}")

    results = []
    if header["magic"] != mkapp.MAGIC:
        print(f"[FAIL] wrong magic: {header['magic']!r}")
        return 1

    results.append(("build_hash ok", check_build_hash(header, fw_root)))
    results.append(("size ok", check_size(header, fw_root)))

    if app_dir is not None:
        bin_name = app_dir.name
        elf = app_dir / "target" / "thumbv7em-none-eabihf" / "release" / bin_name
        results.append(("relocations ok", check_relocations(elf)))
        if not args.skip_origin:
            results.append(("ORIGIN ok", check_origin(app_dir, fw_root)))

    ok = True
    for name, (passed, detail) in results:
        mark = "OK  " if passed else "FAIL"
        print(f"[{mark}] {name}: {detail}")
        ok = ok and passed

    return 0 if ok else 1


if __name__ == "__main__":
    raise SystemExit(main())
