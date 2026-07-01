---
tags:
  - alife
  - worklog/plan
---

# PLAN: Documentation Cleanup and Optimization

Дата: 2026-06-30 13:19

## Goal

Очистити поточну документацію від LLM-шуму, повторів, службових блоків, надмірних прикладів і розмазаних визначень, не змінюючи прийняті правила світу.

Obsidian links/backlinks у цьому плані не чіпати. Це окремий наступний крок.

## Current State Summary

Документація після GZ-01..GZ-18 має коректніші правила, але нерівномірну форму:

- великі Canon-файли часто мають 1000+ рядків;
- у багатьох файлах є повторний шаблон `Призначення -> Основна ідея -> Що НЕ є -> десятки пояснень -> базова модель -> приклади -> правила -> заборонено -> Open Questions`;
- config-файли містять багато `text id="..."`, `yaml id="..."` службових атрибутів;
- приклади часто займають третину або більше документа і дублюють правила;
- частина "базова модель" розмазана між Canon, engine і config;
- Open Questions частково містять уже прийняті рішення або реалізаційні параметри;
- старі великі файли дублюють нові contract-документи, додані під час GZ-аудиту.

## Cleanup Principles

1. Canon-файл має містити правило, межі відповідальності, мінімальну модель і посилання на деталі, а не довгу лекцію.
2. Приклади залишати лише там, де вони пояснюють неоднозначний механізм.
3. Якщо приклад корисний, але великий, винести його в `docs/examples/`.
4. Якщо правило вже винесене в contract-документ, у старому файлі лишити короткий summary і посилання.
5. `Open Questions` повинні містити тільки реальні відкриті питання, не backlog реалізації й не вже прийняті рішення.
6. `Що НЕ є` і `Заборонено` не повинні дублювати одне одного. У більшості файлів достатньо одного короткого `Заборонено`.
7. Службові code block ids прибрати повністю.
8. Не робити Obsidian link normalization у цьому проході.

## Target Document Shape

Для великих Canon-файлів цільова структура:

```text
# filename.md

> one-line purpose

# Призначення
# Відповідальність файлу
# Canon Rules
# Minimal Model / Baseline
# Interfaces / Related Contracts
# Заборонено
# Open Questions
```

Для config-файлів:

```text
# *_config.md

# Призначення
# Schema Summary
# Required Fields
# Optional Fields
# Validation
# Minimal Example
# Заборонено
# Open Questions
```

Для examples:

```text
# <domain>-examples.md

# Purpose
# Example 1
# Example 2
...
```

## Phase 0: Safety Snapshot

**Files:** no Canon edits.

- [ ] Confirm current working tree before cleanup with `git status --short`.
- [ ] Confirm all GZ contract files exist.
- [ ] Record current line counts for `docs/**/*.md`.
- [ ] Record current code block id count with `rg -n 'id="' docs`.
- [ ] Do not stage or commit automatically.

## Phase 1: Mechanical Noise Removal

**Scope:** all `docs/**/*.md`.

**Goal:** remove formatting artifacts that do not carry domain meaning.

- [ ] Remove code fence attributes like:

```text
```text id="abc123"
```yaml id="abc123"
```

Change to:

```text
```text
```yaml
```

- [ ] Remove accidental LLM wording patterns:

```text
базова модель базова модель
базова модель для базової моделі
Базова модель базової моделі
Recommendation базової моделі
Recommended базова модель
```

- [ ] Normalize section titles:

```text
Recommendation базової моделі -> Базова модель
Minimal ... Для базової моделі -> Мінімальна модель
Example -> Приклад
```

- [ ] Keep code examples themselves if still useful; only remove block ids and noisy titles.

**Verification:**

- [ ] `rg -n 'id="' docs` returns no code block ids or only intentionally retained semantic ids.
- [ ] `rg -n 'базова модель базова модель|Базова модель базової моделі|Recommendation базової моделі|Recommended базова модель' docs` returns nothing.

## Phase 2: Create Examples Area

**Create:**

- `docs/examples/README.md`
- `docs/examples/biology-examples.md`
- `docs/examples/genetics-examples.md`
- `docs/examples/config-examples.md`
- `docs/examples/engine-examples.md`

