#!/usr/bin/env python3
import argparse
import sys
import tempfile
from pathlib import Path

from cps_proto import CpsError, CpsSession
from push_app import parse_pair
from push_boot_logo import load_and_fit, to_rgb565
from push_sat import build_table

TOOLS = Path(__file__).resolve().parent
APPS_DIR = TOOLS.parent / "apps"

DEFAULT_APPS: list[tuple[int, Path]] = [
    (0, APPS_DIR / "hello.app"),
    (1, APPS_DIR / "fm.app"),
    (2, APPS_DIR / "chanmgr.app"),
    (3, APPS_DIR / "satellite.app"),
    (4, APPS_DIR / "scanqt.app"),
    (5, APPS_DIR / "search.app"),
    (6, APPS_DIR / "settings.app"),
    (7, APPS_DIR / "spectrum.app"),
]


def _progress(done: int, total: int) -> None:
    print(f"\r    {min(done, total)}/{total}", end="", flush=True)


def main() -> int:
    ap = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    ap.add_argument("port", help="serial port eg. /dev/ttyUSB0 or COM3")
    ap.add_argument("--baud", type=int, default=115200)
    ap.add_argument(
        "--app",
        action="append",
        type=parse_pair,
        metavar="SLOT:FILE",
        dest="extra_apps",
        default=[],
        help="add/override one app slot; repeatable",
    )
    ap.add_argument(
        "--no-default-apps",
        action="store_true",
        help="skip the built-in 7-app set, install only --app entries",
    )
    ap.add_argument(
        "--sat", type=Path, metavar="SATS.SAT", help="satellite list to build+push"
    )
    ap.add_argument(
        "--logo", metavar="IMAGE", help="image to convert and push as the boot logo"
    )
    ap.add_argument(
        "--logo-fit", choices=["cover", "contain", "stretch"], default="cover"
    )
    args = ap.parse_args()

    apps: dict[int, Path] = {}
    if not args.no_default_apps:
        for slot, path in DEFAULT_APPS:
            if path.exists():
                apps[slot] = path
            else:
                print(f"[skip] {path} not built", file=sys.stderr)
    for slot, path in args.extra_apps:
        apps[slot] = path

    if not apps and not args.sat and not args.logo:
        print("[ERR] nothing to install", file=sys.stderr)
        return 1

    try:
        with CpsSession(args.port, baud=args.baud, reboot=True) as cps:
            for slot in sorted(apps):
                payload = apps[slot].read_bytes()
                print(f"slot {slot}: {apps[slot]} ({len(payload)}B)")
                cps.push_app(slot, payload, on_progress=_progress)
                print()

            if args.sat:
                with tempfile.TemporaryDirectory() as tmp:
                    table_path = Path(tmp) / "satellites.bin"
                    build_table(args.sat, table_path)
                    table = table_path.read_bytes()
                    print(f"satellites: {args.sat} ({len(table)}B)")
                    cps.push_sat(table, on_progress=_progress)
                    print()

            if args.logo:
                fitted = load_and_fit(args.logo, args.logo_fit)
                rgb565 = to_rgb565(fitted)
                print(f"boot logo: {args.logo}")
                cps.push_logo(rgb565, on_progress=_progress)
                print()

            print("done, rebooting")
    except (CpsError, OSError) as e:
        print(f"\n[ERR] {e}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
