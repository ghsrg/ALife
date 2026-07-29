#!/usr/bin/env python3
"""Create comparison artifacts for a reference and browser screenshot."""

from __future__ import annotations

import argparse
from pathlib import Path


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--reference", type=Path, required=True)
    parser.add_argument("--actual", type=Path, required=True)
    parser.add_argument("--output-dir", type=Path, default=Path("artifacts/reference-ui"))
    parser.add_argument("--alpha", type=float, default=0.5)
    args = parser.parse_args()

    try:
        from PIL import Image, ImageChops, ImageEnhance
    except ImportError:
        parser.error("Pillow is required. Install it only with user approval: python -m pip install Pillow")

    if not 0 <= args.alpha <= 1:
        parser.error("--alpha must be between 0 and 1")

    reference = Image.open(args.reference).convert("RGBA")
    actual = Image.open(args.actual).convert("RGBA")
    if reference.size != actual.size:
        parser.error(f"image sizes differ: reference={reference.size}, actual={actual.size}")

    output_dir = args.output_dir
    output_dir.mkdir(parents=True, exist_ok=True)

    overlay = Image.blend(reference, actual, args.alpha)
    overlay.save(output_dir / "overlay.png")

    diff = ImageChops.difference(reference, actual)
    diff.save(output_dir / "diff.png")
    ImageEnhance.Contrast(diff).enhance(4).save(output_dir / "diff-enhanced.png")

    side_by_side = Image.new("RGBA", (reference.width * 2, reference.height))
    side_by_side.paste(reference, (0, 0))
    side_by_side.paste(actual, (reference.width, 0))
    side_by_side.save(output_dir / "side-by-side.png")

    bbox = diff.getbbox()
    differing_pixels = 0
    total_pixels = reference.width * reference.height
    if bbox:
        rgb_diff = diff.convert("RGB")
        differing_pixels = sum(1 for pixel in rgb_diff.getdata() if pixel != (0, 0, 0))

    ratio = differing_pixels / total_pixels if total_pixels else 0
    report = (
        f"reference_size={reference.width}x{reference.height}\n"
        f"actual_size={actual.width}x{actual.height}\n"
        f"difference_bbox={bbox}\n"
        f"differing_pixels={differing_pixels}\n"
        f"differing_ratio={ratio:.6%}\n"
    )
    (output_dir / "report.txt").write_text(report, encoding="utf-8")
    print(report, end="")
    print(f"Artifacts written to {output_dir.resolve()}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
