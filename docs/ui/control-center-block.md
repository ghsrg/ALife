---
tags:
  - alife
  - ui
  - canon
  - control-center
  - layout
  - blocks
---

# Специфікація блоків Control Center Monitor

> Block-level опис final вигляду та interaction Monitor.
>
> Canonical rules, data sources і state semantics визначає
> [[docs/ui/control-center-design-spec|Control Center Design Specification]].
> Цей файл задає їхню композицію за reference PNG.

## Reference and baseline

Baseline viewport: **1920 × 1080 CSS px**.
Minimum supported full Monitor layout: **1366 × 862 CSS px**. Below this,
retain minimum Grid tracks and use root/page vertical scroll; full-grid fidelity
is not an acceptance target.

| Block | Asset | Width | Height | Role |
|---|---|---:|---:|---|
| Global Navigation | `GLOBAL_NAVIGATION.png` | 1920 | 62 | persistent top strip |
| Run & Data Context Bar | `RUN_DATA_CONTEXT_BAR.png` | 1919 | 82 | persistent context strip |
| Level Panel | `LEVEL_PANEL.png` | 83 | 644 | active research lens |
| Layers & Filters | `LAYERS_FILTERS.png` | 262 | 644 | map presentation controls |
| World View | `MAP.png` | 1202 | 630 | primary World surface |
| Focus Panel | `FOCUS_PANEL.png` | 413 | 399 | conditional Map overlay |
| Contextual Inspector | `CONTEXT_INSPECTOR.png` | 335 | 642 | fixed right detail track |
| Data Panel | `DATA_PANEL.png` | 1914 | 281 | fixed bottom analytical surface |

Only `LAYERS_FILTERS.png` has a confirmed origin: **(86, 144)**. Other asset
origins require browser-overlay validation; they are not CSS offsets to guess.

## Composition and layout invariants

```text
Viewport
├─ Global Navigation
├─ Run & Data Context Bar
└─ Monitor
   ├─ Level Panel
   ├─ Layers & Filters
   ├─ World View
   │  └─ Focus Panel
   ├─ Contextual Inspector
   └─ Data Panel
```

Monitor uses a stable CSS Grid. At baseline, Level/layer/filter/selection/content
switches never resize Map or another block. Only application viewport resize or
full-screen changes Map size. Every track has a minimum useful size.
Users cannot drag-resize Monitor tracks; this prevents layout drift and
zero-size panels.

`Map fullscreen` leaves only Map and eligible Focus overlay visible. It
preserves viewport, layers, selection, Pin, and Data Context; Inspector, Level,
Layers, and application chrome do not consume Map space. The same Data Panel
content can be raised on demand as a bottom overlay at its normal Monitor track
height, without resizing Map. Fullscreen is view-only: Run commands and all
other controls require returning to normal Monitor.

Between `862` and `1080` CSS px viewport height, vertical tracks use bounded
responsive sizing; extra height grows mainly Map/Data area. This is not global
UI scaling: text and interaction targets keep a readable minimum.

## 1. Global Navigation

Visible and clickable order:

```text
Monitor | World Editor | Experiments | Evolution | Library | Analysis
```

| Action | Source | Result |
|---|---|---|
| Warnings | Core/Observer diagnostics severity/count | opens filtered Diagnostics |
| Theme | local preference | Dark/Light only |
| Locale | local preference | `uk-UA` / `en-US` |
| Help | active workspace and Level | non-blocking contextual Canon/docs help |
| Settings | local UI preferences | scale, density, accessibility, shortcuts, connection |

No disabled decorative placeholders.

## 2. Run & Data Context Bar

Always shows source/state, run/scenario identity, displayed Tick, simulation rate,
Visual FPS, Seed, Frame Age, and run controls.

| Item | Source | Behaviour |
|---|---|---|
| `SEED` | Runner `effective_seed` | read-only; set before launch through World Editor/launch flow |
| `FRAME AGE` | Runner latest committed Tick from independently refreshed live status − displayed projection Tick; optional secondary receive time − latest frame `wall_clock_generated_at_ms` | primary distance from live, not RTT/latency; `N ticks · M ms` while Runner advances; explicit Core `Paused` does not accumulate Tick lag; unavailable status is `stale/disconnected`, never fake `0` |
| Visual FPS | renderer telemetry | independent from simulation rate |
| Play/Pause, Step | approved commands | follow Run/Interaction Canon |
| Speed | approved target simulation rate command | Core life/execution rate, not visualization FPS; logarithmic slider plus editable TPS input; default is contract real-time TPS; finite `1…10,000 ticks/s`; adjacent `Unlimited` is a separate explicit command/state and disabling it restores previous finite TPS; separate `Real-time` resets finite rate and disables `Unlimited` |
| Stop | approved command | confirmation with run id and displayed Tick |

