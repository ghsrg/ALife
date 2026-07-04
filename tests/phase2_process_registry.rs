use alife::core::process::{MaterialCapability, ProcessId, ProcessSpec, ProcessStatus};

#[test]
fn test_every_process_id_has_registry_entry() {
    let all_ids = [
        ProcessId::MandatoryUpkeep,
        ProcessId::LocalResourceUptake,
        ProcessId::MetabolismEnergyConversion,
        ProcessId::MaterialSynthesis,
        ProcessId::GrowthResourceAllocation,
        ProcessId::Division,
        ProcessId::ContractileDisplacement,
    ];
    for id in all_ids {
        let spec = ProcessSpec::for_id(id);
        assert_eq!(spec.process_id, id, "Missing registry entry for {:?}", id);
    }
}

#[test]
fn test_division_is_future_status() {
    assert_eq!(
        ProcessSpec::for_id(ProcessId::Division).status,
        ProcessStatus::Future,
        "Division must be Future until Phase 2D"
    );
}

#[test]
fn test_uptake_requires_resource_uptake_capability() {
    assert!(
        ProcessSpec::for_id(ProcessId::LocalResourceUptake)
            .required_capabilities
            .contains(&MaterialCapability::ResourceUptake),
        "LocalResourceUptake must declare ResourceUptake capability requirement"
    );
}
