use crate::core::cell_store::{CellIndex, EnergyBuffer, LifecycleState, RuntimeFlags};
use crate::core::config::RuntimeConfig;
use crate::core::deltas::CommitSummary;
use crate::core::events::EventKind;
use crate::core::summary::{CollapseReason, MetricsSummary, RunSummary, SurvivalResult};
use crate::core::units::{EnergyAmount, HeatAmount, WasteAmount};
use crate::core::world::{WorldInitError, WorldState};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TickError {
    WorldInit(WorldInitError),
}

impl From<WorldInitError> for TickError {
    fn from(value: WorldInitError) -> Self {
        Self::WorldInit(value)
    }
}

pub struct TickExecutor {
    world: WorldState,
}

impl TickExecutor {
    pub fn new(config: RuntimeConfig) -> Result<Self, TickError> {
        Ok(Self {
            world: WorldState::from_config(config)?,
        })
    }

    pub fn world(&self) -> &WorldState {
        &self.world
    }

    pub fn step(&mut self) -> Result<RunSummary, TickError> {
        let config = self.world.config().clone();

        let heat_next = HeatAmount::new(
            (self.world.environment().heat().raw()
                + config.environment.heat_generated_per_tick.raw()
                - config.environment.heat_dissipation_rate.raw())
            .max(0.0),
        )
        .expect("heat accounting is clamped");

        let waste_next = WasteAmount::new(
            (self.world.environment().waste().raw()
                + config.environment.waste_generated_per_tick.raw()
                - config.environment.waste_sink_rate.raw())
            .max(0.0),
        )
        .expect("waste accounting is clamped");

        let heat_warning = heat_next.raw() > config.environment.heat_warning_threshold.raw();
        let heat_death = heat_next.raw() > config.environment.heat_death_threshold.raw();
        let waste_warning = waste_next.raw() > config.environment.waste_warning_threshold.raw();
        let waste_death = waste_next.raw() > config.environment.waste_death_threshold.raw();

        let mut final_energy = 0.0;
        let mut overall_lifecycle = LifecycleState::Alive;
        let mut collapse_reason = CollapseReason::None;

        let len = self.world.cells().len();
        for i in 0..len {
            let index = CellIndex::from_raw(i);
            let cell_state_before = self.world.cells().lifecycle_state(index);
            let current = self.world.cells().energy(index);
            let available = current
                .current()
                .saturating_add(config.cell.passive_energy_income);

            let used_capacity = self.world.cells().used_capacity(index);
            let over_capacity = used_capacity.raw() > config.cell.capacity_limit.raw();
            let critical_capacity_exceeded = used_capacity.raw()
                > config.cell.capacity_limit.raw()
                    + config.lifecycle.critical_capacity_overrun.raw();

            let full_cost = config.cell.mandatory_cost_per_tick;
            let dormant_cost = EnergyAmount::new(
                config.cell.mandatory_cost_per_tick.raw()
                    * config.lifecycle.dormant_mandatory_cost_modifier,
            )
            .expect("dormant cost modifier is validated");

            // 1. Wake up check
            let mut current_state = cell_state_before;
            if current_state == LifecycleState::Dormant {
                let can_pay_full = available.raw() >= full_cost.raw();
                let energy_after_full = available.saturating_sub(full_cost);
                let is_stressed_after_full = energy_after_full.raw()
                    < config.lifecycle.stress_energy_threshold.raw()
                    || heat_warning
                    || waste_warning
                    || over_capacity;
                if can_pay_full && !is_stressed_after_full {
                    current_state = LifecycleState::Alive;
                }
            }

            // 2. Determine upkeep cost to pay
            let upkeep_cost = if current_state == LifecycleState::Dormant {
                dormant_cost
            } else {
                full_cost
            };

            let mandatory_paid = available.raw() >= upkeep_cost.raw();
            let mut energy_after = if mandatory_paid {
                available.saturating_sub(upkeep_cost)
            } else {
                available
            };

            // 3. Evaluate death and dormancy transitions
            let mut is_dead = false;
            let mut cell_collapse_reason = CollapseReason::None;
            let mut next_state;

            if energy_after.raw() <= 0.0 {
                is_dead = true;
                cell_collapse_reason = CollapseReason::EnergyDepleted;
                next_state = LifecycleState::Dead;
            } else if critical_capacity_exceeded {
                is_dead = true;
                cell_collapse_reason = CollapseReason::CapacityExceeded;
                next_state = LifecycleState::Dead;
            } else if heat_death {
                is_dead = true;
                cell_collapse_reason = CollapseReason::HeatLimitExceeded;
                next_state = LifecycleState::Dead;
            } else if waste_death {
                is_dead = true;
                cell_collapse_reason = CollapseReason::WasteLimitExceeded;
                next_state = LifecycleState::Dead;
            } else if !mandatory_paid {
                let can_go_dormant = current_state != LifecycleState::Dormant
                    && config.lifecycle.dormancy_allowed
                    && available.raw() >= dormant_cost.raw();
                if can_go_dormant {
                    next_state = LifecycleState::Dormant;
                    energy_after = available.saturating_sub(dormant_cost);
                    if energy_after.raw() <= 0.0 {
                        is_dead = true;
                        cell_collapse_reason = CollapseReason::EnergyDepleted;
                        next_state = LifecycleState::Dead;
                    }
                } else {
                    is_dead = true;
                    cell_collapse_reason = CollapseReason::MandatoryCostUnpaid;
                    next_state = LifecycleState::Dead;
                }
            } else {
                // Determine state for alive cell
                let is_stressed = energy_after.raw()
                    < config.lifecycle.stress_energy_threshold.raw()
                    || heat_warning
                    || waste_warning
                    || over_capacity;
                if is_stressed {
                    next_state = LifecycleState::Stressed;
                } else if current_state == LifecycleState::Dormant {
                    next_state = LifecycleState::Dormant;
                } else {
                    next_state = LifecycleState::Alive;
                }
            }

            // Commit cell state changes
            let energy_after_clamped = energy_after.clamp_max(current.capacity());
            {
                let cells = self.world.cells_mut_for_commit();
                cells.set_energy(
                    index,
                    EnergyBuffer::new(energy_after_clamped, current.capacity()),
                );
                cells.set_lifecycle_state(index, next_state);
                cells.set_runtime_flags(
                    index,
                    RuntimeFlags {
                        mandatory_paid,
                        stalled: !mandatory_paid,
                        over_capacity,
                        inert: is_dead,
                    },
                );
            }

            let current_tick = self.world.tick();
            let cell_id = self.world.cells().id_at(index);
            if !mandatory_paid {
                self.world.events_mut_for_commit().push(
                    current_tick,
                    EventKind::MandatoryCostFailed,
                    Some(cell_id),
                );
            }
            if next_state == LifecycleState::Dead {
                self.world.events_mut_for_commit().push(
                    current_tick,
                    EventKind::CellDead,
                    Some(cell_id),
                );
            }

            final_energy += energy_after_clamped.raw();

            match (overall_lifecycle, next_state) {
                (LifecycleState::Dead, _) | (_, LifecycleState::Dead) => {
                    overall_lifecycle = LifecycleState::Dead;
                }
                (LifecycleState::Stressed, _) | (_, LifecycleState::Stressed) => {
                    overall_lifecycle = LifecycleState::Stressed;
                }
                (LifecycleState::Dormant, _) | (_, LifecycleState::Dormant) => {
                    overall_lifecycle = LifecycleState::Dormant;
                }
                _ => {
                    overall_lifecycle = LifecycleState::Alive;
                }
            }

            if collapse_reason == CollapseReason::None
                && cell_collapse_reason != CollapseReason::None
            {
                collapse_reason = cell_collapse_reason;
            }
        }

        self.world.environment_mut_for_commit().set_heat(heat_next);
        self.world
            .environment_mut_for_commit()
            .set_waste(waste_next);
        self.world
            .resources_mut_for_commit()
            .decay_or_passive_update();
        self.world.advance_tick();

        let current_tick = self.world.tick();
        self.world
            .events_mut_for_commit()
            .push(current_tick, EventKind::TickCommitted, None);

        let survival_result = if overall_lifecycle == LifecycleState::Dead {
            SurvivalResult::Collapse
        } else if overall_lifecycle == LifecycleState::Stressed
            || overall_lifecycle == LifecycleState::Dormant
        {
            SurvivalResult::Fragile
        } else {
            SurvivalResult::Stable
        };

        Ok(RunSummary {
            tick: self.world.tick(),
            config_hash: config.config_hash(),
            survival_result,
            collapse_reason,
            metrics: MetricsSummary {
                final_energy,
                heat: heat_next.raw(),
                waste: waste_next.raw(),
            },
        })
    }

    pub fn run_until_configured_tick(&mut self) -> Result<RunSummary, TickError> {
        let target = self.world.config().world.tick_count.raw();
        let mut latest = self.step()?;
        if latest.survival_result == SurvivalResult::Collapse {
            return Ok(latest);
        }
        while self.world.tick().raw() < target {
            latest = self.step()?;
            if latest.survival_result == SurvivalResult::Collapse {
                break;
            }
        }
        Ok(latest)
    }

    pub fn last_commit_summary(&self) -> CommitSummary {
        CommitSummary {
            ticks_committed: self.world.tick().raw(),
            events_emitted: 0,
        }
    }
}
