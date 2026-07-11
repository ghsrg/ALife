use alife::core::cell_store::{CellIndex, CellStore, EnergyBuffer, InitialCellState};
use alife::core::materials::{MaterialComposition, MaterialSlot};
use alife::core::process::MaterialCapability;
use alife::core::units::{
    CapacityAmount, EnergyAmount, MaterialAmount, Position, Radius, ResourceAmount, Temperature,
};

#[test]
fn material_composition_computes_total_and_fraction_deterministically() {
    let composition = MaterialComposition::from_slots([
        (MaterialSlot::Boundary, MaterialAmount::new(1.0).unwrap()),
        (MaterialSlot::Transport, MaterialAmount::new(2.0).unwrap()),
        (MaterialSlot::Metabolic, MaterialAmount::new(3.0).unwrap()),
        (MaterialSlot::Storage, MaterialAmount::zero()),
        (MaterialSlot::Synthesis, MaterialAmount::zero()),
        (MaterialSlot::Structural, MaterialAmount::zero()),
        (MaterialSlot::Repair, MaterialAmount::zero()),
        (MaterialSlot::Contractile, MaterialAmount::zero()),
        (MaterialSlot::Sensory, MaterialAmount::zero()),
    ]);

    assert_eq!(composition.total().raw(), 6.0);
    assert_eq!(composition.amount(MaterialSlot::Transport).raw(), 2.0);
    assert!((composition.fraction(MaterialSlot::Transport) - (2.0 / 6.0)).abs() < 0.0001);
    assert_eq!(composition.fraction(MaterialSlot::Storage), 0.0);
}

#[test]
fn cell_store_exposes_material_composition_and_capability_level() {
    let (cells, idx) = insert_test_cell_with_materials(4.0, 2.0);

    let composition = cells.material_composition(idx);

    assert_eq!(composition.total().raw(), 6.0);
    assert_eq!(composition.amount(MaterialSlot::Transport).raw(), 4.0);
    assert_eq!(
        cells
            .material_amount_for_slot(idx, MaterialSlot::Metabolic)
            .raw(),
        2.0
    );
    assert_eq!(
        cells.capability_level(idx, MaterialCapability::ResourceUptake),
        4.0
    );
    assert_eq!(
        cells.capability_level(idx, MaterialCapability::Metabolism),
        2.0
    );
    assert!(cells.has_capability(idx, MaterialCapability::ResourceUptake));
}

#[test]
fn zero_material_means_missing_capability() {
    let (cells, idx) = insert_test_cell_with_materials(0.0, 2.0);

    assert_eq!(
        cells.capability_level(idx, MaterialCapability::ResourceUptake),
        0.0
    );
    assert!(!cells.has_capability(idx, MaterialCapability::ResourceUptake));
    assert!(cells.has_capability(idx, MaterialCapability::Metabolism));
}

fn insert_test_cell_with_materials(transport: f32, metabolic: f32) -> (CellStore, CellIndex) {
    let mut cells = CellStore::with_capacity(1);
    let id = cells.insert_initial(InitialCellState {
        position: Position::new(1.0, 1.0),
        radius: Radius::new(1.0).unwrap(),
        energy: EnergyBuffer::new(
            EnergyAmount::new(5.0).unwrap(),
            EnergyAmount::new(10.0).unwrap(),
        ),
        resources: ResourceAmount::zero(),
        boundary_material: MaterialAmount::zero(),
        transport_material: MaterialAmount::new(transport).unwrap(),
        metabolic_material: MaterialAmount::new(metabolic).unwrap(),
        storage_material: MaterialAmount::zero(),
        synthesis_material: MaterialAmount::zero(),
        structural_material: MaterialAmount::zero(),
        repair_material: MaterialAmount::zero(),
        contractile_material: MaterialAmount::zero(),
        sensory_material: MaterialAmount::zero(),
        capacity_limit: CapacityAmount::new(20.0).unwrap(),
        temperature: Temperature::new(25.0),
    });

    let idx = cells.resolve_id_cold(id).unwrap();
    (cells, idx)
}