**Goal:** preserve useful scenarios without letting Canon files become tutorials.

- [ ] Create `docs/examples/README.md` explaining that examples are illustrative, not new Canon rules.
- [ ] Move long lifecycle/cell/joint/communication/organism examples to `biology-examples.md`.
- [ ] Move mutation/runtime/recombination/HGT/inheritance examples to `genetics-examples.md`.
- [ ] Move long YAML scenario/config examples to `config-examples.md`.
- [ ] Move scheduler/storage/serialization/performance examples to `engine-examples.md`.
- [ ] In source files, replace moved example blocks with a one-line pointer:

```text
Довші приклади винесені в `docs/examples/...`.
```

**Do not do yet:** Obsidian-style backlink cleanup.

## Phase 3: Collapse Contract Duplicates

**Goal:** use new GZ contract files as the single place for cross-cutting rules.

### Tick / Scheduler

**Files:**

- `docs/world/tick.md`
- `docs/world/tick-semantics.md`
- `docs/engine/scheduler.md`

**Plan:**

- [ ] Keep detailed snapshot/delta/commit/same-tick rules only in `tick-semantics.md`.
- [ ] Compress `tick.md` to conceptual Tick lifecycle and phase list.
- [ ] Compress `scheduler.md` to engine responsibility and required semantic invariants.
- [ ] Remove duplicated same-tick examples from `tick.md` and `scheduler.md`.

### Feasibility / Processes / Energy

**Files:**

- `docs/biology/feasibility.md`
- `docs/biology/processes.md`
- `docs/biology/process-progress.md`
- `docs/world/energy.md`

**Plan:**

- [ ] Keep action rejection semantics only in `feasibility.md`.
- [ ] Keep long-running process semantics only in `process-progress.md`.
- [ ] Keep Energy budget ordering only in `energy.md`, with short references elsewhere.
- [ ] In `processes.md`, reduce process catalog to canonical process list and interfaces.

### Field / Reactions / Chemistry

**Files:**

- `docs/world/field-semantics.md`
- `docs/world/fields.md`
- `docs/world/reactions.md`
- `docs/engine/chemistry.md`
- `docs/config/reactions_config.md`

**Plan:**

- [ ] Keep field behavior contract in `field-semantics.md`.
- [ ] Keep Field catalog and conceptual model in `fields.md`.
- [ ] Keep reaction semantics in `reactions.md`.
- [ ] Keep implementation responsibility in `engine/chemistry.md`.
- [ ] Keep schema details in `reactions_config.md`.

### Division / Inheritance / Lifecycle

**Files:**

- `docs/biology/division-partition.md`
- `docs/biology/lifecycle.md`
- `docs/genetics/inheritance.md`
- `docs/genetics/heredity.md`

**Plan:**

- [ ] Keep physical partition table only in `division-partition.md`.
- [ ] Keep lifecycle transitions in `lifecycle.md`.
- [ ] Keep inheritance mechanics in `inheritance.md`.
- [ ] Keep heredity concept and lineage meaning in `heredity.md`.
- [ ] Remove duplicated division examples from `lifecycle.md` and `inheritance.md` after moving useful ones to examples.

### Genome Interface / Runtime / Representation / Network

**Files:**

- `docs/genetics/regulatory-interface.md`
- `docs/biology/genome.md`
- `docs/genetics/genome-runtime.md`
- `docs/genetics/genome-representation.md`
- `docs/genetics/regulatory-network.md`

**Plan:**

- [ ] Keep input/output boundary only in `regulatory-interface.md`.
- [ ] Keep high-level biological meaning in `biology/genome.md`.
- [ ] Keep execution algorithm in `genome-runtime.md`.
- [ ] Keep data shape in `genome-representation.md`.
- [ ] Keep graph structure rules in `regulatory-network.md`.
- [ ] Remove repeated "Genome is not behavior script" blocks where already covered by short `Заборонено`.

## Phase 4: Reduce Large Domain Files

**Priority files by bloat and repetition:**

1. `docs/biology/organism.md`
2. `docs/biology/joint.md`
3. `docs/biology/communication.md`
4. `docs/biology/lifecycle.md`
5. `docs/biology/cell.md`
6. `docs/biology/specialization.md`
7. `docs/biology/processes.md`