`Jump to Live`, `Reset view`, RTT, and `Latency` are absent. Debug buffer returns
to its `LIVE` marker without a Core command.

## 3. Level Panel

Use icons in this order:

| Icon | Level | Selection |
|---|---|---|
| globe | World | one canonical `World block` (one Resource/Field grid cell); multi-selection is Inspector-only |
| dotted ring | Cells | Cell |
| connected spheres | Organisms | observer-side OrganismView at displayed Tick |
| graph | Lineages | Cell selects its `lineage_id` |
| DNA | Evolution | Cell selects its Genome |
| bar chart | Analytics | chart/bar/segment selects analytical subset; no Focus by default |

Level is an active research lens, not workspace navigation. It preserves Data
Context and camera, while changing Map interpretation, permitted layers,
Inspector, Focus, and Data Panel. Incompatible selection clears explicitly.

Unmodified click-drag pans Map; wheel/scroll zooms it. `Shift + click`
adds/removes one compatible Map target to/from the current selection set.
`Shift + drag-select` draws a selection frame and adds all compatible targets
it intersects. Multi-selection is Inspector/Data Panel context only and never
opens Focus. Clicking empty Map clears selection and returns Inspector to
`World total`.
During live projection, an unpinned selection follows its entity/set and
refreshes on every displayed Tick. `Pause` freezes displayed context; `Pin` is
a separate read-only comparison baseline.
`Dead` remains a valid lifecycle state, so selection stays and shows final
data. If a target genuinely disappears from projection, its selection clears,
Focus closes, and UI shows a temporary reason.

## 4. Layers & Filters

One vertical list, grouped as:

```text
Fields | Resources | Cell Energy | Structure | Selection
```

Cells are permanent structural foreground. Resource/Field layers are
data-bound backgrounds that composite their own colours; `Primary Color Mode`
does not exist. Only dynamic Fields/Resources rows scroll.

Each active dynamic layer row exposes swatch, name, toggle, and compact gradient.
Expansion reveals unit, min/max, normalization, and full legend.

| Control | Behaviour |
|---|---|
| Cell Energy | mutually exclusive `Cells` direct encoding or `Heatmap` aggregate in the same `World block` grid |
| Energy Heatmap | always shows unit, bin size, sampling, and aggregate provenance |
| Joints | neutral structural toggle; future coloured types require source data |
| Trail | one selected target; retained `(Tick, x, y)` samples after selection only |

`Cell Energy`, `Joints`, and `Trail` are available only at `Cells` and
`Organisms` Level. On other Levels they are absent and not rendered; their
user state restores on return to a compatible Level and persists directly
between `Cells` and `Organisms`.

`Fields` and `Resources` are available at every Level. They are enabled by
default at `World`, `Cells`, and `Organisms`; at `Lineages`, `Evolution`, and
`Analytics` they start disabled but remain available on demand, keeping carrier
and analytical highlights readable.

Once changed by the user, common-layer state persists across Level changes;
defaults apply only before a user override.

`Lineages`, `Evolution`, and `Analytics` add no separate Layer by default:
their carrier/metric highlighting is presentation state, not a duplicate layer.
A contextual Level-specific layer requires distinct source-backed raster or
geometry.

Per-layer min/max and normalization change colour scale only; they never hide
or exclude Map values.

Layers/filters are Map presentation state only: they neither change Data
Context nor remove data from Inspector/Data Panel or selection, and never
change WorldState or Tick. The only cross-surface exception is explicit
`Analytics` interaction, which may highlight its source-backed subset on Map
according to the Analytics rule. This presentation highlight leaves current
selection and Inspector unchanged.

## 5. World View

World View renders committed projections only.

- initialization and scenario changes apply `Fit World`;
- Fit World maximizes whole World within Map while preserving aspect ratio;
- panel switches/live frames do not reset pan or zoom;
- viewport resize preserves world-space center and zoom, then may clamp bounds;
- single click selects according to Level;
- double click selected compatible target opens/closes Focus;
- Analytics subsets use `Highlight` by default; `Hide` and `Isolate` are explicit.

## 6. Contextual Inspector

Inspector always occupies the right track.

Without selection, `World total` displays source-backed lifecycle counts, total
Cell Energy, Resource totals, Joint count, Tick, run state, completeness, and
warnings. With selection, it shows compatible detail or aggregate selection-set
values.

`Pin` stores one read-only baseline:

```text
entity or selection set + Data Context + displayed Tick + completeness
```

