# Runner Usage Guide

> Practical commands for running the current ALife Runner.

## 1. Check available scenarios

```bash
cargo run --bin runner -- --list
```

Use this first after pulling new changes. Scenario ids are safer than hardcoded paths.

## 2. Recommended headless demo

Best current demo scenario:

```bash
cargo run --bin runner -- --debug --progress-interval-ms 200 demo_living_world
```

Use this when you want a human-observable terminal demo with a larger 256x256 world, 24 starting cells, abundant `nutrient_A`, low upkeep, division, decomposition, local interaction, joints, Genome bootstrap, and heat/waste handling enabled.

## 3. Short mechanics showcase

```bash
cargo run --bin runner -- --debug --progress-interval-ms 200 world_mechanism_showcase
```

Use this when you want a shorter run with resources, metabolism, division, local interaction, and joints enabled.

## 4. Longer stability run

```bash
cargo run --bin runner -- --debug --progress-interval-ms 500 world_baseline_stable
```

Use this for a longer stability/regression-style run.

## 5. Minimal bootstrap/core smoke

```bash
cargo run --bin runner -- --debug --progress-interval-ms 200 bootstrap_minimal_viable_world
```

Use this to verify Scenario resolution, Bootstrap, PreparedWorld, and Core startup.

This is not a rich world demo.

## 6. Run by file path

Scenario id is preferred, but file path also works:

```bash
cargo run --bin runner -- config/scenarios/demo/demo_living_world.toml
```

With debug progress:

```bash
cargo run --bin runner -- --debug --progress-interval-ms 200 config/scenarios/demo/demo_living_world.toml
```

## 7. Start HTTP service mode

```bash
cargo run --bin runner -- --serve
```

Default server:

```text
http://127.0.0.1:8080
```

## 8. Basic HTTP checks

```bash
curl http://127.0.0.1:8080/server/info
curl http://127.0.0.1:8080/scenarios
curl http://127.0.0.1:8080/run/status
```

## 9. Start a world through HTTP

```bash
curl -X POST http://127.0.0.1:8080/run/start \
  -H "Content-Type: application/json" \
  -d '{"scenario_id":"demo_living_world"}'
```

Then check status:

```bash
curl http://127.0.0.1:8080/run/status
```

## 10. Pause, step, resume, stop

Pause:

```bash
curl -X POST http://127.0.0.1:8080/run/pause
```

Step exactly one committed Tick:

```bash
curl -X POST http://127.0.0.1:8080/run/step \
  -H "Content-Type: application/json" \
  -d '{}'
```

Resume:

```bash
curl -X POST http://127.0.0.1:8080/run/resume
```

Stop:

```bash
curl -X POST http://127.0.0.1:8080/run/stop
```

## 11. Important semantics

`run` mode:

```text
Scenario -> Bootstrap -> Core -> automatic Tick loop -> completion
```

`serve` mode:

```text
Runner starts Ready
Active Run is Idle
UI/API starts the world later
```

`StepRun`:

```text
valid only when Paused
executes exactly one committed Tick
returns to Paused
```

It is a debug command, not "run N ticks".

## 12. Which scenario to use

| Scenario | Use when | Notes |
|---|---|---|
| `demo_living_world` | manual terminal demo | larger and longer-lived; intended for human observation |
| `world_mechanism_showcase` | quick behavior check | shorter mechanics showcase |
| `world_baseline_stable` | longer stability run | more regression-style |
| `bootstrap_minimal_viable_world` | smoke test | minimal, not visually rich |

## 13. Current limitation

Current scenarios are mostly explicit configs. They are useful for smoke, regression, and early demos.

A fully diverse procedural world still needs richer initialization config:

```text
population generation
resource patches
field layers
genome archetype assignment
better spatial projections
```

Until then, use `demo_living_world` as the main terminal demo entry point.
