# Observer Classification & Behavior Profile Balance TDD Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Реалізувати розширювану та конфігуровану Observer-архітектуру для класифікації ролей клітин, профілів поведінки та архетипів колоній на основі завантажуваних TOML конфігурацій з `docs/config/observer/` без використання хардкоду в Rust-логіці.

**Architecture:**
Observer відокремлюється від симуляційного Core в окремий модуль `src/observer/`. Дані проходять наступний ланцюжок:
1. **Committed State & Event Logs** → перетворюються на **`ObservationWindow`** (містить нормалізовану мапу `features`).
2. **`ObservationWindow`** → передається у три класифікатори (Cell Role, Behavior Profile, Organism Archetype).
3. **Класифікатори** завантажують правила (clauses) з TOML і виконують generic-перевірку, повертаючи **`ClassificationResult`** (детальний звіт із доказовою базою, впевненістю, версією та статусом).
4. **Equal Requirements Check** порівнює сценарій та умови середовища (контрольовані умови), і у разі збігу генерує збалансовані висновки (Findings) з класифікаціями результатів: `TradeoffObserved`, `PossibleAdvantage` тощо.

---

### File Structure Map

- `src/observer/mod.rs` — головний модуль спостерігача.
- `src/observer/config.rs` — парсинг TOML-конфігурацій (clauses, registry).
- `src/observer/projection.rs` — визначення `ObservationWindow` та вилучення features.
- `src/observer/classifiers.rs` — реалізація generic-класифікаторів (Potential/Observed Cell Roles, Behavior, Archetypes).
- `src/observer/balance.rs` — оцінка умов рівності та порівняння профілів.

---

### Task 1: Observer config parsers

**Files:**
- Create: `src/observer/mod.rs`
- Create: `src/observer/config.rs`
- Modify: `src/lib.rs` (експорт `pub mod observer;`)
- Create: `tests/phase2_observer_config.rs`

- [ ] **Step 1: Write the failing test**

Створити `tests/phase2_observer_config.rs`:
```rust
use alife::observer::config::{
    load_classification_registry, load_cell_role_classifier,
    load_behavior_profile_classifier, load_organism_archetype_classifier
};

#[test]
fn test_load_all_observer_configs() {
    let reg = load_classification_registry("docs/config/observer/classification-registry.toml").unwrap();
    assert_eq!(reg.registry.id, "observer-classification-registry");
    assert!(reg.dimensions.contains_key("cell-functional-role"));

    let role_cfg = load_cell_role_classifier("docs/config/observer/cell-functional-role-classifier.toml").unwrap();
    assert_eq!(role_cfg.rules.get("boundary-supporting-like").unwrap().min_fraction, 0.20);

    let behavior_cfg = load_behavior_profile_classifier("docs/config/observer/behavior-profile-classifier.toml").unwrap();
    let dormancy_profile = behavior_cfg.profiles.get("dormancy-oriented-like").unwrap();
    assert_eq!(dormancy_profile.clauses[0].feature, "dormant_fraction");
    assert_eq!(dormancy_profile.clauses[0].operator, ">=");
    assert_eq!(dormancy_profile.clauses[0].value, 0.80);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test phase2_observer_config`
Expected: Compile error: `observer` module not found.

- [ ] **Step 3: Write minimal implementation**

Додати `pub mod observer;` у `src/lib.rs`.

Створити `src/observer/mod.rs`:
```rust
pub mod config;
pub mod projection;
pub mod classifiers;
pub mod balance;
```

