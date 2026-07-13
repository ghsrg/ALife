use crate::core::units::{HeatAmount, Temperature};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LocalHeat {
    temperature: Temperature,
    capacity: HeatAmount,
    generated: HeatAmount,
}

impl LocalHeat {
    pub const fn new(temperature: Temperature, capacity: HeatAmount) -> Self {
        Self {
            temperature,
            capacity,
            generated: HeatAmount::zero(),
        }
    }
    pub const fn temperature(self) -> Temperature {
        self.temperature
    }
    pub fn add_generated(&mut self, heat: HeatAmount) {
        self.generated = self.generated.saturating_add(heat);
    }
    pub fn commit(&mut self) {
        if self.capacity.raw() > 0.0 {
            self.temperature = Temperature::new(
                self.temperature.raw() + self.generated.raw() / self.capacity.raw(),
            );
        }
        self.generated = HeatAmount::zero();
    }
    pub fn dissipate_toward(&mut self, ambient: Temperature, rate: f32) {
        let bounded = rate.clamp(0.0, 1.0);
        self.temperature = Temperature::new(
            self.temperature.raw() + (ambient.raw() - self.temperature.raw()) * bounded,
        );
    }
}
