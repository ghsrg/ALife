# engine-examples.md

> Приклади engine/debug/storage рішень без створення нових правил світу.

---

# Scheduler Trace

```text
Tick 120
1. Environment systems committed resource diffusion.
2. Cell decision systems read stable snapshot.
3. Genome runtime produced action plans.
4. Feasibility rejected 18 actions and allowed 42.
5. Execution committed allowed deltas.
6. Statistics read final state without changing behavior.
```

---

# Feasibility Trace

```text
action_id: repair_boundary
status: rejected
failure_reasons:
  - insufficient_energy
required_energy: 0.4
available_energy: 0.2
```

---

# Runtime Trace

```text
cell_id: 88
energy_level: 0.15
boundary_state: 0.40
outputs:
  produce_energy: 0.90
  repair_boundary: 0.70
  divide: -0.80
```

---

# Snapshot Metadata

```yaml
snapshot:
  tick: 1000
  world_id: "smoke_world"
  seed: 42
  schema_version: 1
```

