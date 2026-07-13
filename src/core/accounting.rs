use crate::core::cell_store::CellIndex;
use crate::core::resources::ResourceLayerIndex;
use crate::core::world::WorldState;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IntegratedAccountingSnapshot {
    pub world_resources: f32,
    pub cell_internal_resources: f32,
    pub cell_materials: f32,
    pub fragment_materials: f32,
    pub joint_materials: f32,
    pub explicit_sinks: f32,
}

impl IntegratedAccountingSnapshot {
    pub fn from_world(world: &WorldState) -> Self {
        let cells = world.cells();
        let mut cell_internal_resources = 0.0_f32;
        let mut cell_materials = 0.0_f32;
        for raw in 0..cells.len() {
            let index = CellIndex::from_raw(raw);
            cell_internal_resources += cells.resource_amount(index).raw();
            cell_materials += cells.total_materials(index).raw();
        }

        let mut world_resources = 0.0_f32;
        for raw in 0..world.resources().layer_count() {
            world_resources += world
                .resources()
                .total_amount_for_layer(ResourceLayerIndex::from_raw(raw))
                .map(|amount| amount.raw())
                .unwrap_or(0.0);
        }

        let fragment_materials = world.fragments().total_amount().raw();
        let joint_materials = world
            .joints()
            .all_ids()
            .map(|id| {
                world
                    .joints()
                    .material_amount(id)
                    .map(|amount| amount.raw())
                    .unwrap_or(0.0)
            })
            .sum();

        Self {
            world_resources,
            cell_internal_resources,
            cell_materials,
            fragment_materials,
            joint_materials,
            explicit_sinks: 0.0,
        }
    }

    pub fn total_matter(self) -> f32 {
        self.world_resources
            + self.cell_internal_resources
            + self.cell_materials
            + self.fragment_materials
            + self.joint_materials
            + self.explicit_sinks
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MatterAccountingDelta {
    pub before_total: f32,
    pub after_total: f32,
    pub unclassified_loss: f32,
    pub unclassified_gain: f32,
}

impl MatterAccountingDelta {
    pub fn between(
        before: IntegratedAccountingSnapshot,
        after: IntegratedAccountingSnapshot,
    ) -> Self {
        let before_total = before.total_matter();
        let after_total = after.total_matter();
        let diff = after_total - before_total;
        Self {
            before_total,
            after_total,
            unclassified_loss: (-diff).max(0.0),
            unclassified_gain: diff.max(0.0),
        }
    }

    pub fn is_clean(self, tolerance: f32) -> bool {
        self.unclassified_loss <= tolerance && self.unclassified_gain <= tolerance
    }
}
