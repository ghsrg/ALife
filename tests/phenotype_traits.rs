use alife::core::cell_store::LifecycleState;
use alife::core::ids::CellId;
use alife::core::snapshot::{CellSnapshot, CommittedSnapshot};
use alife::core::units::{EnergyAmount, Position, Radius, ResourceAmount, Tick};
use alife::observer::projection::build_visual_world_projection;

#[test]
fn test_visual_cell_payload_contains_phenotype_traits() {
    let cell = CellSnapshot {
        id: CellId::from_raw(42),
        position: Position::new(10.0, 20.0),
        radius: Radius::new(12.0).unwrap(),
        energy: EnergyAmount::new(50.0).unwrap(),
        energy_capacity: EnergyAmount::new(100.0).unwrap(),
        lifecycle_state: LifecycleState::Alive,
        materials: [0.0; 9],
        internal_resources: vec![ResourceAmount::new(5.0).unwrap()],
        local_external_resources: vec![ResourceAmount::new(2.0).unwrap()],
    };

    let snapshot = CommittedSnapshot {
        tick: Tick::from_raw(10),
        cells: vec![cell],
        joints: vec![],
        organisms: vec![],
        heat: 0.0,
        waste: 0.0,
        resource_layer_totals: vec![],
        resource_layers: vec![],
    };

    let proj = build_visual_world_projection(&snapshot);
    assert_eq!(proj.cells.len(), 1);

    let traits = &proj.cells[0].phenotype_traits;
    assert!(traits.flagella_count > 0);
    assert!(traits.receptor_halo_intensity > 0.0);
    assert_eq!(traits.lineage_hue, ((42 * 137) % 360) as u16);
}