**Goal:** each file becomes domain authority, not a tutorial.

### `biology/organism.md`

- [ ] Keep: organism as observer/analytics view, connected component, dependency metrics, forbidden global controller.
- [ ] Move: tissue/organ/movement/fragmentation/collapse examples to `docs/examples/biology-examples.md`.
- [ ] Remove: repeated explanations that organism is not brain/body/species if already in `Заборонено`.
- [ ] Target size: under 450 lines.

### `biology/joint.md`

- [ ] Keep: Joint object, channels, material basis, creation, maintenance, damage, division/death behavior.
- [ ] Move: colony/resource sharing/signal path/mechanical shell examples.
- [ ] Collapse repeated Joint type descriptions into one parameter table.
- [ ] Target size: under 500 lines.

### `biology/communication.md`

- [ ] Keep: signal definition, channels, scalar baseline, carrier/material requirement, same-tick semantics.
- [ ] Move: trace following, signal chain, tissue coordination examples.
- [ ] Collapse signal gain/threshold/accumulation into concise subsections or move formulas to examples.
- [ ] Target size: under 450 lines.

### `biology/lifecycle.md`

- [ ] Keep: lifecycle states, transition rules, death/decomposition, references to `cell-state.md` and `division-partition.md`.
- [ ] Move: ten narrative examples to examples file.
- [ ] Remove duplicated partition details already in `division-partition.md`.
- [ ] Target size: under 450 lines.

### `biology/cell.md`

- [ ] Keep: Cell state inventory, state boundaries, capacity, Boundary, Energy, local inputs, core rules.
- [ ] Move neural-like/signaling elaboration to `communication.md` or examples.
- [ ] Remove duplicated lifecycle/process/genome explanations already owned by other docs.
- [ ] Target size: under 500 lines.

### `biology/specialization.md`

- [ ] Keep: specialization definition, emergence conditions, no hardcoded cell types, observer/debug role labels.
- [ ] Move role examples to examples.
- [ ] Collapse repeated tissue/organ-like text into references to `organism.md`.
- [ ] Target size: under 400 lines.

### `biology/processes.md`

- [ ] Keep: process definition, process list, active/passive split, Feasibility and ProcessProgress references.
- [ ] Remove detailed per-process teaching text if duplicated by contract docs.
- [ ] Keep one concise table of processes with inputs/outputs/constraints.
- [ ] Target size: under 500 lines.

## Phase 5: Reduce Large Genetics Files

**Priority files:**

1. `docs/genetics/mutation.md`
2. `docs/genetics/regulatory-network.md`
3. `docs/genetics/genome-runtime.md`
4. `docs/genetics/epigenetics.md`
5. `docs/genetics/inheritance.md`
6. `docs/genetics/horizontal-transfer.md`
7. `docs/genetics/heredity.md`
8. `docs/genetics/recombination.md`
9. `docs/genetics/genome-representation.md`

**Goal:** keep Canon decisions; move educational examples and future branches out.

### Mutation / Recombination / HGT

- [ ] Keep mutation/recombination/HGT definitions and forbidden shortcuts.
- [ ] Keep minimal baseline/future-compatible boundary.
- [ ] Move operator examples and scenario examples to `genetics-examples.md`.
- [ ] Move broad future alternatives that are not Canon into existing `docs/research/` files if they are not already there.
- [ ] Target each file: under 450 lines, except `mutation.md` may be under 550 if operator tables remain.

### Runtime / Network / Representation

- [ ] Use `regulatory-interface.md` as input/output authority.
- [ ] Use `genome-runtime.md` for execution only.
- [ ] Use `regulatory-network.md` for graph semantics only.
- [ ] Use `genome-representation.md` for storage/data shape only.
- [ ] Remove duplicated runtime examples and repeated baseline paragraphs.
- [ ] Target each file: under 500 lines.

### Epigenetics / Inheritance / Heredity

- [ ] Keep state layer boundaries and inheritance rules.
- [ ] Remove repeated learning-not-mutation explanations after one canonical statement.
- [ ] Move narrative examples to `genetics-examples.md`.
- [ ] Target each file: under 450 lines.

## Phase 6: Reduce Config Files

**Files:**

