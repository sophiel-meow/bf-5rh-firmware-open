#!/usr/bin/env python3
import argparse
import sys
from pathlib import Path

from cps_proto import CpsError, CpsSession, OVERLAY_SLOT_COUNT


def parse_pair(spec: str) -> tuple[int, Path]:
    slot_str, _, path_str = spec.partition(":")
    if not path_str:
        raise argparse.ArgumentTypeError(f"expected SLOT:FILE, got {spec!r}")
    try:
        slot = int(slot_str)
    except ValueError:
        raise argparse.ArgumentTypeError(f"bad slot number in {spec!r}")
    if not 0 <= slot < OVERLAY_SLOT_COUNT:
        raise argparse.ArgumentTypeError(
            f"slot {slot} out of range 0..{OVERLAY_SLOT_COUNT - 1}"
        )
    return slot, Path(path_str)


def push_all(
    port_name: str, baud: int, apps: list[tuple[int, Path]], reboot: bool
) -> None:
    with CpsSession(port_name, baud=baud, reboot=reboot) as cps:
        for slot, path in apps:
            payload = path.read_bytes()
            print(f"slot {slot}: {path} ({len(payload)}B)")

            def progress(done: int, total: int) -> None:
                print(f"\r    {min(done, total)}/{total}", end="", flush=True)

            cps.push_app(slot, payload, on_progress=progress)
            print()
        if reboot:
            print("done, rebooting")
        else:
            print("done (not rebooting, --no-reboot)")


def main() -> int:
    ap = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    ap.add_argument("port", help="serial port eg. /dev/ttyUSB0 or COM3")
    ap.add_argument(
        "apps",
        nargs="+",
        type=parse_pair,
        metavar="SLOT:FILE",
        help=".app file per slot",
    )
    ap.add_argument("--baud", type=int, default=115200)
    ap.add_argument(
        "--no-reboot",
        action="store_true",
        help="leave the session open instead of sending CMD_END (it will "
        "still idle-timeout and reboot ~2s after the last byte)",
    )
    args = ap.parse_args()

    slots = [slot for slot, _ in args.apps]
    if len(slots) != len(set(slots)):
        print("[ERR] duplicate slot in the SLOT:FILE list", file=sys.stderr)
        return 1

    try:
        push_all(args.port, args.baud, args.apps, reboot=not args.no_reboot)
    except (CpsError, OSError) as e:
        print(f"\n[ERR] {e}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
