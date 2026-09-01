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
MAX_SEGMENTS = 4

PKG_FMT = f"<4sIB3x{NAME_LEN}s"
# offset, image_len, bss_len, entry_off, crc32
SEG_FMT = "<IIIII"
SEG_SIZE = struct.calcsize(SEG_FMT)
HEADER_SIZE = struct.calcsize(PKG_FMT) + MAX_SEGMENTS * SEG_SIZE
assert SEG_SIZE == 20 and HEADER_SIZE == 108

SEG_ALIGN = 4

APP_RUSTFLAGS = "-C link-arg=-Tapp.x -C link-arg=--nmagic"


def run(cmd, **kw):
    return subprocess.run(cmd, check=True, capture_output=True, text=True, **kw)


def find_repo_root(app_dir: Path) -> Path:
    for cand in app_dir.parents:
        if (cand / "abi" / "src" / "lib.rs").exists():
            return cand
    sys.exit("[ERR] missing abi/src/lib.rs above {}".format(app_dir))


def build_hash(fw_root: Path) -> int:
    lib_rs = (fw_root / "abi" / "src" / "lib.rs").read_bytes()
    return zlib.crc32(lib_rs) & 0xFFFFFFFF


def _const(fw_root: Path, name: str) -> int:
    lib_rs = (fw_root / "abi" / "src" / "lib.rs").read_text()
    m = re.search(rf"pub const {name}:\s*usize\s*=\s*(\d+)", lib_rs)
    if not m:
        sys.exit(f"[ERR] missing {name} in abi/src/lib.rs")
    return int(m.group(1))


def arena_size(fw_root: Path) -> int:
    return _const(fw_root, "ARENA_SIZE")


def arena_max(fw_root: Path) -> int:
    return _const(fw_root, "ARENA_MAX")


def slot_size(fw_root: Path) -> int:
    flash_map = (fw_root / "src" / "flash_map.rs").read_text()
    m = re.search(r"OVERLAY_SLOT_SIZE:\s*u32\s*=\s*(\d+)\s*\*\s*1024", flash_map)
    if not m:
        sys.exit("[ERR] missing OVERLAY_SLOT_SIZE in src/flash_map.rs")
    return int(m.group(1)) * 1024


def arena_origin(app_dir: Path) -> int:
    app_x = (app_dir / "app.x").read_text()
    m = re.search(r"ORIGIN\s*=\s*(0x[0-9A-Fa-f]+)", app_x)
    if not m:
        sys.exit(f"[ERR] missing ORIGIN in {app_dir/'app.x'}")
    return int(m.group(1), 16)


def touch_if_stale_linker_script(app_dir: Path, elf: Path) -> None:
    import shutil

    app_x = app_dir / "app.x"
    if not (app_x.exists() and elf.exists()):
        return
    if app_x.stat().st_mtime <= elf.stat().st_mtime:
        return
    print(f"[..] {app_dir.name}: app.x changed, clean relink")
    shutil.rmtree(app_dir / "target", ignore_errors=True)


def check_is_crate(app_dir: Path) -> None:
    if (app_dir / "Cargo.toml").exists():
        return
    if not app_dir.is_dir():
        sys.exit(f"[ERR] {app_dir} does not exist")
    inner = sorted(d.name for d in app_dir.iterdir() if (d / "Cargo.toml").exists())
    hint = (
        f"\n       it holds {len(inner)} segment crate(s); name them in order:"
        f"\n       {' '.join(str(app_dir / d) for d in inner)}"
        if inner
        else ""
    )
    sys.exit(f"[ERR] {app_dir} is not a crate: no Cargo.toml{hint}")


def cargo_build(app_dir: Path, bin_name: str) -> Path:
    print(f"[..] {app_dir.name}: cargo build --release")
    touch_if_stale_linker_script(app_dir, elf_path(app_dir))
    full_env = {**os.environ, "RUSTFLAGS": APP_RUSTFLAGS}
    subprocess.run(
        ["cargo", "build", "--release"], cwd=app_dir, env=full_env, check=True
    )
    elf = app_dir / "target" / "thumbv7em-none-eabi" / "release" / bin_name
    if not elf.exists():
        sys.exit(f"[ERR] No elf generated: {elf}")
    return elf


def elf_path(app_dir: Path) -> Path:
    return app_dir / "target" / "thumbv7em-none-eabi" / "release" / app_dir.name


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
    lines = [l for l in out.splitlines() if l.strip()]
    return int(lines[-1].split()[2])


def build_segment(app_dir: Path, out_dir: Path) -> dict:
    """Build one crate and return its image bytes + segment fields."""
    elf = cargo_build(app_dir, app_dir.name)

    image_bin = out_dir / f".{app_dir.name}.image.bin"
    extract_image(elf, image_bin)
    image_bytes = image_bin.read_bytes()
    image_bin.unlink()

    entry = entry_address(elf)
    origin = arena_origin(app_dir)
    entry_off = entry - origin
    if entry_off < 0:
        sys.exit(f"[ERR] entry 0x{entry:X} smaller than ORIGIN 0x{origin:X}")

    return {
        "dir": app_dir,
        "image": image_bytes,
        "image_len": len(image_bytes),
        "bss_len": bss_size(elf),
        "entry_off": entry_off,
        "crc32": zlib.crc32(image_bytes) & 0xFFFFFFFF,
    }


