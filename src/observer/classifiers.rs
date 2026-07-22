use crate::observer::config::{CellRoleClassifierConfig, OrganismArchetypeClassifierConfig};
use crate::observer::projection::ObservationWindow;
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

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct LabelResult {
    pub label: String,
    pub confidence: f32,
}

#[derive(Debug, Serialize, PartialEq, Clone)]
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

pub fn classify_cell_roles_potential(
    window: &ObservationWindow,
    config: &CellRoleClassifierConfig,
) -> ClassificationResult {
    let mut primary_label = None;
    let mut evidence = Vec::new();
    let mut max_fraction = 0.0;

    let mut sorted_rules: Vec<(&String, &crate::observer::config::RoleRule)> =
        config.rules.iter().collect();
    sorted_rules.sort_by_key(|(name, _)| *name);

    for (role_name, rule) in sorted_rules {
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

    let is_classified = primary_label.is_some();
    ClassificationResult {
        dimension_id: "cell-functional-role".to_string(),
        entity_id: window.entity_id.clone(),
        mode: ClassificationMode::Potential,
        primary_label,
        secondary_labels: vec![],
        status: if is_classified {
            ClassificationStatus::Classified
        } else {
            ClassificationStatus::Unknown
        },
        confidence: if is_classified { 0.9 } else { 0.0 },
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
    let mut evidence = Vec::new();
    let actions = [
        ("boundary-supporting", "PassiveUptake_executed"),
        ("contractile-like", "ContractileDisplacement_executed"),
        ("growth-oriented", "Growth_executed"),
        ("metabolic-like", "Metabolism_executed"),
        ("synthesis-oriented", "MaterialSynthesis_executed"),
        ("transport-like", "ActiveUptake_executed"),
    ];

    let mut candidates = Vec::new();
    for &(role_name, action_name) in &actions {
        let val = window.features.get(action_name).copied().unwrap_or(0.0);
        evidence.push(EvidenceRecord {
            feature: action_name.to_string(),
            expected: "> 0".to_string(),
            actual: val,
            matched: val > 0.0,
        });
        candidates.push((role_name, action_name, val));
    }

    // Sort to find the candidate with the highest non-zero value,
    // breaking ties alphabetically by role name.
    candidates.sort_by(|a, b| match b.2.partial_cmp(&a.2) {
        Some(std::cmp::Ordering::Equal) => a.0.cmp(b.0),
        Some(ord) => ord,
        None => std::cmp::Ordering::Equal,
    });

    let primary_label = if candidates[0].2 > 0.0 {
        Some(candidates[0].0.to_string())
    } else {
        None
    };

    let is_classified = primary_label.is_some();
    ClassificationResult {
        dimension_id: "cell-functional-role".to_string(),
        entity_id: window.entity_id.clone(),
        mode: ClassificationMode::Observed,
        primary_label,
        secondary_labels: vec![],
        status: if is_classified {
            ClassificationStatus::Classified
        } else {
            ClassificationStatus::Unknown
        },
        confidence: if is_classified { 0.95 } else { 0.0 },
        tick_start: window.tick_start,
        tick_end: window.tick_end,
        classifier_version: config.version.clone(),
        evidence,
        data_completeness: window.data_completeness,
    }
}

pub fn evaluate_clause(clause: &crate::observer::config::RuleClause, actual_value: f32) -> bool {
    match clause.operator.as_str() {
        ">=" => actual_value >= clause.value,
        "<=" => actual_value <= clause.value,
        "==" => (actual_value - clause.value).abs() < 1e-5,
        _ => false,
    }
}

pub fn classify_behavior_profiles(
    window: &ObservationWindow,
    config: &crate::observer::config::BehaviorClassifierConfig,
) -> ClassificationResult {
    let mut evidence = Vec::new();
    let mut matched_profiles = Vec::new();

    // Sort profiles alphabetically to ensure deterministic tie-breaking
    let mut sorted_profiles: Vec<(&String, &crate::observer::config::ProfileRule)> =
        config.profiles.iter().collect();
    sorted_profiles.sort_by_key(|(name, _)| *name);

    for (profile_name, rule) in sorted_profiles {
        let mut profile_matched = true;
        for clause in &rule.clauses {
            let actual_value = window.features.get(&clause.feature).copied().unwrap_or(0.0);
            let matched = evaluate_clause(clause, actual_value);

            evidence.push(EvidenceRecord {
                feature: clause.feature.clone(),
                expected: format!("{} {}", clause.operator, clause.value),
                actual: actual_value,
                matched,
            });

            if !matched {
                profile_matched = false;
            }
        }

        if profile_matched {
            matched_profiles.push(profile_name.clone());
        }
    }

    let primary_label = matched_profiles.first().cloned();
    let is_classified = primary_label.is_some();

    ClassificationResult {
        dimension_id: "behavior-profile".to_string(),
        entity_id: window.entity_id.clone(),
        mode: ClassificationMode::Observed,
        primary_label,
        secondary_labels: vec![],
        status: if is_classified {
            ClassificationStatus::Classified
        } else {
            ClassificationStatus::Unknown
        },
        confidence: if is_classified { 0.9 } else { 0.0 },
        tick_start: window.tick_start,
        tick_end: window.tick_end,
        classifier_version: config.version.clone(),
        evidence,
        data_completeness: window.data_completeness,
    }
}

pub fn classify_organism_archetypes(
    window: &ObservationWindow,
    config: &OrganismArchetypeClassifierConfig,
) -> ClassificationResult {
    let mut evidence = Vec::new();
    let mut matched_archetypes = Vec::new();

    // Sort archetypes alphabetically to ensure deterministic tie-breaking
    let mut sorted_archetypes: Vec<(&String, &crate::observer::config::ArchetypeRule)> =
        config.archetypes.iter().collect();
    sorted_archetypes.sort_by_key(|(name, _)| *name);

    for (archetype_name, rule) in sorted_archetypes {
        let mut archetype_matched = true;
        for clause in &rule.clauses {
            let actual_value = window.features.get(&clause.feature).copied().unwrap_or(0.0);
            let matched = evaluate_clause(clause, actual_value);

            evidence.push(EvidenceRecord {
                feature: clause.feature.clone(),
                expected: format!("{} {}", clause.operator, clause.value),
                actual: actual_value,
                matched,
            });

            if !matched {
                archetype_matched = false;
            }
        }

        if archetype_matched {
            matched_archetypes.push(archetype_name.clone());
        }
    }

    let primary_label = matched_archetypes.first().cloned();
    let is_classified = primary_label.is_some();

    ClassificationResult {
        dimension_id: "organism-archetype".to_string(),
        entity_id: window.entity_id.clone(),
        mode: ClassificationMode::Observed,
        primary_label,
        secondary_labels: vec![],
        status: if is_classified {
            ClassificationStatus::Classified
        } else {
            ClassificationStatus::Unknown
        },
        confidence: if is_classified { 0.9 } else { 0.0 },
        tick_start: window.tick_start,
        tick_end: window.tick_end,
        classifier_version: config.version.clone(),
        evidence,
        data_completeness: window.data_completeness,
    }
}
