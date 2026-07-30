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
    pub lineages: MonitorUnavailableSection,
    pub evolution: MonitorUnavailableSection,
    pub analytics: MonitorUnavailableSection,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct MonitorWorldPayload {
    pub population_lifecycle: PopulationLifecyclePayload,
    pub resource_cycle: ResourceCyclePayload,
    pub material_cycle: MonitorUnavailableSection,
    pub energy_flow: MonitorUnavailableSection,
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
    pub behavior_profiles: MonitorUnavailableSection,
    pub size_bins: MonitorUnavailableSection,
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
                "world.material_cycle",
                "world.energy_flow",
                "world.accounting_time",
                "cells.observed_primary_roles",
                "cells.potential_roles",
                "cells.radius_distribution",
                "organisms.behavior_profiles",
                "organisms.size_bins",
                "lineages",
                "evolution",
                "analytics",
            ],
            "Monitor Data Panel contract is available, but several source-backed analytic subsections are not populated yet.",
        ),
        payload: MonitorPayload {
            world: MonitorWorldPayload {
                population_lifecycle: population_lifecycle(snapshot),
                resource_cycle: resource_cycle(snapshot),
                material_cycle: MonitorUnavailableSection::new(
                    "MaterialAccountingProjection",
                    "Source-backed Material accounting locations and flow are not populated yet.",
                ),
                energy_flow: MonitorUnavailableSection::new(
                    "EnergyAccountingProjection",
                    "Source-backed Energy Flow accounting is not populated yet.",
                ),
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
                    "Cell radius histogram is not populated in Monitor payload yet.",
                ),
            },
            organisms: MonitorOrganismsPayload {
                behavior_profiles: MonitorUnavailableSection::new(
                    "BehaviorProfileProjection",
                    "Observed Behavior Profiles require typed Observer behavior profile rows.",
                ),
                size_bins: MonitorUnavailableSection::new(
                    "OrganismViewProjection",
                    "Organism size bins require source-backed organism membership.",
                ),
            },
            lineages: MonitorUnavailableSection::new(
                "LineageProjection",
                "Lineage Data Panel summaries are not populated yet.",
            ),
            evolution: MonitorUnavailableSection::new(
                "GenomeProjection",
                "Genome and evolution summaries are not populated yet.",
            ),
            analytics: MonitorUnavailableSection::new(
                "MetricsProjection",
                "No selected source-backed Analytics metric is available.",
            ),
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
