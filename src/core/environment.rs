use crate::core::config::EnvironmentConfig;
use crate::core::units::{HeatAmount, WasteAmount};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EnvironmentState {
    heat: HeatAmount,
    waste: WasteAmount,
}

impl EnvironmentState {
    pub fn from_config(config: &EnvironmentConfig) -> Self {
        Self {
            heat: config.heat_current,
            waste: config.waste_current,
        }
    }

    pub const fn heat(self) -> HeatAmount {
        self.heat
    }

    pub const fn waste(self) -> WasteAmount {
        self.waste
    }

    pub(crate) fn set_heat(&mut self, heat: HeatAmount) {
        self.heat = heat;
    }

    pub(crate) fn set_waste(&mut self, waste: WasteAmount) {
        self.waste = waste;
    }
}
