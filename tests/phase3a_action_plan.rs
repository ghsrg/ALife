use alife::core::action_plan::ActionPlan;
use alife::core::genome::{GenomeOutputId, GenomeOutputValue, GenomeState, GenomeTemplateId};
use alife::core::process::ProcessId;

fn genome(outputs: Vec<(GenomeOutputId, f32)>) -> GenomeState {
    GenomeState {
        id: alife::core::genome::GenomeId::from_raw(1),
        template_id: GenomeTemplateId::new("balanced").unwrap(),
        carrier: alife::core::genome::GenomeCarrierState::new(
            "genome_carrier_A".to_string(),
            1.0,
            1.0,
        )
        .unwrap(),
        outputs: outputs
            .into_iter()
            .map(|(id, value)| (id, GenomeOutputValue::new(value)))
            .collect(),
    }
}

#[test]
fn action_plan_sorts_processes_by_descending_genome_priority() {
    let genome = genome(vec![
        (GenomeOutputId::ResourceUptakePriority, 0.1),
        (GenomeOutputId::EnergyConversionPriority, 0.8),
        (GenomeOutputId::MaterialSynthesisPriority, 0.3),
    ]);

    let plan = ActionPlan::from_genome(Some(&genome));
    assert_eq!(
        plan.ordered_processes(),
        &[
            ProcessId::MetabolismEnergyConversion,
            ProcessId::MaterialSynthesis,
            ProcessId::LocalResourceUptake,
            ProcessId::GenomeRecombination,
            ProcessId::GenomeCopying,
            ProcessId::RepairBoundary,
            ProcessId::ContractileDisplacement,
            ProcessId::GrowthResourceAllocation,
        ]
    );
}

#[test]
fn action_plan_uses_stable_baseline_order_without_genome() {
    let plan = ActionPlan::from_genome(None);
    assert_eq!(
        plan.ordered_processes(),
        &[
            ProcessId::LocalResourceUptake,
            ProcessId::MetabolismEnergyConversion,
            ProcessId::MaterialSynthesis,
            ProcessId::GrowthResourceAllocation,
            ProcessId::ContractileDisplacement,
            ProcessId::RepairBoundary,
        ]
    );
}

#[test]
fn action_plan_includes_genome_copying_priority_candidate() {
    let genome = genome(vec![
        (GenomeOutputId::GenomeCopyingPriority, 1.0),
        (GenomeOutputId::ResourceUptakePriority, 0.1),
    ]);

    let plan = ActionPlan::from_genome(Some(&genome));

    assert_eq!(plan.ordered_processes()[0], ProcessId::GenomeCopying);
}

#[test]
fn action_plan_keeps_stable_tie_break_order() {
    let genome = genome(vec![
        (GenomeOutputId::MaterialSynthesisPriority, 0.5),
        (GenomeOutputId::ResourceUptakePriority, 0.5),
    ]);

    let plan = ActionPlan::from_genome(Some(&genome));
    let order = plan.ordered_processes();
    let uptake = order
        .iter()
        .position(|id| *id == ProcessId::LocalResourceUptake)
        .unwrap();
    let synthesis = order
        .iter()
        .position(|id| *id == ProcessId::MaterialSynthesis)
        .unwrap();

    assert!(
        uptake < synthesis,
        "baseline order must break priority ties"
    );
}
