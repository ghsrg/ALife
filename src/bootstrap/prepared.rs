use crate::bootstrap::manifest::{
    BootstrapManifest, CellSummary, FieldLayerSummary, ResourceLayerSummary, WorldSummary,
};
use crate::core::config::RuntimeConfig;
use crate::runner::scenario_doc::{ScenarioHash, fnv1a64};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PreparedStateHash(u64);

impl PreparedStateHash {
    pub const fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u64 {
        self.0
    }
}

impl fmt::Display for PreparedStateHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "prepared_state_hash_v1:{:016x}", self.0)
    }
}

#[derive(Clone, Debug)]
pub struct PreparedWorld {
    pub runtime_config: RuntimeConfig,
    pub manifest: BootstrapManifest,
    pub prepared_state_hash: PreparedStateHash,
}

pub fn prepared_state_hash_v1(
    scenario_hash: ScenarioHash,
    root_seed: u64,
    world: &WorldSummary,
    cells: &CellSummary,
    resources: &[ResourceLayerSummary],
    fields: &[FieldLayerSummary],
) -> PreparedStateHash {
    let mut source = format!(
        "scenario={}\nseed={}\nworld={:.6},{:.6},{:.6},{}\ncells={},{}\n",
        scenario_hash.raw(),
        root_seed,
        world.width,
        world.height,
        world.spatial_grid_size,
        world.initial_cells,
        cells.initial_cells,
        cells.genome_assigned_cells
    );
    for layer in resources {
        source.push_str(&format!(
            "resource={}, {:.6}, {:.6}, {:.6}\n",
            layer.layer_index, layer.total, layer.min, layer.max
        ));
    }
    for field in fields {
        source.push_str(&format!(
            "field={}, {:.6}, {:.6}\n",
            field.field_id, field.min, field.max
        ));
    }
    PreparedStateHash(fnv1a64(source.as_bytes()))
}