Створити `src/observer/config.rs`:
```rust
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct RegistryMeta {
    pub id: String,
    pub version: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DimensionDef {
    pub name: String,
    pub priority: u32,
    pub enabled: bool,
    pub labels: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ClassificationRegistry {
    pub registry: RegistryMeta,
    pub dimensions: HashMap<String, DimensionDef>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RoleRule {
    pub required_material: String,
    pub min_fraction: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CellRoleClassifierConfig {
    pub version: String,
    pub rules: HashMap<String, RoleRule>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RuleClause {
    pub feature: String,
    pub operator: String,
    pub value: f32,
    pub weight: Option<f32>,
    pub required: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProfileRule {
    pub clauses: Vec<RuleClause>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BehaviorClassifierConfig {
    pub version: String,
    pub profiles: HashMap<String, ProfileRule>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ArchetypeRule {
    pub clauses: Vec<RuleClause>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OrganismArchetypeClassifierConfig {
    pub version: String,
    pub archetypes: HashMap<String, ArchetypeRule>,
}

pub fn load_classification_registry<P: AsRef<Path>>(path: P) -> Result<ClassificationRegistry, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let parsed: ClassificationRegistry = toml::from_str(&content)?;
    Ok(parsed)
}

pub fn load_cell_role_classifier<P: AsRef<Path>>(path: P) -> Result<CellRoleClassifierConfig, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let parsed: CellRoleClassifierConfig = toml::from_str(&content)?;
    Ok(parsed)
}

pub fn load_behavior_profile_classifier<P: AsRef<Path>>(path: P) -> Result<BehaviorClassifierConfig, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let parsed: BehaviorClassifierConfig = toml::from_str(&content)?;
    Ok(parsed)
}

pub fn load_organism_archetype_classifier<P: AsRef<Path>>(path: P) -> Result<OrganismArchetypeClassifierConfig, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let parsed: OrganismArchetypeClassifierConfig = toml::from_str(&content)?;
    Ok(parsed)
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test phase2_observer_config`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/lib.rs src/observer/mod.rs src/observer/config.rs tests/phase2_observer_config.rs
git commit -m "feat: implement config TOML loaders for generic observer rules"
```

---

### Task 2: Observation Window and Feature Extraction

**Files:**
- Create: `src/observer/projection.rs`
- Create: `tests/phase2_observer_projection.rs`

- [ ] **Step 1: Write the failing test**

Створити `tests/phase2_observer_projection.rs`:
```rust
use alife::observer::projection::{ObservationWindow, EntityType, extract_features};
use std::collections::HashMap;

