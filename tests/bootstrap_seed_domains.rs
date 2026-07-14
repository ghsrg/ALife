use alife::bootstrap::seed_domains::{
    SeedDomain, SplitMix64, derive_seed_domain, seed_domain_records,
};
use alife::runner::scenario_doc::ScenarioHash;

#[test]
fn same_root_seed_hash_and_domain_produce_same_domain_seed() {
    let hash = ScenarioHash::from_raw(0x1234);
    let a = derive_seed_domain(42, hash, "resources.layers");
    let b = derive_seed_domain(42, hash, "resources.layers");

    assert_eq!(a.domain_seed, b.domain_seed);
    assert_eq!(a.label, "resources.layers");
}

#[test]
fn different_domain_labels_produce_different_domain_seeds() {
    let hash = ScenarioHash::from_raw(0x1234);
    let resources = derive_seed_domain(42, hash, "resources.layers");
    let placement = derive_seed_domain(42, hash, "cells.placement");

    assert_ne!(resources.domain_seed, placement.domain_seed);
}

#[test]
fn independent_rng_streams_keep_other_domains_stable() {
    let hash = ScenarioHash::from_raw(0x1234);
    let placement = derive_seed_domain(42, hash, "cells.placement");
    let before = SplitMix64::new(placement.domain_seed).next_u64();

    let _resource_samples = {
        let resource = derive_seed_domain(99, hash, "resources.layers");
        let mut rng = SplitMix64::new(resource.domain_seed);
        [rng.next_u64(), rng.next_u64(), rng.next_u64()]
    };

    let after = SplitMix64::new(placement.domain_seed).next_u64();
    assert_eq!(before, after);
}

#[test]
fn manifest_seed_domain_records_are_sorted_by_label() {
    let records = seed_domain_records(
        42,
        ScenarioHash::from_raw(0x1234),
        [
            SeedDomain::CellsPlacement,
            SeedDomain::ResourcesLayers,
            SeedDomain::WorldLayout,
        ],
    );

    let labels: Vec<_> = records.iter().map(|record| record.label.as_str()).collect();
    assert_eq!(
        labels,
        ["cells.placement", "resources.layers", "world.layout"]
    );
}
