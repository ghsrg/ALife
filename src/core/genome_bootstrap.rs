use crate::core::genome::{GenomeId, GenomeOutputValue, GenomeState, GenomeTemplate};

pub fn instantiate_initial_genome(
    world_seed: u64,
    initial_cell_ordinal: usize,
    template: &GenomeTemplate,
) -> GenomeState {
    let outputs = template
        .outputs()
        .iter()
        .map(|(output_id, value)| {
            let noise = deterministic_noise(world_seed, initial_cell_ordinal, output_id.as_str());
            (
                *output_id,
                GenomeOutputValue::new(value.raw() + noise * template.variation_amplitude()),
            )
        })
        .collect();
    GenomeState {
        id: GenomeId::from_raw((initial_cell_ordinal as u32) + 1),
        template_id: template.id().clone(),
        carrier: template.carrier().clone(),
        outputs,
    }
}

fn deterministic_noise(world_seed: u64, initial_cell_ordinal: usize, output_id: &str) -> f32 {
    let mut value = world_seed ^ (initial_cell_ordinal as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
    for byte in output_id.as_bytes() {
        value ^= *byte as u64;
        value = value.wrapping_mul(0xBF58_476D_1CE4_E5B9);
        value ^= value >> 27;
    }
    value ^= value >> 30;
    value = value.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value ^= value >> 27;
    value = value.wrapping_mul(0x94D0_49BB_1331_11EB);
    value ^= value >> 31;
    let sample = (value >> 40) as f32 / (1_u32 << 24) as f32;
    sample * 2.0 - 1.0
}
