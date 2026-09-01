import argparse
import re
import subprocess
import sys
import zlib
from pathlib import Path

import mkapp


def run(cmd, **kw):
    return subprocess.run(cmd, check=True, capture_output=True, text=True, **kw)


def real_arena_address(fw_root: Path) -> int:
    memory_x = fw_root / "memory.x"
    original = memory_x.read_text()
    widened = re.sub(r"LENGTH\s*=\s*60K", "LENGTH = 64K", original)
    if widened == original:
        sys.exit("[ERR] missing `LENGTH = 60K`")
    memory_x.write_text(widened)
    try:
        subprocess.run(["cargo", "build", "--release"], cwd=fw_root, check=True)
        elf = fw_root / "target" / "thumbv7em-none-eabi" / "release" / "bf5rh-fw"
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


def check_origin(app_dirs: list[Path], fw_root: Path) -> tuple[bool, str]:
    actual = real_arena_address(fw_root)
    bad = [d for d in app_dirs if mkapp.arena_origin(d) != actual]
    if bad:
        names = ", ".join(f"{d.name}=0x{mkapp.arena_origin(d):X}" for d in bad)
        return False, f"app.x wrong: {names}, should be ARENA=0x{actual:X}"
    return True, f"ORIGIN=0x{actual:X} correct"


def check_size(header: dict, fw_root: Path) -> tuple[bool, str]:
    limit = mkapp.arena_size(fw_root)
    limit_max = mkapp.arena_max(fw_root)
    slot = mkapp.slot_size(fw_root)
    parts = []
    ok = header["size"] <= slot
    for i, seg in enumerate(header["segments"]):
        total = seg["image_len"] + seg["bss_len"]
        ok = ok and total <= limit_max
        tag = " oversized" if total > limit else ""
        parts.append(f"seg{i}={total}B{tag}")
    return ok, (
        f"{', '.join(parts)} (each / {limit}B, oversized / {limit_max}B), "
        f"package={header['size']}B / {slot}B"
    )


def check_crc(header: dict) -> tuple[bool, str]:
    data = header["data"]
    for i, seg in enumerate(header["segments"]):
        start = seg["offset"]
        end = start + seg["image_len"]
        if end > len(data):
            return False, f"seg{i} runs past end of file"
        crc = zlib.crc32(data[start:end]) & 0xFFFFFFFF
        if crc != seg["crc32"]:
            return False, f"seg{i} crc 0x{crc:08X} != header 0x{seg['crc32']:08X}"
    return True, f"{len(header['segments'])} segment(s) ok"


def check_build_hash(header: dict, fw_root: Path) -> tuple[bool, str]:
    expected = mkapp.build_hash(fw_root)
    if header["build_hash"] != expected:
        return False, (
            f"app header hash 0x{header['build_hash']:08X} "
            f"should be: 0x{expected:08X}"
        )
    return True, f"0x{expected:08X}"


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument(
        "targets",
        type=Path,
        nargs="+",
        help="app crate dirs (repacked first) or one .app file",
    )
    ap.add_argument("-o", "--out", type=Path, help=".app path, when packing")
    ap.add_argument("--skip-origin", action="store_true", help="skip origin check")
    ap.add_argument("--name", help="for mkapp.py")
    args = ap.parse_args()

    targets = [t.resolve() for t in args.targets]
    if targets[0].is_dir():
        app_dirs = targets
        app_path = args.out.resolve() if args.out else mkapp.default_out(app_dirs)
        mkapp.pack(app_dirs, app_path, args.name)
    else:
        if len(targets) != 1:
            sys.exit("[ERR] pass exactly one .app file")
        app_dirs = []
        app_path = targets[0]

    fw_root = mkapp.find_repo_root(app_dirs[0]) if app_dirs else app_path.parent.parent
    header = mkapp.parse_header(app_path)
    print(f"[..] name={header['name']!r} segments={header['segment_count']}")

    if header["magic"] != mkapp.MAGIC:
        print(f"[FAIL] wrong magic: {header['magic']!r}")
        return 1
    if not 1 <= header["segment_count"] <= mkapp.MAX_SEGMENTS:
        print(f"[FAIL] segment_count={header['segment_count']}")
        return 1

    results = [
        ("build_hash ok", check_build_hash(header, fw_root)),
        ("size ok", check_size(header, fw_root)),
        ("crc ok", check_crc(header)),
    ]

    if app_dirs:
        for d in app_dirs:
            results.append(
                (f"{d.name} relocations ok", check_relocations(mkapp.elf_path(d)))
            )
        if not args.skip_origin:
            results.append(("ORIGIN ok", check_origin(app_dirs, fw_root)))

    ok = True
    for name, (passed, detail) in results:
        print(f"[{'OK  ' if passed else 'FAIL'}] {name}: {detail}")
        ok = ok and passed

    return 0 if ok else 1


if __name__ == "__main__":
    raise SystemExit(main())