- `docs/config/world_config.md`
- `docs/config/fields_config.md`
- `docs/config/resources_config.md`
- `docs/config/materials_config.md`
- `docs/config/reactions_config.md`
- `docs/config/stability_bounds.md`

**Goal:** convert config docs from long tutorials to schemas.

- [ ] Keep one `Schema Summary` per config file.
- [ ] Keep required/optional fields as concise tables.
- [ ] Keep one minimal YAML example per file.
- [ ] Move long scenario examples to `docs/examples/config-examples.md`.
- [ ] Remove code fence ids.
- [ ] Remove repeated "not hardcoded Earth / not toxicity / not behavior" sections if already covered by short `Заборонено`.
- [ ] Target large config files:
  - `fields_config.md`: under 450 lines.
  - `materials_config.md`: under 500 lines.
  - `resources_config.md`: under 400 lines.
  - `world_config.md`: under 400 lines.

## Phase 7: Reduce Engine Files

**Files:**

- `docs/engine/ecs.md`
- `docs/engine/scheduler.md`
- `docs/engine/chemistry.md`
- `docs/engine/physics.md`
- `docs/engine/performance.md`
- `docs/engine/rendering.md`
- `docs/engine/serialization.md`
- `docs/engine/storage.md`

**Goal:** engine docs should explain implementation responsibility, not repeat world laws.

- [ ] Keep "engine implements, does not define laws" boundary.
- [ ] Remove biological examples if they belong to `biology/` or `world/`.
- [ ] Move format examples to `docs/examples/engine-examples.md`.
- [ ] Keep minimal component/system/schema lists.
- [ ] Target files: under 350 lines unless schema-heavy.

## Phase 8: Open Questions Cleanup

**Scope:** all Canon docs.

- [ ] Classify each Open Question as:
  - `Requirement question`;
  - `Implementation parameter`;
  - `Research/future`;
  - `Already decided`.
- [ ] Remove `Already decided` questions.
- [ ] Move `Research/future` to `docs/research/` if it is not a Canon blocker.
- [ ] Keep `Implementation parameter` only if it blocks future design.
- [ ] Ensure each remaining Open Question is specific and answerable.

## Phase 9: Glossary and Roadmap Cleanup

**Files:**

- `docs/GLOSSARY.md`
- `docs/ROADMAP.md`
- `docs/README.md`
- `docs/STYLE_GUIDE.md`

**Plan:**

- [ ] Keep `GLOSSARY.md` as definitions only; remove process-like explanations.
- [ ] Keep `ROADMAP.md` as the only status registry.
- [ ] Keep `docs/README.md` as navigation only.
- [ ] Update `STYLE_GUIDE.md` with concise cleanup rules after the actual cleanup, not before.

## Phase 10: Verification

Run after cleanup:

- [ ] `rg -n 'id="' docs` — should return no accidental code block ids.
- [ ] `rg -n 'MVP|перша симуляція|first simulation|першої симуляції' docs README.md` — should remain clean.
- [ ] `rg -n 'базова модель базова модель|Базова модель базової моделі|Recommendation базової моделі|Recommended базова модель' docs` — should return nothing.
- [ ] `Get-ChildItem docs -Recurse -Filter *.md | ... line counts ...` — confirm largest files are materially smaller.
- [ ] Spot-read:
  - `docs/biology/cell.md`
  - `docs/biology/joint.md`
  - `docs/genetics/genome-runtime.md`
  - `docs/config/materials_config.md`
  - `docs/world/tick.md`
- [ ] Confirm no Canon rule from GZ-01..GZ-18 was removed.

## Expected Outcome

After applying this plan:

- Canon docs become shorter and sharper.
- Long examples live in `docs/examples/`.
- Contract docs become authoritative for cross-cutting rules.
- Config docs become schema-like instead of tutorial-like.
- Open Questions become actionable.
- Obsidian link cleanup remains isolated for the next work package.

## Risks

- Over-cleaning may remove nuance from emerging-system rules.
- Moving examples without preserving context may make Canon too abstract.
- Some old files use duplicated text to reinforce project principles; remove repetition only when the remaining rule is still explicit.
- Link paths may remain inconsistent until the dedicated Obsidian link-audit.

