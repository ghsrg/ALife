# Panel composition guide

Read this file when the interface is assembled from image slices, textured borders, sprites, or legacy panel artwork.

## Identify the slice model

### Three-slice horizontal

Use when a control has fixed left and right caps and a flexible middle.

- left cap: fixed intrinsic width;
- middle: repeat-x or stretch according to texture continuity;
- right cap: fixed intrinsic width.

### Three-slice vertical

Use when a panel has fixed top and bottom and a flexible body.

- top: fixed intrinsic height;
- body: repeat-y or stretch;
- bottom: fixed intrinsic height.

### Nine-slice

Use for framed rectangles with four corners, four edges, and a center.

Recommended track model:

```css
.frame {
  display: grid;
  grid-template-columns: var(--left) minmax(0, 1fr) var(--right);
  grid-template-rows: var(--top) minmax(0, 1fr) var(--bottom);
}
```

Corners must keep their intrinsic dimensions. Edges tile or stretch only in their long axis. The center may tile or stretch depending on the artwork.

## How to decide tile versus stretch

Prefer tiling when:

- a repeated motif is visible;
- both ends of the strip appear designed to continue;
- stretching produces obvious distortion;
- pixel-art texture density should stay constant.

Prefer stretching when:

- the asset is a smooth gradient;
- it contains no fixed-size motif;
- the reference shows continuous scaling;
- the source appears explicitly authored as a stretch region.

## Common mistakes

- using `cover` on a frame;
- stretching corner artwork;
- using one image for all edges despite different lighting;
- introducing subpixel tracks that blur seams;
- overlooking transparent padding;
- placing content above decorative inset bounds;
- setting `overflow: hidden` on a parent that clips intended shadows or overlays;
- reconstructing supplied artwork with approximate CSS.

## Sprite handling

Record:

- sprite sheet dimensions;
- each frame rectangle;
- frame spacing and transparent padding;
- normal, hover, pressed, selected, and disabled states;
- intended scale.

Use exact integer `background-position` values. Avoid CSS scaling before verifying native-size rendering.

## Seam diagnosis

A one-pixel seam may come from:

- fractional grid tracks;
- device-pixel ratio mismatch;
- transparent border pixels in the image;
- `background-size` rounding;
- content box versus border box dimensions;
- transformed parents;
- interpolation caused by non-integer scaling.

Remove the root cause before hiding the seam with overlap or negative margins.
