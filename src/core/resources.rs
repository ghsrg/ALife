use crate::core::units::ResourceAmount;

#[derive(Clone, Debug, PartialEq)]
pub struct ResourceGrid {
    layers: Vec<ResourceAmount>,
}

impl ResourceGrid {
    pub fn phase1_placeholder(initial: ResourceAmount) -> Self {
        Self {
            layers: vec![initial],
        }
    }

    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }
}
