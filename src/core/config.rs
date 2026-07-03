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
    pub physics_solver_iterations: usize,
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
    pub initial_boundary_material: MaterialAmount,
    pub initial_transport_material: MaterialAmount,
    pub initial_metabolic_material: MaterialAmount,
    pub initial_storage_material: MaterialAmount,
    pub initial_synthesis_material: MaterialAmount,
    pub initial_structural_material: MaterialAmount,
    pub initial_repair_material: MaterialAmount,
    pub initial_contractile_material: MaterialAmount,
    pub initial_sensory_material: MaterialAmount,
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GrowthConfig {
    pub growth_cost_resource: ResourceAmount,
    pub growth_cost_energy: EnergyAmount,
    pub growth_target_radius: Radius,
    pub max_division_pressure: f32,
}

impl Default for GrowthConfig {
    fn default() -> Self {
        Self {
            growth_cost_resource: ResourceAmount::new(2.0).unwrap(),
            growth_cost_energy: EnergyAmount::new(1.0).unwrap(),
            growth_target_radius: Radius::new(2.0).unwrap(),
            max_division_pressure: 0.5,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SynthesisConfig {
    pub cost_resource: ResourceAmount,
    pub cost_energy: EnergyAmount,
}

impl Default for SynthesisConfig {
    fn default() -> Self {
        Self {
            cost_resource: ResourceAmount::new(1.0).unwrap(),
            cost_energy: EnergyAmount::new(5.0).unwrap(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ContractilityConfig {
    pub energy_cost: EnergyAmount,
    pub force_factor: f32,
}

impl Default for ContractilityConfig {
    fn default() -> Self {
        Self {
            energy_cost: EnergyAmount::new(1.0).unwrap(),
            force_factor: 0.1,
        }
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
    pub growth: GrowthConfig,
    pub growth_enabled: bool,
    pub initial_cells: Vec<CellInitialConfig>,
    pub synthesis: SynthesisConfig,
    pub contractility: ContractilityConfig,
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

        let initial_cells = vec![cell];

        Ok(Self {
            world,
            space,
            resources,
            resource_interaction,
            cell,
            environment,
            lifecycle,
            growth: GrowthConfig::default(),
            growth_enabled: false,
            initial_cells,
            synthesis: SynthesisConfig::default(),
            contractility: ContractilityConfig::default(),
        })
    }

    pub fn with_cells(mut self, cells: Vec<CellInitialConfig>) -> Self {
        for cell in &cells {
            assert!(
                cell.initial_energy.raw() <= cell.energy_capacity.raw(),
                "Initial energy exceeds capacity for cell at {:?}",
                cell.position
            );
        }
        if let Some(first) = cells.first() {
            self.cell = *first;
        }
        self.initial_cells = cells;
        self
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
            self.cell.initial_boundary_material.raw().to_bits() as u64,
            self.cell.initial_transport_material.raw().to_bits() as u64,
            self.cell.initial_metabolic_material.raw().to_bits() as u64,
            self.cell.initial_storage_material.raw().to_bits() as u64,
            self.cell.initial_synthesis_material.raw().to_bits() as u64,
            self.cell.initial_structural_material.raw().to_bits() as u64,
            self.cell.initial_repair_material.raw().to_bits() as u64,
            self.cell.initial_contractile_material.raw().to_bits() as u64,
            self.cell.initial_sensory_material.raw().to_bits() as u64,
            self.growth.growth_cost_resource.raw().to_bits() as u64,
            self.growth.growth_cost_energy.raw().to_bits() as u64,
            self.growth.growth_target_radius.raw().to_bits() as u64,
            self.growth.max_division_pressure.to_bits() as u64,
            self.synthesis.cost_resource.raw().to_bits() as u64,
            self.synthesis.cost_energy.raw().to_bits() as u64,
            self.contractility.energy_cost.raw().to_bits() as u64,
            self.contractility.force_factor.to_bits() as u64,
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

        for cell in &self.initial_cells {
            for value in [
                cell.initial_energy.raw().to_bits() as u64,
                cell.energy_capacity.raw().to_bits() as u64,
                cell.mandatory_cost_per_tick.raw().to_bits() as u64,
                cell.passive_energy_income.raw().to_bits() as u64,
                cell.capacity_limit.raw().to_bits() as u64,
                cell.initial_boundary_material.raw().to_bits() as u64,
                cell.initial_transport_material.raw().to_bits() as u64,
                cell.initial_metabolic_material.raw().to_bits() as u64,
                cell.initial_storage_material.raw().to_bits() as u64,
                cell.initial_synthesis_material.raw().to_bits() as u64,
                cell.initial_structural_material.raw().to_bits() as u64,
                cell.initial_repair_material.raw().to_bits() as u64,
                cell.initial_contractile_material.raw().to_bits() as u64,
                cell.initial_sensory_material.raw().to_bits() as u64,
            ] {
                hash ^= value;
                hash = hash.wrapping_mul(0x100000001b3);
            }
        }

        hash
    }
}
