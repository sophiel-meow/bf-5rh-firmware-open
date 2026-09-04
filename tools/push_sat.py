#!/usr/bin/env python3
import argparse
import subprocess
import sys
import tempfile
from pathlib import Path

from cps_proto import CpsError, CpsSession

TOOLS = Path(__file__).resolve().parent
MKRECORD = TOOLS / "mkrecord"


def host_triple() -> str:
    out = subprocess.run(
        ["rustc", "-vV"], check=True, capture_output=True, text=True
    ).stdout
    for line in out.splitlines():
        if line.startswith("host: "):
            return line[len("host: ") :].strip()
    raise RuntimeError("could not read the host triple from `rustc -vV`")


def build_table(sat_path: Path, out_path: Path) -> None:
    triple = host_triple()
    subprocess.run(
        [
            "cargo",
            "run",
            "--release",
            "--quiet",
            "--target",
            triple,
            "--",
            str(sat_path),
            str(out_path),
        ],
        cwd=MKRECORD,
        check=True,
    )


def push(sat_path: Path, port_name: str, baud: int, reboot: bool) -> None:
    with tempfile.TemporaryDirectory() as tmp:
        table_path = Path(tmp) / "satellites.bin"
        build_table(sat_path, table_path)
        table = table_path.read_bytes()
        print(f"{sat_path}: table built, {len(table)}B")

        with CpsSession(port_name, baud=baud, reboot=reboot) as cps:
            print("erasing satellite table sectors...")

            def progress(done: int, total: int) -> None:
                print(f"\r    {min(done, total)}/{total}", end="", flush=True)

            cps.push_sat(table, on_progress=progress)
            print()
            print("done, rebooting" if reboot else "done (not rebooting, --no-reboot)")


def main() -> int:
    ap = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    ap.add_argument("sat", type=Path, help="satellite list (.sat)")
    ap.add_argument("port", help="serial port eg. /dev/ttyUSB0 or COM3")
    ap.add_argument("--baud", type=int, default=115200)
    ap.add_argument(
        "--no-reboot",
        action="store_true",
        help="leave the session open instead of sending CMD_END (it will "
        "still idle-timeout and reboot ~2s after the last byte)",
    )
    ap.add_argument(
        "--build-only",
        type=Path,
        metavar="OUT.BIN",
        help="write the table to a file and skip the upload",
    )
    args = ap.parse_args()

    try:
        if args.build_only:
            build_table(args.sat, args.build_only)
            return 0
        push(args.sat, args.port, args.baud, reboot=not args.no_reboot)
    except subprocess.CalledProcessError:
        # mkrecord already printed what was wrong with the input
        return 1
    except (CpsError, OSError) as e:
        print(f"\n[ERR] {e}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
