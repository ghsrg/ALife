use crate::runner::scenario_doc::{ScenarioDocument, ScenarioDocumentError, ScenarioSource};
use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScenarioMeta {
    pub id: String,
    pub path: PathBuf,
}

#[derive(Debug)]
pub enum ScenarioError {
    Io(std::io::Error),
    Document(ScenarioDocumentError),
}

impl fmt::Display for ScenarioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "scenario io error: {err}"),
            Self::Document(err) => write!(f, "scenario document error: {err}"),
        }
    }
}

impl std::error::Error for ScenarioError {}

pub fn scan_scenarios(dir: impl AsRef<Path>) -> Result<Vec<ScenarioMeta>, ScenarioError> {
    let mut paths = Vec::new();
    collect_toml_files(dir.as_ref(), &mut paths).map_err(ScenarioError::Io)?;
    let mut scenarios = Vec::new();
    for path in paths {
        let content = std::fs::read_to_string(&path).map_err(ScenarioError::Io)?;
        let Some(id) = scenario_id_from_source(&content) else {
            continue;
        };
        scenarios.push(ScenarioMeta { id, path });
    }
    scenarios.sort_by(|a, b| a.id.cmp(&b.id).then_with(|| a.path.cmp(&b.path)));
    Ok(scenarios)
}

pub fn load_scenario_document(meta: &ScenarioMeta) -> Result<ScenarioDocument, ScenarioError> {
    ScenarioDocument::resolve(ScenarioSource::Path(meta.path.clone()))
        .map_err(ScenarioError::Document)
}

fn collect_toml_files(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), std::io::Error> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_toml_files(&path, out)?;
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("toml") {
            out.push(path);
        }
    }
    Ok(())
}

fn scenario_id_from_source(content: &str) -> Option<String> {
    content
        .parse::<toml::Value>()
        .ok()?
        .get("scenario_id")
        .and_then(toml::Value::as_str)
        .map(ToOwned::to_owned)
}
