#!/usr/bin/env python3
import argparse
import sys

from PIL import Image

from cps_proto import BOOT_LOGO_HEIGHT, BOOT_LOGO_WIDTH, CpsError, CpsSession

W, H = BOOT_LOGO_WIDTH, BOOT_LOGO_HEIGHT


def _resize_cover(img: Image.Image) -> Image.Image:
    src_ratio = img.width / img.height
    dst_ratio = W / H
    if src_ratio > dst_ratio:
        new_h = H
        new_w = round(H * src_ratio)
    else:
        new_w = W
        new_h = round(W / src_ratio)
    img = img.resize((new_w, new_h), Image.LANCZOS)
    left = (new_w - W) // 2
    top = (new_h - H) // 2
    return img.crop((left, top, left + W, top + H))


def _resize_contain(img: Image.Image) -> Image.Image:
    src_ratio = img.width / img.height
    dst_ratio = W / H
    if src_ratio > dst_ratio:
        new_w = W
        new_h = round(W / src_ratio)
    else:
        new_h = H
        new_w = round(H * src_ratio)
    img = img.resize((new_w, new_h), Image.LANCZOS)
    canvas = Image.new("RGB", (W, H), (0, 0, 0))
    canvas.paste(img, ((W - new_w) // 2, (H - new_h) // 2))
    return canvas


def load_and_fit(path: str, fit: str) -> Image.Image:
    img = Image.open(path).convert("RGB")
    if fit == "stretch":
        return img.resize((W, H), Image.LANCZOS)
    if fit == "contain":
        return _resize_contain(img)
    return _resize_cover(img)


def to_rgb565(img: Image.Image) -> bytes:
    out = bytearray(W * H * 2)
    px = img.load()
    i = 0
    for y in range(H):
        for x in range(W):
            r, g, b = px[x, y]
            v = ((r >> 3) << 11) | ((g >> 2) << 5) | (b >> 3)
            out[i] = v & 0xFF
            out[i + 1] = (v >> 8) & 0xFF
            i += 2
    return bytes(out)


def from_rgb565(data: bytes) -> Image.Image:
    img = Image.new("RGB", (W, H))
    px = img.load()
    i = 0
    for y in range(H):
        for x in range(W):
            v = data[i] | (data[i + 1] << 8)
            r5, g6, b5 = (v >> 11) & 0x1F, (v >> 5) & 0x3F, v & 0x1F
            px[x, y] = (r5 * 255 // 31, g6 * 255 // 63, b5 * 255 // 31)
            i += 2
    return img


def main() -> int:
    ap = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    ap.add_argument("image", help="any Pillow-readable image")
    ap.add_argument("port", help="serial port eg. /dev/ttyUSB0 or COM3")
    ap.add_argument("--baud", type=int, default=115200)
    ap.add_argument(
        "--fit",
        choices=["cover", "contain", "stretch"],
        default="cover",
        help="cover: crop to fill 160x128 (default); contain: letterbox; "
        "stretch: ignore aspect ratio",
    )
    ap.add_argument(
        "--preview", metavar="OUT.PNG", help="save what will be written, upscaled"
    )
    ap.add_argument(
        "--no-push", action="store_true", help="only write --preview, skip the radio"
    )
    ap.add_argument(
        "--no-reboot",
        action="store_true",
        help="leave the session open instead of sending CMD_END (it will "
        "still idle-timeout and reboot ~2s after the last byte)",
    )
    args = ap.parse_args()

    fitted = load_and_fit(args.image, args.fit)
    rgb565 = to_rgb565(fitted)

    if args.preview:
        from_rgb565(rgb565).resize((W * 3, H * 3), Image.NEAREST).save(args.preview)
        print(f"wrote {args.preview}")

    if args.no_push:
        return 0

    try:
        with CpsSession(args.port, baud=args.baud, reboot=not args.no_reboot) as cps:
            print("erasing logo sectors (can take several seconds)...")

            def progress(done: int, total: int) -> None:
                print(f"\r    {min(done, total)}/{total}", end="", flush=True)

            cps.push_logo(rgb565, on_progress=progress)
            print()
            print(
                "done, rebooting"
                if not args.no_reboot
                else "done (not rebooting, --no-reboot)"
            )
    except (CpsError, OSError) as e:
        print(f"\n[ERR] {e}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
