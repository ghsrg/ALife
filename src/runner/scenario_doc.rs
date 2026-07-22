use crate::bootstrap::generator_spec::BootstrapGeneratorSpec;
use crate::core::config::RuntimeConfig;
use crate::runner::config_parser::{ParseError, RawScenarioConfig};
use std::fmt;
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScenarioHash(u64);

impl ScenarioHash {
    pub const fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u64 {
        self.0
    }
}

impl fmt::Display for ScenarioHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "scenario_hash_v1:{:016x}", self.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScenarioSource {
    Path(PathBuf),
    Inline { id: String, content: String },
}

#[derive(Clone, Debug)]
pub struct ScenarioDocument {
    pub id: String,
    pub schema_version: u32,
    pub scenario_hash: ScenarioHash,
    pub runtime_config: RuntimeConfig,
    pub bootstrap_spec: Option<BootstrapGeneratorSpec>,
    pub resource_type_ids: Vec<String>,
    pub canonical_source: String,
}

#[derive(Debug)]
pub enum ScenarioDocumentError {
    Io(std::io::Error),
    Parse(ParseError),
}

impl ScenarioDocumentError {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::Io(_) => "SCENARIO_LOAD_FAILED",
            Self::Parse(_) => "SCENARIO_PARSE_FAILED",
        }
    }
}

impl fmt::Display for ScenarioDocumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "scenario load failed: {err}"),
            Self::Parse(err) => write!(f, "scenario parse failed: {err:?}"),
        }
    }
}

impl std::error::Error for ScenarioDocumentError {}

impl ScenarioDocument {
    pub fn resolve(source: ScenarioSource) -> Result<Self, ScenarioDocumentError> {
        let (fallback_id, content) = match source {
            ScenarioSource::Path(path) => {
                let id = path
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .unwrap_or("scenario")
                    .to_string();
                let content = std::fs::read_to_string(path).map_err(ScenarioDocumentError::Io)?;
                (id, content)
            }
            ScenarioSource::Inline { id, content } => (id, content),
        };
        let runtime_config =
            RawScenarioConfig::parse(&content).map_err(ScenarioDocumentError::Parse)?;
        let (bootstrap_spec, resource_type_ids) =
            RawScenarioConfig::parse_bootstrap_context(&content)
                .map_err(ScenarioDocumentError::Parse)?;
        let canonical_source = canonicalize_scenario_source_v1(&content);
        let id = scenario_id_from_toml(&content).unwrap_or(fallback_id);
        Ok(Self {
            id,
            schema_version: 1,
            scenario_hash: scenario_hash_v1(&canonical_source),
            runtime_config,
            bootstrap_spec,
            resource_type_ids,
            canonical_source,
        })
    }

    pub fn from_runtime_config(
        id: impl Into<String>,
        runtime_config: RuntimeConfig,
        canonical_source: impl Into<String>,
    ) -> Self {
        let canonical_source = canonical_source.into();
        Self {
            id: id.into(),
            schema_version: 1,
            scenario_hash: scenario_hash_v1(&canonical_source),
            runtime_config,
            bootstrap_spec: None,
            resource_type_ids: Vec::new(),
            canonical_source,
        }
    }
}

pub fn canonicalize_scenario_source_v1(source: &str) -> String {
    source
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(normalize_line_v1)
        .collect::<Vec<_>>()
        .join("\n")
}

fn normalize_line_v1(line: &str) -> String {
    if line.starts_with('[') {
        return line.to_string();
    }
    if let Some((key, value)) = line.split_once('=') {
        return format!("{}={}", key.trim(), value.trim());
    }
    line.to_string()
}

pub fn scenario_hash_v1(canonical_source: &str) -> ScenarioHash {
    ScenarioHash(fnv1a64(canonical_source.as_bytes()))
}

pub(crate) fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

fn scenario_id_from_toml(content: &str) -> Option<String> {
    let value = content.parse::<toml::Value>().ok()?;
    value
        .get("scenario_id")
        .and_then(toml::Value::as_str)
        .map(ToOwned::to_owned)
}
