use serde::Deserialize;
use std::fmt;

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct BootstrapGeneratorSpec {
    pub family: Option<String>,
    #[serde(default)]
    pub resources: Vec<ResourceGeneratorSpec>,
    #[serde(default)]
    pub fields: Vec<FieldGeneratorSpec>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ResourceGeneratorSpec {
    pub resource_type_id: String,
    pub generator: String,
    pub version: String,
    pub seed_domain: String,
    pub patches: Option<usize>,
    pub min_amount: Option<f32>,
    pub max_amount: Option<f32>,
    pub falloff: Option<f32>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct FieldGeneratorSpec {
    pub field_id: String,
    pub generator: String,
    pub version: String,
    pub seed_domain: String,
    pub min_value: Option<f32>,
    pub max_value: Option<f32>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BootstrapGeneratorSpecError {
    code: &'static str,
}

impl BootstrapGeneratorSpecError {
    pub const fn new(code: &'static str) -> Self {
        Self { code }
    }

    pub const fn code(&self) -> &'static str {
        self.code
    }
}

impl fmt::Display for BootstrapGeneratorSpecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code)
    }
}

impl std::error::Error for BootstrapGeneratorSpecError {}

impl BootstrapGeneratorSpec {
    pub fn validate(&self) -> Result<(), BootstrapGeneratorSpecError> {
        for resource in &self.resources {
            resource.validate()?;
        }
        for field in &self.fields {
            field.validate()?;
        }
        Ok(())
    }
}

impl ResourceGeneratorSpec {
    pub fn validate(&self) -> Result<(), BootstrapGeneratorSpecError> {
        if self.resource_type_id.trim().is_empty()
            || self.version.trim().is_empty()
            || self.seed_domain.trim().is_empty()
        {
            return Err(BootstrapGeneratorSpecError::new(
                "BOOTSTRAP_INVALID_GENERATOR_SPEC",
            ));
        }
        if self.generator != "patches" && self.generator != "gradient" {
            return Err(BootstrapGeneratorSpecError::new(
                "BOOTSTRAP_UNKNOWN_RESOURCE_GENERATOR",
            ));
        }
        if let Some(patches) = self.patches
            && patches == 0
        {
            return Err(BootstrapGeneratorSpecError::new(
                "BOOTSTRAP_INVALID_GENERATOR_SPEC",
            ));
        }
        if invalid_optional_non_negative(self.min_amount)
            || invalid_optional_non_negative(self.max_amount)
            || invalid_optional_range(self.min_amount, self.max_amount)
            || invalid_optional_unit_interval(self.falloff)
        {
            return Err(BootstrapGeneratorSpecError::new(
                "BOOTSTRAP_INVALID_GENERATOR_SPEC",
            ));
        }
        Ok(())
    }
}

impl FieldGeneratorSpec {
    pub fn validate(&self) -> Result<(), BootstrapGeneratorSpecError> {
        if self.field_id.trim().is_empty()
            || self.version.trim().is_empty()
            || self.seed_domain.trim().is_empty()
        {
            return Err(BootstrapGeneratorSpecError::new(
                "BOOTSTRAP_INVALID_GENERATOR_SPEC",
            ));
        }
        if self.generator != "band" && self.generator != "gradient" {
            return Err(BootstrapGeneratorSpecError::new(
                "BOOTSTRAP_UNKNOWN_FIELD_GENERATOR",
            ));
        }
        if invalid_optional_finite(self.min_value)
            || invalid_optional_finite(self.max_value)
            || invalid_optional_range(self.min_value, self.max_value)
        {
            return Err(BootstrapGeneratorSpecError::new(
                "BOOTSTRAP_INVALID_GENERATOR_SPEC",
            ));
        }
        Ok(())
    }
}

fn invalid_optional_finite(value: Option<f32>) -> bool {
    value.is_some_and(|value| !value.is_finite())
}

fn invalid_optional_non_negative(value: Option<f32>) -> bool {
    value.is_some_and(|value| !value.is_finite() || value < 0.0)
}

fn invalid_optional_unit_interval(value: Option<f32>) -> bool {
    value.is_some_and(|value| !value.is_finite() || !(0.0..=1.0).contains(&value))
}

fn invalid_optional_range(min: Option<f32>, max: Option<f32>) -> bool {
    matches!((min, max), (Some(min), Some(max)) if max < min)
}
