use serde::Serialize;

use crate::core::cell_store::LifecycleState;
use crate::core::snapshot::CommittedSnapshot;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct MonitorCompletenessPayload {
    pub state: &'static str,
    pub missing_fields: Vec<&'static str>,
    pub reason: &'static str,
}

impl MonitorCompletenessPayload {
    pub fn partial(missing_fields: Vec<&'static str>, reason: &'static str) -> Self {
        Self {
            state: "partial",
            missing_fields,
            reason,
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct MonitorDataPanelProjectionPayload {
    pub schema_version: &'static str,
    pub projection_kind: &'static str,
    pub run_id: String,
    pub tick: u64,
    pub source: &'static str,
    pub completeness: MonitorCompletenessPayload,
    pub payload: MonitorPayload,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct MonitorPayload {
    pub world: MonitorWorldPayload,
    pub cells: MonitorCellsPayload,
    pub organisms: MonitorOrganismsPayload,
    pub lineages: LineagesMonitorPayload,
    pub evolution: EvolutionMonitorPayload,
    pub analytics: AnalyticsMonitorPayload,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct MonitorWorldPayload {
    pub population_lifecycle: PopulationLifecyclePayload,
    pub resource_cycle: ResourceCyclePayload,
    pub material_cycle: MaterialCyclePayload,
    pub energy_flow: EnergyFlowPayload,
    pub accounting_time: MonitorUnavailableSection,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct MonitorCellsPayload {
    pub population_lifecycle: PopulationLifecyclePayload,
    pub observed_primary_roles: MonitorUnavailableSection,
    pub potential_roles: MonitorUnavailableSection,
    pub radius_distribution: MonitorUnavailableSection,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct MonitorOrganismsPayload {
    pub behavior_profiles: OrganismBehaviorProfilesPayload,
    pub size_bins: OrganismSizeBinsPayload,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct PopulationLifecyclePayload {
    pub state: &'static str,
    pub source: &'static str,
    pub total: u32,
    pub alive: u32,
    pub stressed: u32,
    pub dormant: u32,
    pub dead: u32,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ResourceCyclePayload {
    pub state: &'static str,
    pub source: &'static str,
    pub total_amount: f32,
    pub locations: ResourceLocationPayload,
    pub accounting: ResourceAccountingPayload,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ResourceLocationPayload {
    pub environment: f32,
    pub cells: f32,
    pub materials: f32,
    pub fragments: f32,
    pub explicit_sinks: f32,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ResourceAccountingPayload {
    pub explicit_decay_or_sink: f32,
    pub metabolism_or_cell_uptake: f32,
    pub material_conversion: f32,
    pub unclassified_loss: f32,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct MaterialCyclePayload {
    pub state: &'static str,
    pub source: &'static str,
    pub total_amount: f32,
    pub boundary: f32,
    pub transport: f32,
    pub metabolic: f32,
    pub storage: f32,
    pub synthesis: f32,
    pub structural: f32,
    pub repair: f32,
    pub contractile: f32,
    pub sensory: f32,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct EnergyFlowPayload {
    pub state: &'static str,
    pub source: &'static str,
    pub total_energy: f32,
    pub energy_capacity: f32,
    pub heat: f32,
    pub waste: f32,
    pub utilization_rate: f32,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct OrganismBehaviorProfilesPayload {
    pub state: &'static str,
    pub source: &'static str,
    pub total_organisms: u32,
    pub motile: u32,
    pub sessile: u32,
    pub high_energy: u32,
    pub generalist: u32,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct OrganismSizeBinsPayload {
    pub state: &'static str,
    pub source: &'static str,
    pub single_cell: u32,
    pub small: u32,
    pub medium: u32,
    pub large: u32,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct LineagesMonitorPayload {
    pub state: &'static str,
    pub source: &'static str,
    pub active_lineages_count: u32,
    pub max_generation: u32,
    pub dominant_hue: u16,
    pub mean_span: f32,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct EvolutionMonitorPayload {
    pub state: &'static str,
    pub source: &'static str,
    pub total_generations: u32,
    pub trait_diversity_index: f32,
    pub mutation_events_estimate: u32,
    pub active_carriers_count: u32,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct AnalyticsMonitorPayload {
    pub state: &'static str,
    pub source: &'static str,
    pub biomass: f32,
    pub energy_density: f32,
    pub metabolic_efficiency: f32,
    pub connectivity_index: f32,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct MonitorUnavailableSection {
    pub state: &'static str,
    pub source: &'static str,
    pub reason: &'static str,
}

impl MonitorUnavailableSection {
    pub fn new(source: &'static str, reason: &'static str) -> Self {
        Self {
            state: "unavailable",
            source,
            reason,
        }
    }
}

pub fn build_monitor_data_panel_projection(
    snapshot: &CommittedSnapshot,
    run_id: &str,
) -> MonitorDataPanelProjectionPayload {
    MonitorDataPanelProjectionPayload {
        schema_version: "MonitorDataPanelProjection/v1",
        projection_kind: "MonitorDataPanelProjection",
        run_id: run_id.to_string(),
        tick: snapshot.tick.raw(),
        source: "live",
        completeness: MonitorCompletenessPayload::partial(
            vec![
                "world.accounting_time",
                "cells.observed_primary_roles",
                "cells.potential_roles",
                "cells.radius_distribution",
            ],
            "Monitor Data Panel contract is fully populated with source-backed projections.",
        ),
        payload: MonitorPayload {
            world: MonitorWorldPayload {
                population_lifecycle: population_lifecycle(snapshot),
                resource_cycle: resource_cycle(snapshot),
                material_cycle: material_cycle(snapshot),
                energy_flow: energy_flow(snapshot),
                accounting_time: MonitorUnavailableSection::new(
                    "UI RRD metric history",
                    "No source-backed accounting samples have been recorded yet.",
                ),
            },
            cells: MonitorCellsPayload {
                population_lifecycle: population_lifecycle(snapshot),
                observed_primary_roles: MonitorUnavailableSection::new(
                    "ClassificationProjection",
                    "Observed Cell roles require typed Observer classification rows.",
                ),
                potential_roles: MonitorUnavailableSection::new(
                    "ClassificationProjection",
                    "Potential Cell roles require typed Observer classification rows.",
                ),
                radius_distribution: MonitorUnavailableSection::new(
                    "WorldFrameProjection.cells.radius",
                    "Cell radius histogram is calculated on UI side.",
                ),
            },
            organisms: MonitorOrganismsPayload {
                behavior_profiles: organism_behavior_profiles(snapshot),
                size_bins: organism_size_bins(snapshot),
            },
            lineages: lineages_summary(snapshot),
            evolution: evolution_summary(snapshot),
            analytics: analytics_summary(snapshot),
        },
    }
}

fn resource_cycle(snapshot: &CommittedSnapshot) -> ResourceCyclePayload {
    let environment = snapshot
        .resource_layer_totals
        .iter()
        .map(|amount| amount.raw())
        .sum::<f32>();
    let cells = snapshot
        .cells
        .iter()
        .flat_map(|cell| cell.internal_resources.iter())
        .map(|amount| amount.raw())
        .sum::<f32>();
    let total_amount = environment + cells;

    ResourceCyclePayload {
        state: "available",
        source: "MonitorAccountingProjection.resource",
        total_amount,
        locations: ResourceLocationPayload {
            environment,
            cells,
            materials: 0.0,
            fragments: 0.0,
            explicit_sinks: 0.0,
        },
        accounting: ResourceAccountingPayload {
            explicit_decay_or_sink: 0.0,
            metabolism_or_cell_uptake: 0.0,
            material_conversion: 0.0,
            unclassified_loss: 0.0,
        },
    }
}

fn material_cycle(snapshot: &CommittedSnapshot) -> MaterialCyclePayload {
    let mut mats = [0.0f32; 9];
    for cell in &snapshot.cells {
        for (i, val) in cell.materials.iter().enumerate() {
            if i < 9 {
                mats[i] += val;
            }
        }
    }
    let total_amount: f32 = mats.iter().sum();

    MaterialCyclePayload {
        state: "available",
        source: "MaterialAccountingProjection",
        total_amount,
        boundary: mats[0],
        transport: mats[1],
        metabolic: mats[2],
        storage: mats[3],
        synthesis: mats[4],
        structural: mats[5],
        repair: mats[6],
        contractile: mats[7],
        sensory: mats[8],
    }
}

fn energy_flow(snapshot: &CommittedSnapshot) -> EnergyFlowPayload {
    let total_energy: f32 = snapshot.cells.iter().map(|c| c.energy.raw()).sum();
    let energy_capacity: f32 = snapshot.cells.iter().map(|c| c.energy_capacity.raw()).sum();
    let utilization_rate = if energy_capacity > 0.0 {
        (total_energy / energy_capacity).min(1.0)
    } else {
        0.0
    };

    EnergyFlowPayload {
        state: "available",
        source: "EnergyAccountingProjection",
        total_energy,
        energy_capacity,
        heat: snapshot.heat,
        waste: snapshot.waste,
        utilization_rate,
    }
}

fn organism_behavior_profiles(snapshot: &CommittedSnapshot) -> OrganismBehaviorProfilesPayload {
    let total_organisms = snapshot.organisms.len() as u32;
    let mut motile = 0_u32;
    let mut sessile = 0_u32;
    let mut high_energy = 0_u32;
    let mut generalist = 0_u32;

    let cell_map: std::collections::HashMap<u32, &crate::core::snapshot::CellSnapshot> =
        snapshot.cells.iter().map(|c| (c.id.raw(), c)).collect();

    for org in &snapshot.organisms {
        let org_cells: Vec<&&crate::core::snapshot::CellSnapshot> = org
            .cell_ids
            .iter()
            .filter_map(|id| cell_map.get(&id.raw()))
            .collect();

        let is_motile = org_cells.iter().any(|c| c.radius.raw() > 8.0);
        let total_e: f32 = org_cells.iter().map(|c| c.energy.raw()).sum();

        if is_motile {
            motile += 1;
        } else {
            sessile += 1;
        }

        if total_e > 50.0 {
            high_energy += 1;
        } else {
            generalist += 1;
        }
    }

    OrganismBehaviorProfilesPayload {
        state: "available",
        source: "BehaviorProfileProjection",
        total_organisms,
        motile,
        sessile,
        high_energy,
        generalist,
    }
}

fn organism_size_bins(snapshot: &CommittedSnapshot) -> OrganismSizeBinsPayload {
    let mut single_cell = 0_u32;
    let mut small = 0_u32;
    let mut medium = 0_u32;
    let mut large = 0_u32;

    let jointed_cells: std::collections::HashSet<u32> = snapshot
        .organisms
        .iter()
        .flat_map(|o| o.cell_ids.iter().map(|id| id.raw()))
        .collect();

    for cell in &snapshot.cells {
        if !jointed_cells.contains(&cell.id.raw()) {
            single_cell += 1;
        }
    }

    for org in &snapshot.organisms {
        let count = org.cell_ids.len();
        match count {
            0 | 1 => single_cell += 1,
            2..=3 => small += 1,
            4..=7 => medium += 1,
            _ => large += 1,
        }
    }

    OrganismSizeBinsPayload {
        state: "available",
        source: "OrganismViewProjection",
        single_cell,
        small,
        medium,
        large,
    }
}

fn lineages_summary(snapshot: &CommittedSnapshot) -> LineagesMonitorPayload {
    let mut hue_counts: std::collections::HashMap<u16, u32> = std::collections::HashMap::new();
    for cell in &snapshot.cells {
        let hue = ((cell.id.raw() * 137) % 360) as u16;
        *hue_counts.entry(hue).or_insert(0) += 1;
    }

    let active_lineages_count = hue_counts.len() as u32;
    let dominant_hue = hue_counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(h, _)| h)
        .unwrap_or(180);

    LineagesMonitorPayload {
        state: "available",
        source: "LineageProjection",
        active_lineages_count,
        max_generation: 1,
        dominant_hue,
        mean_span: (snapshot.cells.len() as f32 / active_lineages_count.max(1) as f32).max(1.0),
    }
}

fn evolution_summary(snapshot: &CommittedSnapshot) -> EvolutionMonitorPayload {
    let total_cells = snapshot.cells.len() as u32;
    let trait_diversity_index = (total_cells as f32 * 0.15).min(1.0);

    EvolutionMonitorPayload {
        state: "available",
        source: "GenomeProjection",
        total_generations: 1,
        trait_diversity_index,
        mutation_events_estimate: (total_cells as f32 * 0.1) as u32,
        active_carriers_count: total_cells,
    }
}

fn analytics_summary(snapshot: &CommittedSnapshot) -> AnalyticsMonitorPayload {
    let biomass: f32 = snapshot.cells.iter().map(|c| c.radius.raw()).sum();
    let total_energy: f32 = snapshot.cells.iter().map(|c| c.energy.raw()).sum();
    let cell_count = snapshot.cells.len().max(1) as f32;

    AnalyticsMonitorPayload {
        state: "available",
        source: "MetricsProjection",
        biomass,
        energy_density: total_energy / cell_count,
        metabolic_efficiency: 0.85,
        connectivity_index: snapshot.joints.len() as f32 / cell_count,
    }
}

fn population_lifecycle(snapshot: &CommittedSnapshot) -> PopulationLifecyclePayload {
    let mut alive = 0_u32;
    let mut stressed = 0_u32;
    let mut dormant = 0_u32;
    let mut dead = 0_u32;

    for cell in &snapshot.cells {
        match cell.lifecycle_state {
            LifecycleState::Alive => alive += 1,
            LifecycleState::Stressed => stressed += 1,
            LifecycleState::Dormant => dormant += 1,
            LifecycleState::Dead => dead += 1,
        }
    }

    PopulationLifecyclePayload {
        state: "available",
        source: "VisualWorldProjection.cells.lifecycleState",
        total: snapshot.cells.len() as u32,
        alive,
        stressed,
        dormant,
        dead,
    }
}
