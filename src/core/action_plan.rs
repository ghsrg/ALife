use crate::core::genome::GenomeState;
use crate::core::process::ProcessId;

const PHASE3A_BASELINE: [ProcessId; 6] = [
    ProcessId::LocalResourceUptake,
    ProcessId::MetabolismEnergyConversion,
    ProcessId::MaterialSynthesis,
    ProcessId::GrowthResourceAllocation,
    ProcessId::ContractileDisplacement,
    ProcessId::RepairBoundary,
];

#[derive(Clone, Debug, PartialEq)]
pub struct ActionPlan {
    ordered_processes: Vec<ProcessId>,
}

impl ActionPlan {
    pub fn from_genome(genome: Option<&GenomeState>) -> Self {
        if genome.is_none() {
            return Self {
                ordered_processes: PHASE3A_BASELINE.to_vec(),
            };
        }
        let mut weighted = PHASE3A_BASELINE
            .iter()
            .copied()
            .map(|process| {
                let priority = priority_for_process(genome, process);
                (process, priority, priority.is_some())
            })
            .collect::<Vec<_>>();
        weighted.sort_by(|(left_id, left_priority, left_present), (right_id, right_priority, right_present)| {
            right_priority
                .unwrap_or(0.0)
                .total_cmp(&left_priority.unwrap_or(0.0))
                .then_with(|| right_present.cmp(left_present))
                .then_with(|| {
                    if *left_present && *right_present {
                        left_id
                            .phase3a_baseline_order()
                            .cmp(&right_id.phase3a_baseline_order())
                    } else {
                        right_id
                            .phase3a_baseline_order()
                            .cmp(&left_id.phase3a_baseline_order())
                    }
                })
        });
        Self {
            ordered_processes: weighted.into_iter().map(|(process, _, _)| process).collect(),
        }
    }

    pub fn ordered_processes(&self) -> &[ProcessId] {
        &self.ordered_processes
    }
}

fn priority_for_process(genome: Option<&GenomeState>, process: ProcessId) -> Option<f32> {
    let genome = genome?;
    genome
        .outputs
        .iter()
        .find(|(output_id, _)| output_id.process_id() == process)
        .map(|(_, value)| value.raw())
}
