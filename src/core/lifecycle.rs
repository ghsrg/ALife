use crate::core::cell_store::LifecycleState;
use crate::core::units::EnergyAmount;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LifecycleReason {
    None,
    EnergyDepleted,
    CapacityExceeded,
    HeatLimitExceeded,
    WasteLimitExceeded,
    Dormancy,
    Stress,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LifecycleDecision {
    pub state: LifecycleState,
    pub reason: LifecycleReason,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LifecycleInput {
    pub mandatory_paid: bool,
    pub energy_after_mandatory: EnergyAmount,
    pub stress_energy_threshold: EnergyAmount,
    pub over_capacity: bool,
    pub critical_capacity_exceeded: bool,
    pub heat_warning: bool,
    pub heat_death: bool,
    pub waste_warning: bool,
    pub waste_death: bool,
    pub dormancy_allowed: bool,
    pub dormant_cost_payable: bool,
}

pub fn evaluate_lifecycle(input: LifecycleInput) -> LifecycleDecision {
    if input.energy_after_mandatory.raw() <= 0.0 {
        return LifecycleDecision {
            state: LifecycleState::Dead,
            reason: LifecycleReason::EnergyDepleted,
        };
    }
    if input.critical_capacity_exceeded {
        return LifecycleDecision {
            state: LifecycleState::Dead,
            reason: LifecycleReason::CapacityExceeded,
        };
    }
    if input.heat_death {
        return LifecycleDecision {
            state: LifecycleState::Dead,
            reason: LifecycleReason::HeatLimitExceeded,
        };
    }
    if input.waste_death {
        return LifecycleDecision {
            state: LifecycleState::Dead,
            reason: LifecycleReason::WasteLimitExceeded,
        };
    }
    if !input.mandatory_paid && input.dormancy_allowed && input.dormant_cost_payable {
        return LifecycleDecision {
            state: LifecycleState::Dormant,
            reason: LifecycleReason::Dormancy,
        };
    }
    if !input.mandatory_paid
        || input.energy_after_mandatory.raw() < input.stress_energy_threshold.raw()
        || input.over_capacity
        || input.heat_warning
        || input.waste_warning
    {
        return LifecycleDecision {
            state: LifecycleState::Stressed,
            reason: LifecycleReason::Stress,
        };
    }

    LifecycleDecision {
        state: LifecycleState::Alive,
        reason: LifecycleReason::None,
    }
}
