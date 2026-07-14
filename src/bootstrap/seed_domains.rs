use crate::runner::scenario_doc::{ScenarioHash, fnv1a64};

pub const BOOTSTRAP_SEED_DOMAIN_VERSION: &str = "seed_domains.v1";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SeedDomain {
    WorldLayout,
    ResourcesLayers,
    FieldsLayers,
    CellsPlacement,
    CellsStarterState,
    GenomeInitialization,
    ViabilityAudit,
}

impl SeedDomain {
    pub const ALL: [Self; 7] = [
        Self::WorldLayout,
        Self::ResourcesLayers,
        Self::FieldsLayers,
        Self::CellsPlacement,
        Self::CellsStarterState,
        Self::GenomeInitialization,
        Self::ViabilityAudit,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::WorldLayout => "world.layout",
            Self::ResourcesLayers => "resources.layers",
            Self::FieldsLayers => "fields.layers",
            Self::CellsPlacement => "cells.placement",
            Self::CellsStarterState => "cells.starter_state",
            Self::GenomeInitialization => "genome.initialization",
            Self::ViabilityAudit => "viability.audit",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SeedDomainRecord {
    pub label: String,
    pub domain_seed: u64,
    pub generator_version: String,
}

pub fn derive_seed_domain(
    root_seed: u64,
    scenario_hash: ScenarioHash,
    label: impl AsRef<str>,
) -> SeedDomainRecord {
    let source = format!(
        "{}\n{}\n{}\n{}",
        BOOTSTRAP_SEED_DOMAIN_VERSION,
        root_seed,
        scenario_hash.raw(),
        label.as_ref()
    );
    SeedDomainRecord {
        label: label.as_ref().to_string(),
        domain_seed: fnv1a64(source.as_bytes()),
        generator_version: BOOTSTRAP_SEED_DOMAIN_VERSION.to_string(),
    }
}

pub fn seed_domain_records<const N: usize>(
    root_seed: u64,
    scenario_hash: ScenarioHash,
    domains: [SeedDomain; N],
) -> Vec<SeedDomainRecord> {
    let mut records = domains
        .into_iter()
        .map(|domain| derive_seed_domain(root_seed, scenario_hash, domain.label()))
        .collect::<Vec<_>>();
    records.sort_by(|a, b| a.label.cmp(&b.label));
    records
}

#[derive(Clone, Copy, Debug)]
pub struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    pub const fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut value = self.state;
        value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        value ^ (value >> 31)
    }

    pub fn next_f32(&mut self) -> f32 {
        let sample = (self.next_u64() >> 40) as f32 / (1_u32 << 24) as f32;
        sample.clamp(0.0, 1.0)
    }
}
