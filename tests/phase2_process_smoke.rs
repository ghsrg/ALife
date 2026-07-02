use alife::core::process::{MaterialCapability, MaterialCapabilityFlags};

#[test]
fn material_capabilities_flags_work() {
    let flags = MaterialCapabilityFlags {
        boundary_permeability: true,
        resource_uptake: true,
        metabolism: false,
        structural_growth: false,
        storage_capacity: true,
        repair: false,
    };
    assert!(flags.has(MaterialCapability::BoundaryPermeability));
    assert!(!flags.has(MaterialCapability::Metabolism));
}

#[test]
fn cell_inventory_queries_capabilities_based_on_material_amounts() {
    use alife::core::cell_store::{CellStore, EnergyBuffer, InitialCellState};
    use alife::core::units::{
        CapacityAmount, EnergyAmount, MaterialAmount, Position, Radius, ResourceAmount, Temperature,
    };

    let mut cells = CellStore::with_capacity(1);
    cells.insert_initial(InitialCellState {
        position: Position::new(1.0, 1.0),
        radius: Radius::new(1.0).unwrap(),
        energy: EnergyBuffer::new(
            EnergyAmount::new(5.0).unwrap(),
            EnergyAmount::new(10.0).unwrap(),
        ),
        resources: ResourceAmount::zero(),
        materials: MaterialAmount::new(5.0).unwrap(),
        capacity_limit: CapacityAmount::new(10.0).unwrap(),
        temperature: Temperature::new(25.0),
    });

    let idx = alife::core::cell_store::CellIndex::from_raw(0);
    assert!(cells.has_capability(idx, MaterialCapability::Metabolism));
}
