use crate::bootstrap::generator_spec::BootstrapGeneratorSpecError;
use crate::bootstrap::manifest::WorldFamilySummary;

pub const PATCHY_TEMPERATE_V1: &str = "patchy_temperate_v1";
pub const WORLD_FAMILY_GENERATOR_VERSION: &str = "world_family.patchy_temperate.v1";

pub fn resolve_world_family(
    family: Option<&str>,
) -> Result<Option<WorldFamilySummary>, BootstrapGeneratorSpecError> {
    match family {
        None => Ok(None),
        Some(PATCHY_TEMPERATE_V1) => Ok(Some(WorldFamilySummary {
            family_id: PATCHY_TEMPERATE_V1.to_string(),
            generator_version: WORLD_FAMILY_GENERATOR_VERSION.to_string(),
        })),
        Some(_) => Err(BootstrapGeneratorSpecError::new(
            "BOOTSTRAP_UNKNOWN_WORLD_FAMILY",
        )),
    }
}
