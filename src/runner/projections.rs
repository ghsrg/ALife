use crate::core::cell_store::LifecycleState;
use crate::core::snapshot::CommittedSnapshot;

#[derive(Clone, Debug, PartialEq)]
pub struct ProjectedCell {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub energy: f32,
    pub lifecycle: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorldFrameProjection {
    pub schema_version: u8,
    pub committed_tick: u64,
    pub heat: f32,
    pub waste: f32,
    pub cells: Vec<ProjectedCell>,
}

impl WorldFrameProjection {
    pub const SCHEMA_VERSION: u8 = 1;

    pub fn from_committed_snapshot(snapshot: &CommittedSnapshot) -> Self {
        Self {
            schema_version: Self::SCHEMA_VERSION,
            committed_tick: snapshot.tick.raw(),
            heat: snapshot.heat,
            waste: snapshot.waste,
            cells: snapshot
                .cells
                .iter()
                .map(|cell| ProjectedCell {
                    id: cell.id.raw(),
                    x: cell.position.x(),
                    y: cell.position.y(),
                    radius: cell.radius.raw(),
                    energy: cell.energy.raw(),
                    lifecycle: lifecycle_state_code(cell.lifecycle_state),
                })
                .collect(),
        }
    }
}

fn lifecycle_state_code(state: LifecycleState) -> u8 {
    match state {
        LifecycleState::Alive => 0,
        LifecycleState::Stressed => 1,
        LifecycleState::Dormant => 2,
        LifecycleState::Dead => 3,
    }
}
