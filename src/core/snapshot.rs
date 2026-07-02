use crate::core::cell_store::LifecycleState;
use crate::core::ids::CellId;
use crate::core::resources::ResourceLayerIndex;
use crate::core::units::{EnergyAmount, Position, Radius, ResourceAmount, Tick};
use crate::core::world::WorldState;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CellSnapshot {
    pub id: CellId,
    pub position: Position,
    pub radius: Radius,
    pub energy: EnergyAmount,
    pub lifecycle_state: LifecycleState,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommittedSnapshot {
    pub tick: Tick,
    pub cells: Vec<CellSnapshot>,
    pub heat: f32,
    pub waste: f32,
    pub resource_layer_totals: Vec<ResourceAmount>,
}

impl CommittedSnapshot {
    pub fn from_world(world: &WorldState) -> Self {
        let cells = world
            .cells()
            .iter_indices()
            .map(|index| CellSnapshot {
                id: world.cells().id_at(index),
                position: world.cells().position(index),
                radius: Radius::new(1.0).expect("Phase 1 radius is validated at init"),
                energy: world.cells().energy(index).current(),
                lifecycle_state: world.cells().lifecycle_state(index),
            })
            .collect();

        let resource_layer_totals = (0..world.resources().layer_count())
            .map(|layer| {
                world
                    .resources()
                    .total_amount_for_layer(ResourceLayerIndex::from_raw(layer))
                    .expect("layer range is derived from layer_count")
            })
            .collect();

        Self {
            tick: world.tick(),
            cells,
            heat: world.environment().heat().raw(),
            waste: world.environment().waste().raw(),
            resource_layer_totals,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ViewerCell {
    pub id: CellId,
    pub position: Position,
    pub radius: Radius,
    pub lifecycle_state: LifecycleState,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ViewerFrame {
    pub tick: Tick,
    pub cells: Vec<ViewerCell>,
    pub heat: f32,
    pub waste: f32,
}

impl ViewerFrame {
    pub fn from_snapshot(snapshot: &CommittedSnapshot) -> Self {
        Self {
            tick: snapshot.tick,
            cells: snapshot
                .cells
                .iter()
                .map(|cell| ViewerCell {
                    id: cell.id,
                    position: cell.position,
                    radius: cell.radius,
                    lifecycle_state: cell.lifecycle_state,
                })
                .collect(),
            heat: snapshot.heat,
            waste: snapshot.waste,
        }
    }
}