New selection is current target. Data Panel compares only compatible `Pinned
baseline` and `Current selection`. If current target disappears, selection
clears but pinned baseline remains until `Unpin`.
Starting a new run after `Stop` clears Pin, current selection, and Focus because
their identities belong to the prior run.
For a compatible Data Panel comparison, current data remains primary; `Pinned
baseline` overlays it as a labelled outline/dashed contour on the same scale.
This is a separate selection-comparison surface for pinned/current targets, not
a recalculation of Level baseline distributions/histograms. UI does not create
a second duplicate chart card.

## 7. Focus Panel

Focus is a stable `413 × 399` upper-right Map overlay. It never resizes Map or
moves with target. `Escape`/close dismisses it; multi-selection does not open it
and remains Inspector-only.

It is Level-bound and procedural/data-bound: supported geometry, Materials,
Joints, Processes, Genome, and projection data define pseudo-3D content.
Behavior/role can add only labelled provenance with confidence, interval, and
classifier version; it cannot create invented anatomy or biological types.

At `Analytics` Level, Focus is absent by default. Metric provenance,
aggregation, interval, and detail belong to Data Panel.

## 8. Data Panel

Data Panel has no tabs. Its content is:

```text
active Level + analysis scope + optional Pin
```

`analysis scope` is `World total` with no World selection, one selected `World
block` at World Level, or current multi-selection at any compatible Level. A
single Cell/Organism selection does not rebuild distribution charts/histograms;
its detail belongs to Inspector/Focus. With compatible `Pin`, it also supplies
the current side of separate selection-comparison surface.

Vertical overflow of Data Panel content belongs to root/page scrolling, not a
Data Panel-only layout scrollbar. Monitor's only local vertical scroll is the
dynamic `Fields`/`Resources` list.

`Raw Data` is Diagnostics. With Pin, only compatible baseline/current comparison
is shown.

| Level | Required content |
|---|---|
| World | Population Lifecycle; selected Matter Cycle/Energy Flow; time evolution |
| Cells | Observed primary roles with Potential markers; Cell radius distribution |
| Organisms | primary observed Behavior Profiles; Cell-count size bins |
| Lineages | current population, history, genealogy, spatial footprint |
| Evolution | Genome provenance, mutation history, diversity, carrier history |
| Analytics | selected metric with complete provenance |

Population Lifecycle is a stacked bar from `Alive`, `Stressed`, `Dormant`, and
`Dead` counts in the same displayed projection. Labels and exact counts remain
visible.

With one selected `World block`, every World-level aggregate is scoped to that
block. Without selection, it is scoped to `World total`.
The same rule applies at every Level: a compatible entity or selection set
scopes all Data Panel aggregates, distributions, and histograms to itself;
without selection, the Level baseline context is used.

World accounting has explicit target selection: `Resource`, `Material`, or
`Energy`. `Energy` is the default target for a new run; an explicit user choice
is retained for the rest of that run. Matter Cycle keeps Resources, Materials,
MaterialFragments,
decomposing Cells, and sinks distinct. Energy Flow needs Core/Observer
accounting; until then it is `unavailable` and never estimated by UI.

Cycle shows location shares with absolute target total. Its companion time chart
is stacked `100%` distribution across locations; absolute target amount is
shown separately, so distribution and total change are not confused.

Selecting `Resource` or `Material` requires an explicit second selector for one
registry type. UI does not combine all types into one Cycle without a separate
validated accounting contract; `Energy` requires no second selector.

Every time series uses one UI RRD compact history, without `Recent` or `Since
start` modes. Maximum: 1,000 samples, including 100 newest consecutive samples
and successive 10× decimation tiers. Its axis uses actual Tick/time positions
and visibly communicates changing sampling density, never implying equal
intervals between decimated samples. For numeric series, a collapsed interval
stores its mean and the chart connects aggregated samples as a trend. This is
not a full World-frame store. A collapsed Trail interval stores mean `(x, y)`,
so older route is smooth but approximate; tooltips identify interval and
aggregation.

## Validation

At `1920 × 1080` validate in a real browser at device scale factor `1` with an
alpha overlay against `control-center-monitor-v3.png`. Correct Grid tracks,
top stack, Layers/Filters origin, Map, Inspector, Focus, and Data Panel before
typography or decoration.

## Related documents

- [[docs/ui/control-center-design-spec|Control Center Design Specification]]
- [[docs/ui/principles|UI Principles]]
- [[docs/ui/navigation|UI Navigation]]
- [[docs/ui/visualization|UI Visualization]]
- [[docs/ui/exploration|UI Exploration]]
- [[docs/ui/analytics|UI Analytics]]
- [[docs/ui/interaction|UI Interaction]]
- [[docs/ui/quality|UI Quality]]