def default_out(app_dirs: list[Path]) -> Path:
    if len(app_dirs) == 1:
        return app_dirs[0].with_suffix(".app")
    parent = app_dirs[0].parent
    if any(d.parent != parent for d in app_dirs):
        sys.exit("[ERR] segments in different dirs, pass -o explicitly")
    return parent.with_suffix(".app")


def default_name(app_dirs: list[Path]) -> str:
    return (app_dirs[0] if len(app_dirs) == 1 else app_dirs[0].parent).name


def pack(app_dirs: list[Path], out_path: Path, name: str | None = None) -> None:
    if not 1 <= len(app_dirs) <= MAX_SEGMENTS:
        sys.exit(f"[ERR] need 1..{MAX_SEGMENTS} segments, got {len(app_dirs)}")

    fw_root = find_repo_root(app_dirs[0])
    limit_arena = arena_size(fw_root)
    limit_max = arena_max(fw_root)
    limit_slot = slot_size(fw_root)

    display_name = (name or default_name(app_dirs)).upper()
    name_bytes = display_name.encode("ascii", "replace")
    if len(name_bytes) > NAME_LEN:
        print(f"[WARN] app name {display_name!r} trunked at max {NAME_LEN} bytes")
    name_field = name_bytes[:NAME_LEN].ljust(NAME_LEN, b"\0")

    for d in app_dirs:
        check_is_crate(d)

    segments = [build_segment(d, out_path.parent) for d in app_dirs]

    body = bytearray()
    entries = []
    for seg in segments:
        pad = (-len(body)) % SEG_ALIGN
        body.extend(b"\0" * pad)
        offset = HEADER_SIZE + len(body)
        body.extend(seg["image"])
        entries.append(
            struct.pack(
                SEG_FMT,
                offset,
                seg["image_len"],
                seg["bss_len"],
                seg["entry_off"],
                seg["crc32"],
            )
        )

    header = struct.pack(PKG_FMT, MAGIC, build_hash(fw_root), len(segments), name_field)
    header += b"".join(entries)
    header += b"\0" * SEG_SIZE * (MAX_SEGMENTS - len(segments))
    assert len(header) == HEADER_SIZE

    package = header + bytes(body)

    failed = False
    for i, seg in enumerate(segments):
        total = seg["image_len"] + seg["bss_len"]
        limit = limit_arena
        mark = "OK  "
        if total > limit_max:
            mark, failed, limit = "OVER", True, limit_max
        elif total > limit_arena:
            mark, limit = "BIG ", limit_max
        print(
            f"[{mark}] seg{i} {seg['dir'].name}: image={seg['image_len']}B "
            f"bss={seg['bss_len']}B total={total}B / {limit}B "
            f"entry_off=0x{seg['entry_off']:X}"
        )
    if len(package) > limit_slot:
        print(f"[OVER] package {len(package)}B over slot {limit_slot}B")
        failed = True
    if failed:
        sys.exit(1)

    out_path.write_bytes(package)
    print(
        f"[OK] {out_path}  name={display_name!r}  segments={len(segments)}  "
        f"package={len(package)}B / {limit_slot}B  "
        f"build_hash=0x{build_hash(fw_root):08X}"
    )


def parse_header(app_path: Path) -> dict:
    data = app_path.read_bytes()
    if len(data) < HEADER_SIZE:
        sys.exit(f"[ERR] {app_path} shorter than a package header")
    magic, hash_, count, name = struct.unpack(PKG_FMT, data[: struct.calcsize(PKG_FMT)])
    segs = []
    for i in range(MAX_SEGMENTS):
        off = struct.calcsize(PKG_FMT) + i * SEG_SIZE
        offset, image_len, bss_len, entry_off, crc32 = struct.unpack(
            SEG_FMT, data[off : off + SEG_SIZE]
        )
        segs.append(
            {
                "offset": offset,
                "image_len": image_len,
                "bss_len": bss_len,
                "entry_off": entry_off,
                "crc32": crc32,
            }
        )
    return {
        "magic": magic,
        "build_hash": hash_,
        "segment_count": count,
        "name": name.rstrip(b"\0").decode("ascii", "replace"),
        "segments": segs[:count],
        "size": len(data),
        "data": data,
    }


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument(
        "app_dirs",
        type=Path,
        nargs="+",
        help=f"app crate dirs, in segment order (max {MAX_SEGMENTS})",
    )
    ap.add_argument("-o", "--out", type=Path, help=".app path")
    ap.add_argument("--name", help="app name shown in the launcher")
    args = ap.parse_args()

    app_dirs = [d.resolve() for d in args.app_dirs]
    out_path = args.out.resolve() if args.out else default_out(app_dirs)
    pack(app_dirs, out_path, args.name)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
