#!/usr/bin/env python3
"""Inventory common image assets without third-party dependencies."""

from __future__ import annotations

import argparse
import csv
import re
import struct
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable


@dataclass
class ImageInfo:
    path: Path
    format: str
    width: int | None
    height: int | None
    transparency: str
    note: str = ""


def png_info(path: Path) -> ImageInfo:
    with path.open("rb") as f:
        signature = f.read(8)
        if signature != b"\x89PNG\r\n\x1a\n":
            raise ValueError("invalid PNG signature")
        length = struct.unpack(">I", f.read(4))[0]
        chunk = f.read(4)
        if chunk != b"IHDR" or length < 13:
            raise ValueError("missing PNG IHDR")
        data = f.read(13)
        width, height, bit_depth, color_type = struct.unpack(">IIBB", data[:10])
        alpha = color_type in (4, 6)
        return ImageInfo(path, "PNG", width, height, "yes" if alpha else "unknown/no")


def gif_info(path: Path) -> ImageInfo:
    with path.open("rb") as f:
        header = f.read(10)
    if header[:6] not in (b"GIF87a", b"GIF89a"):
        raise ValueError("invalid GIF signature")
    width, height = struct.unpack("<HH", header[6:10])
    return ImageInfo(path, "GIF", width, height, "possible")


def bmp_info(path: Path) -> ImageInfo:
    with path.open("rb") as f:
        header = f.read(30)
    if header[:2] != b"BM":
        raise ValueError("invalid BMP signature")
    width, height = struct.unpack("<ii", header[18:26])
    return ImageInfo(path, "BMP", abs(width), abs(height), "possible")


def jpeg_info(path: Path) -> ImageInfo:
    with path.open("rb") as f:
        if f.read(2) != b"\xff\xd8":
            raise ValueError("invalid JPEG signature")
        while True:
            marker_start = f.read(1)
            if not marker_start:
                break
            if marker_start != b"\xff":
                continue
            marker = f.read(1)
            while marker == b"\xff":
                marker = f.read(1)
            if marker in (b"\xd8", b"\xd9"):
                continue
            length_bytes = f.read(2)
            if len(length_bytes) != 2:
                break
            length = struct.unpack(">H", length_bytes)[0]
            if marker and marker[0] in {
                0xC0, 0xC1, 0xC2, 0xC3, 0xC5, 0xC6, 0xC7,
                0xC9, 0xCA, 0xCB, 0xCD, 0xCE, 0xCF,
            }:
                data = f.read(5)
                if len(data) != 5:
                    break
                height, width = struct.unpack(">HH", data[1:5])
                return ImageInfo(path, "JPEG", width, height, "no")
            f.seek(max(0, length - 2), 1)
    raise ValueError("JPEG dimensions not found")


def webp_info(path: Path) -> ImageInfo:
    with path.open("rb") as f:
        data = f.read(32)
    if data[:4] != b"RIFF" or data[8:12] != b"WEBP":
        raise ValueError("invalid WebP signature")
    chunk = data[12:16]
    if chunk == b"VP8X" and len(data) >= 30:
        flags = data[20]
        width = 1 + int.from_bytes(data[24:27], "little")
        height = 1 + int.from_bytes(data[27:30], "little")
        return ImageInfo(path, "WebP", width, height, "yes" if flags & 0x10 else "unknown/no")
    return ImageInfo(path, "WebP", None, None, "unknown", "dimensions require Pillow for this WebP variant")


def svg_info(path: Path) -> ImageInfo:
    text = path.read_text(encoding="utf-8", errors="ignore")[:16384]
    root = re.search(r"<svg\b([^>]*)>", text, flags=re.IGNORECASE | re.DOTALL)
    if not root:
        raise ValueError("SVG root not found")
    attrs = root.group(1)

    def numeric_attr(name: str) -> int | None:
        match = re.search(rf"\b{name}\s*=\s*['\"]\s*([0-9.]+)", attrs, flags=re.IGNORECASE)
        return round(float(match.group(1))) if match else None

    width = numeric_attr("width")
    height = numeric_attr("height")
    if width is None or height is None:
        view_box = re.search(
            r"\bviewBox\s*=\s*['\"]\s*[-0-9.]+\s+[-0-9.]+\s+([0-9.]+)\s+([0-9.]+)",
            attrs,
            flags=re.IGNORECASE,
        )
        if view_box:
            width = width or round(float(view_box.group(1)))
            height = height or round(float(view_box.group(2)))
    return ImageInfo(path, "SVG", width, height, "yes")


def inspect(path: Path) -> ImageInfo:
    suffix = path.suffix.lower()
    parsers = {
        ".png": png_info,
        ".gif": gif_info,
        ".bmp": bmp_info,
        ".jpg": jpeg_info,
        ".jpeg": jpeg_info,
        ".webp": webp_info,
        ".svg": svg_info,
    }
    parser = parsers.get(suffix)
    if not parser:
        raise ValueError("unsupported format")
    return parser(path)


def iter_images(root: Path) -> Iterable[Path]:
    extensions = {".png", ".gif", ".bmp", ".jpg", ".jpeg", ".webp", ".svg"}
    for path in sorted(root.rglob("*")):
        if path.is_file() and path.suffix.lower() in extensions:
            yield path


def write_markdown(items: list[ImageInfo], root: Path, output: Path) -> None:
    lines = [
        "# Reference UI asset inventory",
        "",
        "| Asset | Format | Width | Height | Transparency | Notes |",
        "|---|---|---:|---:|---|---|",
    ]
    for item in items:
        rel = item.path.relative_to(root).as_posix()
        lines.append(
            f"| `{rel}` | {item.format} | {item.width or ''} | {item.height or ''} | "
            f"{item.transparency} | {item.note} |"
        )
    output.write_text("\n".join(lines) + "\n", encoding="utf-8")


def write_csv(items: list[ImageInfo], root: Path, output: Path) -> None:
    with output.open("w", newline="", encoding="utf-8") as f:
        writer = csv.writer(f)
        writer.writerow(["asset", "format", "width", "height", "transparency", "notes"])
        for item in items:
            writer.writerow([
                item.path.relative_to(root).as_posix(),
                item.format,
                item.width or "",
                item.height or "",
                item.transparency,
                item.note,
            ])


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, required=True, help="Directory containing UI assets")
    parser.add_argument("--output", type=Path, default=Path("reference-ui-assets.md"))
    args = parser.parse_args()

    root = args.root.resolve()
    if not root.is_dir():
        parser.error(f"asset root does not exist or is not a directory: {root}")

    items: list[ImageInfo] = []
    errors: list[str] = []
    for path in iter_images(root):
        try:
            items.append(inspect(path))
        except Exception as exc:  # keep inventory useful even when one file is malformed
            errors.append(f"{path.relative_to(root).as_posix()}: {exc}")

    output = args.output.resolve()
    output.parent.mkdir(parents=True, exist_ok=True)
    if output.suffix.lower() == ".csv":
        write_csv(items, root, output)
    else:
        write_markdown(items, root, output)

    print(f"Wrote {len(items)} assets to {output}")
    if errors:
        print("Warnings:")
        for error in errors:
            print(f"- {error}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
