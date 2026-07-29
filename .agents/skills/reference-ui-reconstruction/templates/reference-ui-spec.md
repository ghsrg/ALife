# Control Center Monitor reference UI specification

## Purpose and source material

This is an implementation-facing visual description, not a simulation or
interaction contract. UI Canon and accepted ADRs remain authoritative.

- Reference screenshot: `docs/ui/control-center-monitor-v3.png`.
- Asset directory: `docs/ui/`.
- Existing route/component: insufficient data; out of this audit's scope.
- Reference viewport: `1920 × 1080` CSS pixels.
- Device pixel ratio: unknown; validate at `1` unless later evidence differs.
- Known font(s): insufficient data.

## Environment

- Vite framework: not audited; no implementation is requested.
- Language/package manager/CSS/UI library/test tool: use the existing project.
- Visual intent: dark, compact scientific-instrument shell. The World map is
  the dominant task surface; navigation/context, map controls, inspection, and
  supporting data remain persistent surrounding surfaces.

## Asset inventory

| Asset | Intrinsic size | Transparency | Role | Tile/stretch/fixed | State | Notes |
|---|---:|---|---|---|---|---|
| `control-center-monitor-v3.png` | 1920 × 1080 | no | baseline screenshot | fixed reference | default | measured |
| `GLOBAL_NAVIGATION.png` | 1920 × 62 | yes | global navigation | fixed full-width strip | default | measured |
| `RUN_DATA_CONTEXT_BAR.png` | 1919 × 82 | yes | active run/data context | fixed strip below navigation | default | measured |
| `LAYERS_FILTERS.png` | 262 × 644 | yes | layer/filter panel | fixed width | default | exact match at `(86, 144)` |
| `LEVEL_PANEL.png` | 83 × 644 | yes | level/zoom rail | fixed width | default | placement unresolved |
| `MAP.png` | 1202 × 630 | yes | primary World viewport | preserve aspect ratio | default | origin unresolved |
| `FOCUS_PANEL.png` | 413 × 399 | yes | selected-focus detail | fixed panel | selected state | coexistence unresolved |
| `CONTEXT_INSPECTOR.png` | 335 × 642 | yes | context inspector | fixed panel | selected/context state | coexistence unresolved |
| `DATA_PANEL.png` | 1914 × 281 | yes | bottom supporting data | nearly full-width | default | origin unresolved |

## Geometry

All values are measured in reference-image pixels.

| Region | X | Y | Width | Height | Parent | Layout rule | Notes |
|---|---:|---:|---:|---:|---|---|
| Viewport | 0 | 0 | 1920 | 1080 | — | fixed baseline | measured |
| Global navigation | 0 | 0 | 1920 | 62 | Viewport | fixed top strip | measured |
| Run/data context | 0 or 1 | 62 | 1919 | 82 | Viewport | fixed below navigation | inferred from stack |
| Main workspace | — | 144 | — | — | Viewport | begins after top stack | derived |
| Layers/filters | 86 | 144 | 262 | 644 | Main workspace | fixed left panel | exact image match |
| Level rail | — | 144 | 83 | 644 | Main workspace | fixed near map controls | x unresolved |
| Map | — | — | 1202 | 630 | Main workspace | dominant surface | origin unresolved |
| Focus panel | — | — | 413 | 399 | Main workspace | right contextual surface | state/origin unresolved |
| Context inspector | — | — | 335 | 642 | Main workspace | right contextual surface | state/origin unresolved |
| Data panel | — | — | 1914 | 281 | Main workspace | bottom supporting surface | origin unresolved |

## Panel composition

| Panel | Top-left | Top | Top-right | Left | Center | Right | Bottom-left | Bottom | Bottom-right |
|---|---|---|---|---|---|---|---|---|---|
| All supplied panels | not supplied | not supplied | not supplied | not supplied | fixed panel artwork/reference | not supplied | not supplied | not supplied | not supplied |

No supplied asset is a nine-slice frame, repeat strip, or sprite sheet. Do not
stretch artwork to invent responsive variants. Use CSS Grid: fixed tracks for
controls/context, `minmax(0, 1fr)` for the map, and a distinct bottom data row.
Use absolute positioning only for map-local overlays, selection rings, and
tooltips.

## Typography

| Style | Font | Weight | Size | Line height | Letter spacing | Alignment | Wrapping |
|---|---|---:|---:|---:|---:|---|---|
| Navigation labels | project font, unknown | — | compact | unknown | unknown | single-line | no wrap at baseline |
| Data values | project font, unknown | — | compact | unknown | unknown | stable numeric alignment | controlled truncation |
| Panel titles | project font, unknown | — | compact | unknown | unknown | panel-local | no decorative wrapping |

## Interaction state matrix

| Component | Trigger | State | Visual change | Functional result | Evidence |
|---|---|---|---|---|---|
| Global navigation | workspace selected | selected | persistent active item | changes workspace only | inferred |
| Run/data context | fixture/live/stale/unavailable | provenance state | explicit, never silently live | communicates projection state | UI plan |
| Layer/filter | click/key activation | enabled/selected/disabled | distinguishable state | presentation only; no simulation mutation | inferred |
| Level rail | hover/active/focus-visible | compact feedback | viewer presentation/navigation only | inferred |
| Map | hover/selection/pan/zoom | feedback aligns with entities | UI selection/navigation state | UI plan |
| Focus/context | no selection/selection/missing data | empty and unavailable states | observes selection, never mutates World | UI Canon |
| Data panel | loading/empty/partial/error | legible panel-local state | presents available projection data | UI quality direction |

## Scroll and overflow

| Region | Scroll axis | Fixed/sticky elements | Clipping behavior | Notes |
|---|---|---|---|---|
| Navigation | none evidenced | fixed top | own decoration only | z-index 40 |
| Run/data context | none evidenced | fixed under navigation | own decoration only | z-index 30 |
| Map | none | map-local controls | clip renderer, not deliberate overlays | z-index 10 |
| Context panels | vertical when content exceeds bounds | panel heading | internal scroll only | z-index 20 |
| Data panel | internal only | baseline panel visible | no page-wide overflow | z-index 20 |

## Responsive rules

- Baseline: reproduce `1920 × 1080` first.
- Narrower: at `1024 × 768`, preserve navigation/context and usable map; use
  controlled collapse or internal scroll, never accidental page-wide overflow.
- Wider: give additional width to the map before growing fixed-panel artwork.
- Smallest supported viewport: no mobile composition is supplied; do not invent
  it in the visual-acceptance slice.

## Validation log

| Pass | Screenshot | Largest mismatch | Root cause | Change made | Result |
|---:|---|---|---|---|---|
| 1 | reference asset inventory | panel origins except Layers/Filters | exports are not byte-identical composites | defer offsets to overlay validation | geometry facts and uncertainties recorded |

## Known limitations

- Missing assets: none from the requested set.
- Missing font: exact family, metrics, and weights are insufficient data.
- Ambiguous behaviour: Focus and Context Inspector may coexist, nest, or be
  alternative states; controls, chart semantics, commands, hit areas, and
  keyboard behaviour are not derivable from the images.
- Environment-dependent rendering: validate integer CSS-pixel seams at device
  scale factor `1` using an alpha overlay against the baseline screenshot.
