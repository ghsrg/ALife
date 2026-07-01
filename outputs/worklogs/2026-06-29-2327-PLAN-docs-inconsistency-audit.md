---
tags:
  - alife
  - worklog/plan
---

# Plan: docs inconsistency audit

## Goal

Find and clarify contradictions, mismatches, and non-aligned descriptions in the current documentation.

## Scope

- Review `README.md`, `docs/README.md`, `docs/PRINCIPLES.md`, `docs/GLOSSARY.md`, `docs/ROADMAP.md`, and `docs/STYLE_GUIDE.md`.
- Review all documentation under `docs/world/`, `docs/biology/`, `docs/genetics/`, `docs/evolution/`, `docs/config/`, `docs/engine/`, `docs/research/`, and `docs/decisions/`.
- Produce a report only; do not rewrite Canon documents in this task.

## Method

1. Build an inventory of documentation files and headings.
2. Extract rules, prohibitions, MVP claims, ADR claims, and Open Questions.
3. Compare high-level project rules against detailed documents.
4. Identify contradictions, ambiguous overlaps, outdated references, and missing decision points.
5. Prioritize findings by impact on future implementation.

## Deliverable

- `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-docs-inconsistency-audit.md`

## Out of Scope

- Removing water.
- Refactoring documentation placement.
- Physics/chemistry feasibility audit.
- MVP implementation planning.
