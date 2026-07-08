use crate::observer::config::CellRoleClassifierConfig;
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
