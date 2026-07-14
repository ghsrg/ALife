use crate::core::config::RuntimeConfig;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ViabilityStatus {
    Pass,
    Warn,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ViabilityCheck {
    pub code: String,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ViabilityReport {
    pub status: ViabilityStatus,
    pub checks: Vec<ViabilityCheck>,
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ViabilityError {
    pub code: String,
    pub message: String,
}

impl fmt::Display for ViabilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for ViabilityError {}

pub fn validate_prepared_config(config: &RuntimeConfig) -> Result<ViabilityReport, ViabilityError> {
    if config.initial_cells.is_empty() {
        return Err(ViabilityError {
            code: "BOOTSTRAP_NO_INITIAL_CELLS".to_string(),
            message: "Bootstrap requires at least one initial cell".to_string(),
        });
    }

    let mut checks = Vec::new();
    checks.push(ViabilityCheck {
        code: "WORLD_DIMENSIONS_POSITIVE".to_string(),
        passed: config.world.size.width() > 0.0 && config.world.size.height() > 0.0,
    });
    checks.push(ViabilityCheck {
        code: "INITIAL_CELL_COUNT_POSITIVE".to_string(),
        passed: !config.initial_cells.is_empty(),
    });
    checks.push(ViabilityCheck {
        code: "STARTER_ENERGY_WITHIN_CAPACITY".to_string(),
        passed: config
            .initial_cells
            .iter()
            .all(|cell| cell.initial_energy.raw() <= cell.energy_capacity.raw()),
    });
    checks.push(ViabilityCheck {
        code: "CELLS_WITHIN_WORLD_BOUNDS".to_string(),
        passed: config.initial_cells.iter().all(|cell| {
            let radius = cell.radius.raw();
            cell.position.x() - radius >= 0.0
                && cell.position.y() - radius >= 0.0
                && cell.position.x() + radius <= config.world.size.width()
                && cell.position.y() + radius <= config.world.size.height()
        }),
    });

    if let Some(failed) = checks.iter().find(|check| !check.passed) {
        return Err(ViabilityError {
            code: failed.code.clone(),
            message: "prepared world failed structural viability".to_string(),
        });
    }

    let mut warnings = Vec::new();
    if config
        .initial_cells
        .iter()
        .any(|cell| cell.initial_energy.raw() <= config.lifecycle.stress_energy_threshold.raw())
    {
        warnings.push("BOOTSTRAP_LOW_START_ENERGY".to_string());
    }

    Ok(ViabilityReport {
        status: if warnings.is_empty() {
            ViabilityStatus::Pass
        } else {
            ViabilityStatus::Warn
        },
        checks,
        warnings,
    })
}
