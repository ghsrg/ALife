use crate::bootstrap::manifest::FieldLayerSummary;
use crate::bootstrap::resource_layers::ResourceLayerError;

pub fn constant_field_layer(
    field_id: impl Into<String>,
    value: f32,
) -> Result<FieldLayerSummary, ResourceLayerError> {
    if !value.is_finite() {
        return Err(ResourceLayerError::new("BOOTSTRAP_INVALID_RESOURCE_LAYER"));
    }
    Ok(FieldLayerSummary {
        field_id: field_id.into(),
        min: value,
        max: value,
    })
}
