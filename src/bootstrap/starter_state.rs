use crate::core::config::CellInitialConfig;
use crate::core::config::RuntimeConfig;
use crate::core::genome::GenomeState;
use crate::core::genome_bootstrap::instantiate_initial_genome;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StarterStateError {
    code: &'static str,
}

impl StarterStateError {
    pub const fn code(&self) -> &'static str {
        self.code
    }
}

impl fmt::Display for StarterStateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code)
    }
}

impl std::error::Error for StarterStateError {}

pub fn starter_energy_range(cells: &[CellInitialConfig]) -> Option<(f32, f32)> {
    let mut iter = cells.iter().map(|cell| cell.initial_energy.raw());
    let first = iter.next()?;
    let mut min = first;
    let mut max = first;
    for value in iter {
        min = min.min(value);
        max = max.max(value);
    }
    Some((min, max))
}

pub fn assign_initial_genomes(
    config: &RuntimeConfig,
) -> Result<Vec<GenomeState>, StarterStateError> {
    let mut genomes = Vec::new();
    for (cell_ordinal, assignment) in config.initial_cell_genome_templates.iter().enumerate() {
        let Some(template_id) = assignment else {
            continue;
        };
        let Some(template) = config
            .genome_templates
            .iter()
            .find(|template| template.id().as_str() == template_id.as_str())
        else {
            return Err(StarterStateError {
                code: "BOOTSTRAP_UNKNOWN_GENOME_TEMPLATE",
            });
        };
        genomes.push(instantiate_initial_genome(
            config.world.seed.raw(),
            cell_ordinal,
            template,
        ));
    }
    Ok(genomes)
}
