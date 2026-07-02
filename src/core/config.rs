use crate::core::units::{
    CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount,
    Seed, Tick, WasteAmount, WorldSize,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WorldConfig {
    pub tick_count: Tick,
    pub seed: Seed,
    pub size: WorldSize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpaceConfig {
    pub spatial_grid_size: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResourceConfig {
    pub initial_distribution: Vec<ResourceAmount>,
    pub optional_decay_rate: f32,
}

impl ResourceConfig {
    pub fn new(
        initial_distribution: Vec<ResourceAmount>,
        optional_decay_rate: f32,
    ) -> Result<Self, ConfigError> {
        if initial_distribution.is_empty() {
            return Err(ConfigError::EmptyResourceDistribution);
        }
        if !optional_decay_rate.is_finite() || !(0.0..=1.0).contains(&optional_decay_rate) {
            return Err(ConfigError::InvalidDecayRate);
        }

        Ok(Self {
            initial_distribution,
            optional_decay_rate,
        })
    }

    pub fn layer_count(&self) -> usize {
        self.initial_distribution.len()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CellInitialConfig {
    pub position: Position,
    pub radius: Radius,
    pub initial_energy: EnergyAmount,
    pub energy_capacity: EnergyAmount,
    pub mandatory_cost_per_tick: EnergyAmount,
    pub passive_energy_income: EnergyAmount,
    pub capacity_limit: CapacityAmount,
    pub initial_resource_amount: ResourceAmount,
    pub initial_material_amount: MaterialAmount,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EnvironmentConfig {
    pub heat_current: HeatAmount,
    pub heat_generated_per_tick: HeatAmount,
    pub heat_dissipation_rate: HeatAmount,
    pub heat_warning_threshold: HeatAmount,
    pub heat_death_threshold: HeatAmount,
    pub waste_current: WasteAmount,
    pub waste_generated_per_tick: WasteAmount,
    pub waste_sink_rate: WasteAmount,
    pub waste_warning_threshold: WasteAmount,
    pub waste_death_threshold: WasteAmount,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LifecycleConfig {
    pub stress_energy_threshold: EnergyAmount,
    pub dormancy_allowed: bool,
    pub dormant_mandatory_cost_modifier: f32,
    pub critical_capacity_overrun: CapacityAmount,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResourceInteractionConfig {
    pub enabled: bool,
    pub uptake_layer_index: usize,
    pub max_uptake_per_tick: ResourceAmount,
    pub metabolism_resource_per_tick: ResourceAmount,
    pub energy_per_resource: f32,
    pub heat_per_resource: f32,
    pub waste_per_resource: f32,
}

impl ResourceInteractionConfig {
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            uptake_layer_index: 0,
            max_uptake_per_tick: ResourceAmount::zero(),
            metabolism_resource_per_tick: ResourceAmount::zero(),
            energy_per_resource: 0.0,
            heat_per_resource: 0.0,
            waste_per_resource: 0.0,
        }
    }

    pub fn validate(self, resources: &ResourceConfig) -> Result<(), ConfigError> {
        if !self.enabled {
            return Ok(());
        }
        if self.uptake_layer_index >= resources.layer_count() {
            return Err(ConfigError::InvalidResourceInteractionLayer);
        }
        for value in [
            self.energy_per_resource,
            self.heat_per_resource,
            self.waste_per_resource,
        ] {
            if !value.is_finite() || value < 0.0 {
                return Err(ConfigError::InvalidResourceInteractionRate);
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RuntimeConfig {
    pub world: WorldConfig,
    pub space: SpaceConfig,
    pub resources: ResourceConfig,
    pub resource_interaction: ResourceInteractionConfig,
    pub cell: CellInitialConfig,
    pub environment: EnvironmentConfig,
    pub lifecycle: LifecycleConfig,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfigError {
    InitialEnergyExceedsCapacity,
    InvalidSpatialGridSize,
    InvalidDormancyModifier,
    InvalidDecayRate,
    EmptyResourceDistribution,
    InvalidResourceInteractionLayer,
    InvalidResourceInteractionRate,
}

impl RuntimeConfig {
    pub fn new(
        world: WorldConfig,
        space: SpaceConfig,
        resources: ResourceConfig,
        resource_interaction: ResourceInteractionConfig,
        cell: CellInitialConfig,
        environment: EnvironmentConfig,
        lifecycle: LifecycleConfig,
    ) -> Result<Self, ConfigError> {
        if cell.initial_energy.raw() > cell.energy_capacity.raw() {
            return Err(ConfigError::InitialEnergyExceedsCapacity);
        }
        if !space.spatial_grid_size.is_finite() || space.spatial_grid_size <= 0.0 {
            return Err(ConfigError::InvalidSpatialGridSize);
        }
        if !lifecycle.dormant_mandatory_cost_modifier.is_finite()
            || lifecycle.dormant_mandatory_cost_modifier < 0.0
            || lifecycle.dormant_mandatory_cost_modifier > 1.0
        {
            return Err(ConfigError::InvalidDormancyModifier);
        }

        resource_interaction.validate(&resources)?;

        Ok(Self {
            world,
            space,
            resources,
            resource_interaction,
            cell,
            environment,
            lifecycle,
        })
    }

    pub fn config_hash(&self) -> u64 {
        let mut hash = 0xcbf29ce484222325_u64;
        for value in [
            self.world.tick_count.raw(),
            self.world.seed.raw(),
            self.cell.initial_energy.raw().to_bits() as u64,
            self.cell.energy_capacity.raw().to_bits() as u64,
            self.cell.mandatory_cost_per_tick.raw().to_bits() as u64,
            self.cell.passive_energy_income.raw().to_bits() as u64,
            self.cell.capacity_limit.raw().to_bits() as u64,
        ] {
            hash ^= value;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        for amount in &self.resources.initial_distribution {
            hash ^= amount.raw().to_bits() as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= self.resources.optional_decay_rate.to_bits() as u64;
        hash = hash.wrapping_mul(0x100000001b3);

        for value in [
            self.resource_interaction.enabled as u64,
            self.resource_interaction.uptake_layer_index as u64,
            self.resource_interaction
                .max_uptake_per_tick
                .raw()
                .to_bits() as u64,
            self.resource_interaction
                .metabolism_resource_per_tick
                .raw()
                .to_bits() as u64,
            self.resource_interaction.energy_per_resource.to_bits() as u64,
            self.resource_interaction.heat_per_resource.to_bits() as u64,
            self.resource_interaction.waste_per_resource.to_bits() as u64,
        ] {
            hash ^= value;
            hash = hash.wrapping_mul(0x100000001b3);
        }

        hash
    }
}
