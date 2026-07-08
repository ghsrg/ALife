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

#[derive(Debug, Deserialize)]
struct RawBehaviorConfig {
    pub version: String,
    pub profiles: HashMap<String, HashMap<String, toml::Value>>,
}

#[derive(Debug, Deserialize)]
struct RawArchetypeConfig {
    pub version: String,
    pub archetypes: HashMap<String, HashMap<String, toml::Value>>,
}

fn map_field_to_clause(key: &str, value: &toml::Value) -> Option<RuleClause> {
    let val_f32 = match value {
        toml::Value::Integer(i) => *i as f32,
        toml::Value::Float(f) => *f as f32,
        toml::Value::Boolean(b) => {
            if *b {
                1.0
            } else {
                0.0
            }
        }
        _ => return None,
    };

    let (operator, raw_feature) = if let Some(pref) = key.strip_prefix("min_") {
        (">=", pref)
    } else if let Some(pref) = key.strip_prefix("max_") {
        ("<=", pref)
    } else {
        ("==", key)
    };

    // Normalize raw feature name to match registry / observation window feature name
    let feature = match raw_feature {
        "dormant_ticks_fraction" => "dormant_fraction",
        "component_size" => "cell_count",
        _ => raw_feature,
    };

    Some(RuleClause {
        feature: feature.to_string(),
        operator: operator.to_string(),
        value: val_f32,
        weight: None,
        required: None,
    })
}

fn feature_priority(feature: &str) -> i32 {
    match feature {
        "dormant_fraction" => 0,
        "dormancy_entries" => 1,
        _ => 2,
    }
}

pub fn load_classification_registry<P: AsRef<Path>>(
    path: P,
) -> Result<ClassificationRegistry, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let parsed: ClassificationRegistry = toml::from_str(&content)?;
    Ok(parsed)
}

pub fn load_cell_role_classifier<P: AsRef<Path>>(
    path: P,
) -> Result<CellRoleClassifierConfig, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let raw: CellRoleClassifierConfig = toml::from_str(&content)?;
    Ok(raw)
}

pub fn load_behavior_profile_classifier<P: AsRef<Path>>(
    path: P,
) -> Result<BehaviorClassifierConfig, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let raw: RawBehaviorConfig = toml::from_str(&content)?;

    let mut profiles = HashMap::new();
    for (k, v) in raw.profiles {
        let mut clauses = Vec::new();
        for (field_name, field_val) in v {
            if let Some(clause) = map_field_to_clause(&field_name, &field_val) {
                clauses.push(clause);
            }
        }

        // Sort clauses using priority
        clauses.sort_by(|a, b| {
            let p_a = feature_priority(&a.feature);
            let p_b = feature_priority(&b.feature);
            if p_a != p_b {
                p_a.cmp(&p_b)
            } else {
                a.feature.cmp(&b.feature)
            }
        });

        profiles.insert(k, ProfileRule { clauses });
    }

    Ok(BehaviorClassifierConfig {
        version: raw.version,
        profiles,
    })
}

pub fn load_organism_archetype_classifier<P: AsRef<Path>>(
    path: P,
) -> Result<OrganismArchetypeClassifierConfig, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let raw: RawArchetypeConfig = toml::from_str(&content)?;

    let mut archetypes = HashMap::new();
    for (k, v) in raw.archetypes {
        let mut clauses = Vec::new();
        for (field_name, field_val) in v {
            if let Some(clause) = map_field_to_clause(&field_name, &field_val) {
                clauses.push(clause);
            }
        }

        // Sort clauses alphabetically by feature name
        clauses.sort_by(|a, b| a.feature.cmp(&b.feature));

        archetypes.insert(k, ArchetypeRule { clauses });
    }

    Ok(OrganismArchetypeClassifierConfig {
        version: raw.version,
        archetypes,
    })
}