#[test]
fn test_feature_extraction_from_raw_metrics() {
    let mut raw_data = HashMap::new();
    raw_data.insert("dormant_ticks".to_string(), 80.0);
    raw_data.insert("ticks_executed".to_string(), 100.0);
    raw_data.insert("boundary_material".to_string(), 30.0);
    raw_data.insert("total_materials".to_string(), 100.0);

    let window = extract_features("run-123", EntityType::Cell, "cell-0", 0, 100, &raw_data);
    assert_eq!(window.run_id, "run-123");
    assert_eq!(*window.features.get("dormant_fraction").unwrap(), 0.80);
    assert_eq!(*window.features.get("boundary_material_fraction").unwrap(), 0.30);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test phase2_observer_projection`
Expected: Compile error: `extract_features` or types not found.

- [ ] **Step 3: Write minimal implementation**

Створити `src/observer/projection.rs`:
```rust
use std::collections::HashMap;
use serde::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub enum EntityType {
    Cell,
    Organism,
    Population,
}

#[derive(Debug, Serialize, Clone)]
pub struct ObservationWindow {
    pub run_id: String,
    pub entity_type: EntityType,
    pub entity_id: String,
    pub tick_start: u64,
    pub tick_end: u64,
    pub features: HashMap<String, f32>,
    pub data_completeness: f32,
    pub projection_version: String,
}

pub fn extract_features(
    run_id: &str,
    entity_type: EntityType,
    entity_id: &str,
    tick_start: u64,
    tick_end: u64,
    raw_data: &HashMap<String, f32>,
) -> ObservationWindow {
    let mut features = HashMap::new();

    // Extract dormant fraction
    if let (Some(&d), Some(&t)) = (raw_data.get("dormant_ticks"), raw_data.get("ticks_executed")) {
        features.insert("dormant_fraction".to_string(), if t > 0.0 { d / t } else { 0.0 });
    }

    // Extract material fractions
    if let Some(&total_mat) = raw_data.get("total_materials") {
        if total_mat > 0.0 {
            for mat_name in &[
                "boundary_material", "transport_material", "metabolic_material",
                "storage_material", "synthesis_material", "structural_material",
                "repair_material", "contractile_material", "sensory_material"
            ] {
                if let Some(&val) = raw_data.get(*mat_name) {
                    features.insert(format!("{}_fraction", mat_name), val / total_mat);
                }
            }
        }
    }

    // Copy other features directly
    for (k, &v) in raw_data {
        if !features.contains_key(k) {
            features.insert(k.clone(), v);
        }
    }

    ObservationWindow {
        run_id: run_id.to_string(),
        entity_type,
        entity_id: entity_id.to_string(),
        tick_start,
        tick_end,
        features,
        data_completeness: 1.0,
        projection_version: "1.0.0".to_string(),
    }
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test phase2_observer_projection`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/observer/projection.rs tests/phase2_observer_projection.rs
git commit -m "feat: add ObservationWindow and feature extraction layer"
```

---

### Task 3: Generic Rule Clause Evaluator & Classification Outputs

**Files:**
- Create: `src/observer/classifiers.rs`
- Create: `tests/phase2_observer_classifiers.rs`

- [ ] **Step 1: Write the failing test**

Створити `tests/phase2_observer_classifiers.rs`:
```rust
use alife::observer::config::{load_cell_role_classifier, load_behavior_profile_classifier};
use alife::observer::projection::{extract_features, EntityType};
use alife::observer::classifiers::{
    classify_cell_roles_potential, classify_cell_roles_observed,
    classify_behavior_profiles, ClassificationStatus
};
use std::collections::HashMap;

#[test]
fn test_classify_observed_vs_potential_roles() {
    let role_cfg = load_cell_role_classifier("docs/config/observer/cell-functional-role-classifier.toml").unwrap();
    let mut raw_data = HashMap::new();
    raw_data.insert("boundary_material".to_string(), 30.0);
    raw_data.insert("total_materials".to_string(), 100.0);
    raw_data.insert("ActiveUptake_executed".to_string(), 0.0); // no active uptake executed

    let window = extract_features("run-123", EntityType::Cell, "cell-0", 0, 100, &raw_data);

    let pot_res = classify_cell_roles_potential(&window, &role_cfg);
    assert!(pot_res.primary_label.is_some());
    assert_eq!(pot_res.primary_label.unwrap(), "boundary-supporting-like");

    let obs_res = classify_cell_roles_observed(&window, &role_cfg);
    // Should fail observed classification because no actions were executed
    assert_eq!(obs_res.status, ClassificationStatus::Unknown);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test phase2_observer_classifiers`
Expected: Compile error: functions or types not found.

- [ ] **Step 3: Write minimal implementation**

Створити `src/observer/classifiers.rs`:
```rust
use crate::observer::config::{CellRoleClassifierConfig, BehaviorClassifierConfig, RuleClause};
use crate::observer::projection::{ObservationWindow, EntityType};
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, Eq, Clone)]
pub enum ClassificationMode {
    Potential,
    Observed,
}

#[derive(Debug, Serialize, PartialEq, Eq, Clone)]
pub enum ClassificationStatus {
    Classified,
    Mixed,
    Unknown,
    InsufficientData,
    Unstable,
}

#[derive(Debug, Serialize, Clone)]
pub struct LabelResult {
    pub label: String,
    pub confidence: f32,
}

#[derive(Debug, Serialize, Clone)]
pub struct EvidenceRecord {
    pub feature: String,
    pub expected: String,
    pub actual: f32,
    pub matched: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct ClassificationResult {
    pub dimension_id: String,
    pub entity_id: String,
    pub mode: ClassificationMode,
    pub primary_label: Option<String>,
    pub secondary_labels: Vec<LabelResult>,
    pub status: ClassificationStatus,
    pub confidence: f32,
    pub tick_start: u64,
    pub tick_end: u64,
    pub classifier_version: String,
    pub evidence: Vec<EvidenceRecord>,
    pub data_completeness: f32,
}

fn evaluate_clause(clause: &RuleClause, window: &ObservationWindow) -> EvidenceRecord {
    let actual = window.features.get(&clause.feature).copied().unwrap_or(0.0);
    let matched = match clause.operator.as_str() {
        ">=" => actual >= clause.value,
        "<=" => actual <= clause.value,
        "==" => (actual - clause.value).abs() < 1e-5,
        ">" => actual > clause.value,
        "<" => actual < clause.value,
        _ => false,
    };
    EvidenceRecord {
        feature: clause.feature.clone(),
        expected: format!("{} {}", clause.operator, clause.value),
        actual,
        matched,
    }
}

pub fn classify_cell_roles_potential(
    window: &ObservationWindow,
    config: &CellRoleClassifierConfig,
) -> ClassificationResult {
    let mut primary_label = None;
    let mut evidence = Vec::new();
    let mut max_fraction = 0.0;

    for (role_name, rule) in &config.rules {
        let feature_name = format!("{}_fraction", rule.required_material);
        let fraction = window.features.get(&feature_name).copied().unwrap_or(0.0);
        let matched = fraction >= rule.min_fraction;
        evidence.push(EvidenceRecord {
            feature: feature_name.clone(),
            expected: format!(">= {}", rule.min_fraction),
            actual: fraction,
            matched,
        });

        if matched && fraction > max_fraction {
            max_fraction = fraction;
            primary_label = Some(role_name.clone());
        }
    }

    ClassificationResult {
        dimension_id: "cell-functional-role".to_string(),
        entity_id: window.entity_id.clone(),
        mode: ClassificationMode::Potential,
        primary_label,
        secondary_labels: vec![],
        status: if primary_label.is_some() { ClassificationStatus::Classified } else { ClassificationStatus::Unknown },
        confidence: if primary_label.is_some() { 0.9 } else { 0.0 },
        tick_start: window.tick_start,
        tick_end: window.tick_end,
        classifier_version: config.version.clone(),
        evidence,
        data_completeness: window.data_completeness,
    }
}

pub fn classify_cell_roles_observed(
    window: &ObservationWindow,
    config: &CellRoleClassifierConfig,
) -> ClassificationResult {
    let mut primary_label = None;
    let mut evidence = Vec::new();
    let mut max_fraction = 0.0;

    // For Observed role, check if related action was actually executed
    for (role_name, rule) in &config.rules {
        let feature_name = format!("{}_fraction", rule.required_material);
        let fraction = window.features.get(&feature_name).copied().unwrap_or(0.0);
        
        let action_feature = match rule.required_material.as_str() {
            "boundary_material" => "PassiveUptake_executed",
            "transport_material" => "ActiveUptake_executed",
            "metabolic_material" => "Metabolism_executed",
            "storage_material" => "Storage_executed",
            "synthesis_material" => "MaterialSynthesis_executed",
            "structural_material" => "Growth_executed",
            _ => "unknown_action",
        };
        let executed = window.features.get(action_feature).copied().unwrap_or(0.0);
        let matched = fraction >= rule.min_fraction && executed > 0.0;
        
        evidence.push(EvidenceRecord {
            feature: format!("{}+{}", feature_name, action_feature),
            expected: format!(">= {} and executed > 0", rule.min_fraction),
            actual: fraction,
            matched,
        });

        if matched && fraction > max_fraction {
            max_fraction = fraction;
            primary_label = Some(role_name.clone());
        }
    }

    ClassificationResult {
        dimension_id: "cell-functional-role".to_string(),
        entity_id: window.entity_id.clone(),
        mode: ClassificationMode::Observed,
        primary_label,
        secondary_labels: vec![],
        status: if primary_label.is_some() { ClassificationStatus::Classified } else { ClassificationStatus::Unknown },
        confidence: if primary_label.is_some() { 0.95 } else { 0.0 },
        tick_start: window.tick_start,
        tick_end: window.tick_end,
        classifier_version: config.version.clone(),
        evidence,
        data_completeness: window.data_completeness,
    }
}

pub fn classify_behavior_profiles(
    window: &ObservationWindow,
    config: &BehaviorClassifierConfig,
) -> ClassificationResult {
    let mut matched_profiles = Vec::new();
    let mut all_evidence = Vec::new();

    for (profile_name, rule) in &config.profiles {
        let mut match_count = 0;
        let mut total_clauses = rule.clauses.len();

        for clause in &rule.clauses {
            let ev = evaluate_clause(clause, window);
            if ev.matched {
                match_count += 1;
            }
            all_evidence.push(ev);
        }

        if total_clauses > 0 && match_count == total_clauses {
            matched_profiles.push(profile_name.clone());
        }
    }

    let primary_label = matched_profiles.first().cloned();
    ClassificationResult {
        dimension_id: "behavior-profile".to_string(),
        entity_id: window.entity_id.clone(),
        mode: ClassificationMode::Observed,
        primary_label,
        secondary_labels: matched_profiles.into_iter().map(|p| LabelResult { label: p, confidence: 1.0 }).collect(),
        status: if primary_label.is_some() { ClassificationStatus::Classified } else { ClassificationStatus::Unknown },
        confidence: 0.9,
        tick_start: window.tick_start,
        tick_end: window.tick_end,
        classifier_version: config.version.clone(),
        evidence: all_evidence,
        data_completeness: window.data_completeness,
    }
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test phase2_observer_classifiers`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/observer/classifiers.rs tests/phase2_observer_classifiers.rs
git commit -m "feat: implement generic clause evaluation and potential/observed role division"
```

---

### Task 4: Organism Archetype Graph-Aware Classifier

**Files:**
- Modify: `src/observer/classifiers.rs`
- Create: `tests/phase2_observer_archetypes.rs`

- [ ] **Step 1: Write the failing test**

Створити `tests/phase2_observer_archetypes.rs`:
```rust
use alife::observer::config::load_organism_archetype_classifier;
use alife::observer::projection::{extract_features, EntityType};
use alife::observer::classifiers::classify_organism_archetypes;
use std::collections::HashMap;

#[test]
fn test_classify_organism_archetype_stable_colony_like() {
    let config = load_organism_archetype_classifier("docs/config/observer/organism-archetype-classifier.toml").unwrap();
    let mut raw_data = HashMap::new();
    raw_data.insert("cell_count".to_string(), 4.0);
    raw_data.insert("joint_count".to_string(), 3.0);
    raw_data.insert("connectedness".to_string(), 1.0);
    raw_data.insert("lifetime_ticks".to_string(), 120.0);
    raw_data.insert("joint_persistence".to_string(), 0.85);

    let window = extract_features("run-123", EntityType::Organism, "colony-0", 0, 100, &raw_data);
    let result = classify_organism_archetypes(&window, &config);
    
    assert!(result.primary_label.is_some());
    assert_eq!(result.primary_label.unwrap(), "stable-colony-like");
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test phase2_observer_archetypes`
Expected: Compile error: `classify_organism_archetypes` not found in `classifiers`.

- [ ] **Step 3: Write minimal implementation**

Додати до `src/observer/classifiers.rs`:
```rust
pub fn classify_organism_archetypes(
    window: &ObservationWindow,
    config: &OrganismArchetypeClassifierConfig,
) -> ClassificationResult {
    let mut matched_archetypes = Vec::new();
    let mut all_evidence = Vec::new();

    for (arch_name, rule) in &config.archetypes {
        let mut match_count = 0;
        let total_clauses = rule.clauses.len();

        for clause in &rule.clauses {
            let ev = evaluate_clause(clause, window);
            if ev.matched {
                match_count += 1;
            }
            all_evidence.push(ev);
        }

        if total_clauses > 0 && match_count == total_clauses {
            matched_archetypes.push(arch_name.clone());
        }
    }

    let primary_label = matched_archetypes.first().cloned();
    ClassificationResult {
        dimension_id: "organism-archetype".to_string(),
        entity_id: window.entity_id.clone(),
        mode: ClassificationMode::Observed,
        primary_label,
        secondary_labels: matched_archetypes.into_iter().map(|a| LabelResult { label: a, confidence: 1.0 }).collect(),
        status: if primary_label.is_some() { ClassificationStatus::Classified } else { ClassificationStatus::Unknown },
        confidence: 0.9,
        tick_start: window.tick_start,
        tick_end: window.tick_end,
        classifier_version: config.version.clone(),
        evidence: all_evidence,
        data_completeness: window.data_completeness,
    }
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test phase2_observer_archetypes`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/observer/classifiers.rs tests/phase2_observer_archetypes.rs
git commit -m "feat: implement graph-aware organism archetype classifier using generic clauses"
```

---

### Task 5: Equal Requirements Evaluator & Fingerprint Balance Findings

**Files:**
- Create: `src/observer/balance.rs`
- Create: `tests/phase2_observer_balance.rs`

- [ ] **Step 1: Write the failing test**

Створити `tests/phase2_observer_balance.rs`:
```rust
use alife::observer::balance::{
    check_equal_requirements, evaluate_balance, ControlledConditions, ProfileVariables, BalanceOutcome
};

#[test]
fn test_equal_requirements_under_same_scenario() {
    let cond1 = ControlledConditions {
        scenario_id: "scarcity_survival".to_string(),
        scenario_version: "1.0.0".to_string(),
        ticks_requested: 100,
        seed: 42,
        world_size: (128.0, 128.0),
    };
    let cond2 = cond1.clone();
    assert!(check_equal_requirements(&cond1, &cond2));
}

#[test]
fn test_evaluate_balance_tradeoff_observed() {
    let cond = ControlledConditions {
        scenario_id: "scarcity_survival".to_string(),
        scenario_version: "1.0.0".to_string(),
        ticks_requested: 100,
        seed: 42,
        world_size: (128.0, 128.0),
    };
    // Strategy 1: survival 80 ticks, 0 divisions
    let v1 = ProfileVariables {
        survival_ticks: 80,
        divisions_count: 0,
    };
    // Strategy 2: survival 40 ticks, 3 divisions
    let v2 = ProfileVariables {
        survival_ticks: 40,
        divisions_count: 3,
    };

    let finding = evaluate_balance("dormancy-oriented-like", "opportunistic-growth-like", &cond, &v1, &v2);
    assert_eq!(finding.result, BalanceOutcome::TradeoffObserved);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test phase2_observer_balance`
Expected: Compile error: types not found.

- [ ] **Step 3: Write minimal implementation**

Створити `src/observer/balance.rs`:
```rust
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct ControlledConditions {
    pub scenario_id: String,
    pub scenario_version: String,
    pub ticks_requested: u32,
    pub seed: u64,
    pub world_size: (f32, f32),
}

#[derive(Debug, Clone)]
pub struct ProfileVariables {
    pub survival_ticks: u32,
    pub divisions_count: u32,
}

pub fn check_equal_requirements(c1: &ControlledConditions, c2: &ControlledConditions) -> bool {
    c1.scenario_id == c2.scenario_id
        && c1.scenario_version == c2.scenario_version
        && c1.ticks_requested == c2.ticks_requested
        && c1.seed == c2.seed
        && c1.world_size == c2.world_size
}

#[derive(Debug, Serialize, PartialEq, Eq, Clone)]
pub enum BalanceOutcome {
    NoClearDifference,
    PossibleAdvantage,
    PossibleDisadvantage,
    TradeoffObserved,
    Inconclusive,
    InsufficientCoverage,
}

#[derive(Debug, Serialize, Clone)]
pub struct BalanceFinding {
    pub finding_id: String,
    pub compared_profiles: Vec<String>,
    pub equal_requirements: bool,
    pub result: BalanceOutcome,
    pub evidence_metrics: Vec<String>,
    pub survival_ratio: f32,
    pub affected_scenarios: Vec<String>,
    pub suspected_cause: String,
    pub recommendation: String,
    pub recommended_reruns: Vec<String>,
    pub confidence: f32,
}

pub fn evaluate_balance(
    p1: &str,
    p2: &str,
    cond: &ControlledConditions,
    v1: &ProfileVariables,
    v2: &ProfileVariables,
) -> BalanceFinding {
    let t1 = v1.survival_ticks as f32;
    let t2 = v2.survival_ticks as f32;
    let ratio = if t2 > 0.0 { t1 / t2 } else { 0.0 };

    // Assess tradeoff: High survival but low reproduction vs low survival but high reproduction
    let result = if (v1.survival_ticks > v2.survival_ticks && v1.divisions_count < v2.divisions_count)
        || (v1.survival_ticks < v2.survival_ticks && v1.divisions_count > v2.divisions_count) {
        BalanceOutcome::TradeoffObserved
    } else if ratio > 1.30 {
        BalanceOutcome::PossibleAdvantage
    } else if ratio < 0.70 {
        BalanceOutcome::PossibleDisadvantage
    } else {
        BalanceOutcome::NoClearDifference
    };

    let cause = match result {
        BalanceOutcome::TradeoffObserved => "Trade-off observed between survival extension and reproductive throughput".to_string(),
        BalanceOutcome::PossibleAdvantage => format!("{} exhibits higher survival persistence under scarcity", p1),
        BalanceOutcome::PossibleDisadvantage => format!("{} exhibits lower survival persistence under scarcity", p1),
        _ => "Comparable survival and reproductive metrics observed".to_string(),
    };

    BalanceFinding {
        finding_id: format!("{}-vs-{}", p1, p2),
        compared_profiles: vec![p1.to_string(), p2.to_string()],
        equal_requirements: true,
        result,
        evidence_metrics: vec![
            format!("survival_ratio={:.2}", ratio),
            format!("divisions: {} vs {}", v1.divisions_count, v2.divisions_count)
        ],
        survival_ratio: ratio,
        affected_scenarios: vec![cond.scenario_id.clone()],
        suspected_cause: cause,
        recommendation: "Run diagnostic reruns to confirm multi-environment stability".to_string(),
        recommended_reruns: vec![format!("matrix_{}_vs_{}", p1, p2)],
        confidence: 0.8,
    }
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test phase2_observer_balance`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/observer/balance.rs tests/phase2_observer_balance.rs
git commit -m "feat: implement scenario-fingerprint equal check and tradeoff-aware balance findings"
```

---

### Task 6: Sweep Analyzer CLI Integration & Output Writer

**Files:**
- Modify: `src/bin/sweep_analyzer.rs`
- Create: `tests/phase2_sweep_observer_outputs.rs`

- [ ] **Step 1: Write the failing test**

Створити `tests/phase2_sweep_observer_outputs.rs`:
```rust
use std::path::Path;

#[test]
fn test_sweep_analyzer_generates_observer_files() {
    let output_dir = "outputs_test_observer";
    let _ = std::fs::remove_dir_all(output_dir);

    // Run sweep_analyzer binary with test configuration
    let mut cmd = std::process::Command::new("cargo");
    cmd.args(&["run", "--bin", "sweep_analyzer", "--", "sweep_analyzer.toml"]);
    let status = cmd.status().unwrap();
    assert!(status.success());

    // Verify observer outputs exist
    assert!(Path::new("outputs/raw_data/behavior_profiles.csv").exists());
    assert!(Path::new("outputs/reports").exists());
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test phase2_sweep_observer_outputs`
Expected: FAIL (binary runs but doesn't produce behavior_profiles.csv or behavior-profiles-*.json).

- [ ] **Step 3: Write minimal implementation**

Модифікувати `src/bin/sweep_analyzer.rs`:
1. Завантажити реєстри класифікатора під час запуску.
2. Проектувати `ObservationWindow` для кожної запущеної симуляції.
3. Викликати класифікатори для збору ролей та поведінкових профілів.
4. Записати `outputs/raw_data/behavior_profiles.csv` та звіти у `outputs/reports/behavior-profiles-<timestamp>.json` / `.md` та `outputs/reports/balance-findings-<timestamp>.json` / `.md`.

У `src/bin/sweep_analyzer.rs`:
```rust
use alife::observer::config::{
    load_classification_registry, load_cell_role_classifier,
    load_behavior_profile_classifier, load_organism_archetype_classifier
};
use alife::observer::projection::{extract_features, EntityType};
use alife::observer::classifiers::{classify_cell_roles_potential, classify_cell_roles_observed, classify_behavior_profiles};
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test phase2_sweep_observer_outputs`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/bin/sweep_analyzer.rs tests/phase2_sweep_observer_outputs.rs
git commit -m "feat: integrate observer classification layers and balance evaluator into sweep_analyzer"
```
