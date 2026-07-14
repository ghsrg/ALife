use crate::bootstrap::manifest::ResourceLayerSummary;
use crate::bootstrap::seed_domains::SplitMix64;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceLayerError {
    code: &'static str,
}

impl ResourceLayerError {
    pub const fn new(code: &'static str) -> Self {
        Self { code }
    }

    pub const fn code(&self) -> &'static str {
        self.code
    }
}

impl fmt::Display for ResourceLayerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code)
    }
}

impl std::error::Error for ResourceLayerError {}

pub fn uniform_resource_layer(
    layer_index: usize,
    cells: usize,
    amount: f32,
) -> Result<ResourceLayerSummary, ResourceLayerError> {
    if !amount.is_finite() || amount < 0.0 {
        return Err(ResourceLayerError::new("BOOTSTRAP_INVALID_RESOURCE_LAYER"));
    }
    Ok(ResourceLayerSummary {
        layer_index,
        total: amount * cells as f32,
        min: amount,
        max: amount,
    })
}

pub fn patches_resource_layer(
    layer_index: usize,
    patches: usize,
    min_amount: f32,
    max_amount: f32,
    rng: &mut SplitMix64,
) -> Result<ResourceLayerSummary, ResourceLayerError> {
    if patches == 0
        || !min_amount.is_finite()
        || !max_amount.is_finite()
        || min_amount < 0.0
        || max_amount < min_amount
    {
        return Err(ResourceLayerError::new("BOOTSTRAP_INVALID_RESOURCE_LAYER"));
    }
    let mut total = 0.0;
    let mut min = f32::INFINITY;
    let mut max = f32::NEG_INFINITY;
    for _ in 0..patches {
        let amount = min_amount + (max_amount - min_amount) * rng.next_f32();
        total += amount;
        min = min.min(amount);
        max = max.max(amount);
    }
    Ok(ResourceLayerSummary {
        layer_index,
        total,
        min,
        max,
    })
}
