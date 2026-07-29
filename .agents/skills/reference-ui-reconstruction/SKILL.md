---
name: reference-ui-reconstruction
description: Reconstructs Vite-based web interfaces from reference screenshots, sliced panels, sprites, and supplied UI assets with measured geometry, faithful behavior, browser rendering, screenshot comparison, and iterative visual correction. Use for pixel-accurate UI implementation, mismatched dimensions, incorrect panel slicing, layout drift, or interactions that do not match a reference.
---

# Reference UI Reconstruction

Reconstruct the supplied interface. Do not redesign it.

The task is not complete when the code merely compiles. Completion requires a browser-rendered comparison against the reference and a correction loop.

## Required workflow

Follow these phases in order:

1. Inspect the project and detect its actual Vite framework and conventions.
2. Inventory every supplied reference image and UI asset.
3. Measure the reference geometry and write `reference-ui-spec.md`.
4. Determine how every sliced panel is composed.
5. Implement the reference viewport before adding responsiveness.
6. Reconstruct evidenced interaction states and behavior.
7. Run the application in a real browser.
8. Capture screenshots at the exact reference viewport.
9. Compare reference and actual images using a diff or alpha overlay.
10. Fix the largest mismatch and repeat.
11. Run build, lint, tests, and final browser checks.
12. Report verified results and unresolved differences.

Do not skip phases 2, 3, 8, or 9.

## 1. Inspect the existing Vite project

Before changing code, inspect:

- `package.json` and the lock file;
- `vite.config.*`;
- application entry points and routes;
- existing component and CSS architecture;
- installed UI libraries;
- existing browser tests;
- static asset directories;
- global resets, fonts, and `box-sizing`;
- current scripts for build, lint, type-check, and tests.

Detect the actual framework. Vite does not imply React. Preserve the current framework, package manager, router, CSS approach, and project conventions.

Do not add Tailwind, Bootstrap, shadcn/ui, a new router, or a new component library unless it is already used or the user explicitly requests it.

## 2. Inventory references and assets

Locate all:

- reference screenshots;
- panel slices and frame pieces;
- spritesheets;
- icons and state variants;
- backgrounds and textures;
- masks and overlays;
- font files;
- mockups showing alternate states.

Run the bundled inventory script when Python is available:

```bash
python .agents/skills/reference-ui-reconstruction/scripts/inspect_assets.py --root <asset-directory> --output reference-ui-assets.md
```

Run scripts with `--help` first.

For every asset record:

- path;
- intrinsic width and height;
- format;
- alpha/transparency availability;
- visible transparent padding when relevant;
- whether it should tile, stretch, remain fixed, or act as an overlay;
- its state, such as default, hover, pressed, selected, or disabled.

Never guess dimensions that can be measured.

## 3. Create the measurement specification

Copy the bundled template:

```bash
cp .agents/skills/reference-ui-reconstruction/templates/reference-ui-spec.md ./reference-ui-spec.md
```

On Windows PowerShell:

```powershell
Copy-Item .agents/skills/reference-ui-reconstruction/templates/reference-ui-spec.md ./reference-ui-spec.md
```

Fill it before major implementation.

Use reference-image pixels as the source coordinate system. If the intended viewport is not otherwise specified, use the reference screenshot's pixel dimensions as the baseline viewport.

Record at minimum:

- reference viewport;
- outer frame bounds;
- major panel bounds;
- fixed gutters and gaps;
- headers, sidebars, toolbars, and content regions;
- text baselines and line heights when visually important;
- controls and hit areas;
- overlay and modal bounds;
- scrollable regions;
- z-index ordering;
- state matrix.

Do not begin fine styling while major geometry remains undocumented.

## 4. Classify panel slices

Read `references/panel-composition.md` before implementing sliced panels.

Classify each image as one of:

- fixed corner;
- horizontal edge;
- vertical edge;
- repeat-x strip;
- repeat-y strip;
- repeating center texture;
- stretchable center;
- fixed decoration;
- foreground overlay;
- icon or sprite;
- state-specific image.

Rules:

- Never stretch corners.
- Never use `background-size: cover` for a sliced frame.
- Do not stretch patterned edges that should tile.
- Do not use the complete reference screenshot as the UI background with invisible click targets.
- Preserve intrinsic aspect ratio for supplied artwork.
- Avoid fractional dimensions on seam-sensitive panel boundaries.
- Inspect transparent margins before compensating with arbitrary CSS offsets.

Use CSS Grid, multiple backgrounds, pseudo-elements, or explicit child elements according to the actual slice model. Nine-slice frames should normally use fixed corner tracks and flexible center tracks.

## 5. Reconstruct layout from evidence

Choose layout primitives based on the reference:

- normal flow for document-like content;
- Grid for two-dimensional panel structures;
- Flexbox for one-dimensional control groups;
- absolute positioning for genuine overlays, decorations, or fixed-coordinate canvases;
- a mixed approach when the source design requires it.

Absolute positioning is not automatically wrong. It is wrong when used to compensate for an incorrectly understood parent layout.

Centralize measured values as CSS custom properties or typed constants instead of scattering unexplained numbers across files.

Check common systematic mismatch sources:

- browser default margins;
- wrong `box-sizing`;
- flex shrinking;
- `min-width: auto`;
- inherited line-height;
- incorrect font or delayed font loading;
- image baseline whitespace;
- transparent image padding;
- border inclusion in width/height;
- scrollbar width;
- browser zoom or device pixel ratio;
- percentage rounding;
- transforms used as layout fixes.

Fix root causes rather than adding layers of offsets.

## 6. Typography

Use supplied or existing project fonts where available.

Measure and document:

- family;
- weight;
- size;
- line height;
- letter spacing;
- alignment;
- wrapping and truncation;
- shadows, outlines, or antialias-sensitive effects.

Do not compensate for a wrong font by repeatedly changing container widths.

Before screenshots, wait for fonts:

```ts
await page.evaluate(() => document.fonts.ready);
```

If the exact font is unavailable, report it as a known visual limitation.

## 7. Behavior and state reconstruction

A screenshot defines appearance, not all behavior. Use the existing implementation, provided state assets, filenames, other mockups, and explicit requirements as evidence.

Build a state matrix for interactive components:

- default;
- hover;
- pressed/active;
- selected;
- disabled where applicable;
- focus-visible;
- open/closed;
- loading and empty states where applicable;
- scrolling and overflow;
- keyboard and escape behavior for overlays.

Do not invent complex behavior unsupported by evidence. Clearly label inferred behavior.

Ensure visual hit areas correspond to the intended control bounds. Do not create tiny hit areas inside large visual controls.

## 8. Implementation order

Implement in this order:

1. reference viewport and outer frame;
2. major structural regions;
3. sliced panel composition;
4. typography;
5. static controls;
6. interaction states;
7. overlays and scrolling;
8. responsive behavior;
9. one-pixel and decorative corrections.

Match the reference viewport first. Add broader responsiveness only after the baseline matches.

## 9. Browser validation

Use the existing browser-test stack. Prefer Playwright when it is already installed.

If no browser tool exists, add only the minimum setup required and do not silently replace the project's test stack.

Use:

- the exact reference viewport;
- `deviceScaleFactor: 1` unless the reference explicitly represents another scale;
- browser zoom at 100%;
- deterministic data;
- disabled animations and transitions;
- fully loaded fonts and images;
- the same browser and operating environment for every comparison.

A starter Playwright example is available at:

`examples/reference-ui.visual.spec.ts`

Capture at least:

- default screen;
- each materially different interactive state;
- open overlays or menus;
- the final corrected baseline.

## 10. Mandatory comparison loop

After every significant layout pass:

1. capture the actual browser screenshot;
2. generate or inspect the diff image;
3. generate an alpha overlay when useful;
4. locate the largest mismatch region;
5. identify its root cause;
6. fix one mismatch category;
7. capture again;
8. repeat.

Optional helper:

```bash
python .agents/skills/reference-ui-reconstruction/scripts/make_overlay.py \
  --reference path/to/reference.png \
  --actual path/to/actual.png \
  --output-dir artifacts/reference-ui
```

This script requires Pillow. If Pillow is unavailable, use Playwright's built-in screenshot comparison or ask before installing a dependency.

Correction priority:

1. viewport and outer frame;
2. major panel bounds;
3. seams and slice composition;
4. primary alignment;
5. text and line height;
6. control dimensions;
7. icons and decorations;
8. minor antialiasing or color differences.

Do not adjust small icons while major regions remain shifted.

Do not increase screenshot-diff tolerance merely to make a failing result pass.

## 11. Acceptance criteria

At the baseline reference viewport:

- the app renders without unexpected console errors;
- no required asset request fails;
- there is no unintended horizontal scrolling;
- interactive controls are not clipped;
- panel slices have no visible unintended gaps;
- structural boundaries are within one CSS pixel where measurable;
- supplied artwork is not distorted;
- text wrapping matches or the difference is explained;
- required interactions work;
- a browser screenshot was captured;
- a comparison or overlay was inspected;
- remaining differences are documented.

A claimed pixel-accurate result requires objective screenshot evidence. Code review alone is insufficient.

## 12. Responsive validation

When responsiveness is required, test:

- exact reference viewport;
- one narrower viewport;
- one wider viewport;
- smallest supported viewport.

Verify overflow, text wrapping, panel tiling/stretching, hit areas, overlay bounds, and image distortion.

If no mobile reference exists, preserve functionality and hierarchy rather than inventing a completely different interface.

## 13. Final checks

Run the project's existing commands for:

- formatter;
- lint;
- type-check;
- unit tests;
- browser tests;
- production build.

Do not fix unrelated repository-wide issues unless they block this task.

## Required completion report

Report:

- detected Vite framework and package manager;
- reference viewport;
- assets used and missing assets;
- files changed;
- browser scenarios tested;
- screenshot and diff locations;
- measured or observed visual-difference result;
- interactions verified;
- build/test results;
- verified matches;
- inferred decisions;
- unresolved mismatches.

Do not say “done”, “pixel perfect”, or “matches the reference” without browser comparison evidence.

## Failure conditions

Do not claim completion when:

- required assets are missing;
- the reference is cropped or inconsistent;
- panel slices do not cover all required regions;
- exact fonts are unavailable;
- behavior has no supporting evidence;
- the app cannot be launched;
- browser rendering or screenshot comparison cannot be performed.

Continue with the best verifiable implementation, but clearly distinguish verified results, inferences, and unresolved limitations.
