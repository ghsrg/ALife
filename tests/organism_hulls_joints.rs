use alife::core::cell_store::LifecycleState;
use alife::core::ids::CellId;
use alife::core::snapshot::{CellSnapshot, CommittedSnapshot, JointSnapshot, OrganismSnapshot};
use alife::core::units::{EnergyAmount, Position, Radius, ResourceAmount, Tick};
use alife::observer::projection::build_visual_world_projection;

#[test]
fn test_organism_hulls_and_joints_payload_projection() {
    let cell1 = CellSnapshot {
        id: CellId::from_raw(1),
        position: Position::new(20.0, 20.0),
        radius: Radius::new(8.0).unwrap(),
        energy: EnergyAmount::new(50.0).unwrap(),
        energy_capacity: EnergyAmount::new(100.0).unwrap(),
        lifecycle_state: LifecycleState::Alive,
        materials: [0.0; 9],
        internal_resources: vec![ResourceAmount::new(5.0).unwrap()],
        local_external_resources: vec![],
    };

    let cell2 = CellSnapshot {
        id: CellId::from_raw(2),
        position: Position::new(30.0, 20.0),
        radius: Radius::new(8.0).unwrap(),
        energy: EnergyAmount::new(40.0).unwrap(),
        energy_capacity: EnergyAmount::new(100.0).unwrap(),
        lifecycle_state: LifecycleState::Alive,
        materials: [0.0; 9],
        internal_resources: vec![ResourceAmount::new(3.0).unwrap()],
        local_external_resources: vec![],
    };

    let joint = JointSnapshot {
        id: 1,
        cell1_id: CellId::from_raw(1),
        cell2_id: CellId::from_raw(2),
        rest_length: 10.0,
        pulse_intensity: 0.8,
        signal_speed: 1.0,
    };

    let organism = OrganismSnapshot {
        id: 1,
        cell_ids: vec![CellId::from_raw(1), CellId::from_raw(2)],
        hull_color_hue: 149,
        organic_membrane_tension: 0.75,
    };

    let snapshot = CommittedSnapshot {
        tick: Tick::from_raw(10),
        cells: vec![cell1, cell2],
        joints: vec![joint],
        organisms: vec![organism],
        heat: 0.0,
        waste: 0.0,
        resource_layer_totals: vec![],
        resource_layers: vec![],
    };

    let projection = build_visual_world_projection(&snapshot);

    assert_eq!(projection.cells.len(), 2);
    assert_eq!(projection.joints.len(), 1);
    assert_eq!(projection.joints[0].cell1_id, 1);
    assert_eq!(projection.joints[0].cell2_id, 2);
    assert_eq!(projection.organisms.len(), 1);
    assert_eq!(projection.organisms[0].cell_ids, vec![1, 2]);
}
