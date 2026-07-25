pub mod analytics_export;

use std::fmt;
use std::path::Path;

use rusqlite::{Connection, OptionalExtension, params};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RunId(String);

impl RunId {
    pub fn new(value: impl Into<String>) -> Result<Self, StorageModelError> {
        let value = value.into();
        if value.trim().is_empty()
            || value.contains('/')
            || value.contains('\\')
            || value.contains('\0')
        {
            return Err(StorageModelError::InvalidRunId);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RunId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StorageSchemaVersion {
    major: u16,
    minor: u16,
}

impl StorageSchemaVersion {
    pub const fn current() -> Self {
        Self { major: 1, minor: 0 }
    }

    pub const fn major(self) -> u16 {
        self.major
    }

    pub const fn minor(self) -> u16 {
        self.minor
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunStatus {
    Prepared,
    Running,
    Completed,
    Failed,
}

impl RunStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Prepared => "prepared",
            Self::Running => "running",
            Self::Completed => "completed",
            Self::Failed => "failed",
        }
    }

    fn from_str(value: &str) -> Result<Self, StorageModelError> {
        match value {
            "prepared" => Ok(Self::Prepared),
            "running" => Ok(Self::Running),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            _ => Err(StorageModelError::InvalidStatus),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactKind {
    Snapshot,
    LineageEvents,
    ProjectionKeyframe,
    Summary,
}

impl ArtifactKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
            Self::LineageEvents => "lineage_events",
            Self::ProjectionKeyframe => "projection_keyframe",
            Self::Summary => "summary",
        }
    }

    fn from_str(value: &str) -> Result<Self, StorageModelError> {
        match value {
            "snapshot" => Ok(Self::Snapshot),
            "lineage_events" => Ok(Self::LineageEvents),
            "projection_keyframe" => Ok(Self::ProjectionKeyframe),
            "summary" => Ok(Self::Summary),
            _ => Err(StorageModelError::InvalidArtifactKind),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactCompleteness {
    Full,
    Bounded,
    Sampled,
    Partial,
    Unavailable,
}

impl ArtifactCompleteness {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Full => "full",
            Self::Bounded => "bounded",
            Self::Sampled => "sampled",
            Self::Partial => "partial",
            Self::Unavailable => "unavailable",
        }
    }

    fn from_str(value: &str) -> Result<Self, StorageModelError> {
        match value {
            "full" => Ok(Self::Full),
            "bounded" => Ok(Self::Bounded),
            "sampled" => Ok(Self::Sampled),
            "partial" => Ok(Self::Partial),
            "unavailable" => Ok(Self::Unavailable),
            _ => Err(StorageModelError::InvalidArtifactCompleteness),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactRecord {
    run_id: RunId,
    kind: ArtifactKind,
    path: String,
    tick_range: TickRange,
    completeness: ArtifactCompleteness,
    notes: Option<String>,
}

impl ArtifactRecord {
    pub fn new(
        run_id: RunId,
        kind: ArtifactKind,
        path: impl Into<String>,
        tick_range: TickRange,
        completeness: ArtifactCompleteness,
        notes: Option<impl Into<String>>,
    ) -> Result<Self, StorageModelError> {
        let path = path.into();
        if path.trim().is_empty() || path.contains('\0') {
            return Err(StorageModelError::InvalidArtifactPath);
        }
        Ok(Self {
            run_id,
            kind,
            path,
            tick_range,
            completeness,
            notes: notes.map(Into::into),
        })
    }

    pub const fn run_id(&self) -> &RunId {
        &self.run_id
    }

    pub const fn kind(&self) -> ArtifactKind {
        self.kind
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub const fn tick_range(&self) -> TickRange {
        self.tick_range
    }

    pub const fn completeness(&self) -> ArtifactCompleteness {
        self.completeness
    }

    pub fn notes(&self) -> Option<&str> {
        self.notes.as_deref()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TickRange {
    start: u64,
    end: Option<u64>,
}

impl TickRange {
    pub fn open(start: u64) -> Self {
        Self { start, end: None }
    }

    pub fn closed(start: u64, end: u64) -> Result<Self, StorageModelError> {
        if end < start {
            return Err(StorageModelError::InvalidTickRange);
        }
        Ok(Self {
            start,
            end: Some(end),
        })
    }

    pub const fn start(self) -> u64 {
        self.start
    }

    pub const fn end(self) -> Option<u64> {
        self.end
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunMetadata {
    run_id: RunId,
    scenario_id: String,
    scenario_hash: String,
    effective_seed: u64,
    engine_version: String,
    scenario_schema_version: u32,
    storage_schema_version: StorageSchemaVersion,
    status: RunStatus,
    tick_range: TickRange,
    started_at_unix_ms: u64,
    ended_at_unix_ms: Option<u64>,
}

impl RunMetadata {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        run_id: RunId,
        scenario_id: impl Into<String>,
        scenario_hash: impl Into<String>,
        effective_seed: u64,
        engine_version: impl Into<String>,
        scenario_schema_version: u32,
        storage_schema_version: StorageSchemaVersion,
        status: RunStatus,
        tick_range: TickRange,
        started_at_unix_ms: u64,
        ended_at_unix_ms: Option<u64>,
    ) -> Result<Self, StorageModelError> {
        let scenario_id = scenario_id.into();
        let scenario_hash = scenario_hash.into();
        let engine_version = engine_version.into();
        if scenario_id.trim().is_empty()
            || scenario_hash.trim().is_empty()
            || engine_version.trim().is_empty()
        {
            return Err(StorageModelError::MissingRequiredField);
        }
        if let Some(ended_at_unix_ms) = ended_at_unix_ms
            && ended_at_unix_ms < started_at_unix_ms
        {
            return Err(StorageModelError::InvalidTimeRange);
        }
        Ok(Self {
            run_id,
            scenario_id,
            scenario_hash,
            effective_seed,
            engine_version,
            scenario_schema_version,
            storage_schema_version,
            status,
            tick_range,
            started_at_unix_ms,
            ended_at_unix_ms,
        })
    }

    pub const fn run_id(&self) -> &RunId {
        &self.run_id
    }

    pub fn scenario_id(&self) -> &str {
        &self.scenario_id
    }

    pub fn scenario_hash(&self) -> &str {
        &self.scenario_hash
    }

    pub const fn effective_seed(&self) -> u64 {
        self.effective_seed
    }

    pub fn engine_version(&self) -> &str {
        &self.engine_version
    }

    pub const fn scenario_schema_version(&self) -> u32 {
        self.scenario_schema_version
    }

    pub const fn storage_schema_version(&self) -> StorageSchemaVersion {
        self.storage_schema_version
    }

    pub const fn status(&self) -> RunStatus {
        self.status
    }

    pub const fn tick_range(&self) -> TickRange {
        self.tick_range
    }

    pub const fn started_at_unix_ms(&self) -> u64 {
        self.started_at_unix_ms
    }

    pub const fn ended_at_unix_ms(&self) -> Option<u64> {
        self.ended_at_unix_ms
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageModelError {
    InvalidRunId,
    InvalidTickRange,
    InvalidTimeRange,
    MissingRequiredField,
    InvalidStatus,
    InvalidArtifactKind,
    InvalidArtifactCompleteness,
    InvalidArtifactPath,
}

impl fmt::Display for StorageModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRunId => f.write_str("invalid run id"),
            Self::InvalidTickRange => f.write_str("invalid tick range"),
            Self::InvalidTimeRange => f.write_str("invalid time range"),
            Self::MissingRequiredField => f.write_str("missing required storage metadata field"),
            Self::InvalidStatus => f.write_str("invalid run status"),
            Self::InvalidArtifactKind => f.write_str("invalid artifact kind"),
            Self::InvalidArtifactCompleteness => f.write_str("invalid artifact completeness"),
            Self::InvalidArtifactPath => f.write_str("invalid artifact path"),
        }
    }
}

impl std::error::Error for StorageModelError {}

#[derive(Debug)]
pub enum StorageIndexError {
    Sqlite(rusqlite::Error),
    Model(StorageModelError),
}

impl fmt::Display for StorageIndexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sqlite(err) => write!(f, "sqlite storage index error: {err}"),
            Self::Model(err) => write!(f, "storage model error: {err}"),
        }
    }
}

impl std::error::Error for StorageIndexError {}

impl From<rusqlite::Error> for StorageIndexError {
    fn from(value: rusqlite::Error) -> Self {
        Self::Sqlite(value)
    }
}

impl From<StorageModelError> for StorageIndexError {
    fn from(value: StorageModelError) -> Self {
        Self::Model(value)
    }
}

pub struct SqliteRunIndex {
    connection: Connection,
}

impl SqliteRunIndex {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, StorageIndexError> {
        let connection = Connection::open(path)?;
        let index = Self { connection };
        index.ensure_schema()?;
        Ok(index)
    }

    pub fn upsert_run(&self, metadata: &RunMetadata) -> Result<(), StorageIndexError> {
        self.connection.execute(
            "INSERT INTO runs (
                run_id,
                scenario_id,
                scenario_hash,
                effective_seed,
                engine_version,
                scenario_schema_version,
                storage_schema_major,
                storage_schema_minor,
                status,
                tick_start,
                tick_end,
                started_at_unix_ms,
                ended_at_unix_ms
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
            ON CONFLICT(run_id) DO UPDATE SET
                scenario_id = excluded.scenario_id,
                scenario_hash = excluded.scenario_hash,
                effective_seed = excluded.effective_seed,
                engine_version = excluded.engine_version,
                scenario_schema_version = excluded.scenario_schema_version,
                storage_schema_major = excluded.storage_schema_major,
                storage_schema_minor = excluded.storage_schema_minor,
                status = excluded.status,
                tick_start = excluded.tick_start,
                tick_end = excluded.tick_end,
                started_at_unix_ms = excluded.started_at_unix_ms,
                ended_at_unix_ms = excluded.ended_at_unix_ms",
            params![
                metadata.run_id().as_str(),
                metadata.scenario_id(),
                metadata.scenario_hash(),
                metadata.effective_seed(),
                metadata.engine_version(),
                metadata.scenario_schema_version(),
                metadata.storage_schema_version().major(),
                metadata.storage_schema_version().minor(),
                metadata.status().as_str(),
                metadata.tick_range().start(),
                metadata.tick_range().end(),
                metadata.started_at_unix_ms(),
                metadata.ended_at_unix_ms(),
            ],
        )?;
        Ok(())
    }

    pub fn get_run(&self, run_id: &RunId) -> Result<Option<RunMetadata>, StorageIndexError> {
        let row = self
            .connection
            .query_row(
                "SELECT
                    run_id,
                    scenario_id,
                    scenario_hash,
                    effective_seed,
                    engine_version,
                    scenario_schema_version,
                    storage_schema_major,
                    storage_schema_minor,
                    status,
                    tick_start,
                    tick_end,
                    started_at_unix_ms,
                    ended_at_unix_ms
                FROM runs
                WHERE run_id = ?1",
                params![run_id.as_str()],
                |row| {
                    Ok(RawRunMetadataRow {
                        run_id: row.get(0)?,
                        scenario_id: row.get(1)?,
                        scenario_hash: row.get(2)?,
                        effective_seed: row.get(3)?,
                        engine_version: row.get(4)?,
                        scenario_schema_version: row.get(5)?,
                        storage_schema_major: row.get(6)?,
                        storage_schema_minor: row.get(7)?,
                        status: row.get(8)?,
                        tick_start: row.get(9)?,
                        tick_end: row.get(10)?,
                        started_at_unix_ms: row.get(11)?,
                        ended_at_unix_ms: row.get(12)?,
                    })
                },
            )
            .optional()?;

        row.map(RawRunMetadataRow::try_into_metadata).transpose()
    }

    pub fn insert_artifact(&self, artifact: &ArtifactRecord) -> Result<(), StorageIndexError> {
        self.connection.execute(
            "INSERT INTO run_artifacts (
                run_id,
                artifact_kind,
                artifact_path,
                tick_start,
                tick_end,
                completeness,
                notes
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                artifact.run_id().as_str(),
                artifact.kind().as_str(),
                artifact.path(),
                artifact.tick_range().start(),
                artifact.tick_range().end(),
                artifact.completeness().as_str(),
                artifact.notes(),
            ],
        )?;
        Ok(())
    }

    pub fn artifacts_for_run(
        &self,
        run_id: &RunId,
    ) -> Result<Vec<ArtifactRecord>, StorageIndexError> {
        let mut statement = self.connection.prepare(
            "SELECT
                run_id,
                artifact_kind,
                artifact_path,
                tick_start,
                tick_end,
                completeness,
                notes
            FROM run_artifacts
            WHERE run_id = ?1
            ORDER BY id",
        )?;
        let rows = statement.query_map(params![run_id.as_str()], |row| {
            Ok(RawArtifactRow {
                run_id: row.get(0)?,
                artifact_kind: row.get(1)?,
                artifact_path: row.get(2)?,
                tick_start: row.get(3)?,
                tick_end: row.get(4)?,
                completeness: row.get(5)?,
                notes: row.get(6)?,
            })
        })?;

        let mut artifacts = Vec::new();
        for row in rows {
            artifacts.push(row?.try_into_artifact()?);
        }
        Ok(artifacts)
    }

    fn ensure_schema(&self) -> Result<(), StorageIndexError> {
        self.connection.execute_batch(
            "CREATE TABLE IF NOT EXISTS runs (
                run_id TEXT PRIMARY KEY NOT NULL,
                scenario_id TEXT NOT NULL,
                scenario_hash TEXT NOT NULL,
                effective_seed INTEGER NOT NULL,
                engine_version TEXT NOT NULL,
                scenario_schema_version INTEGER NOT NULL,
                storage_schema_major INTEGER NOT NULL,
                storage_schema_minor INTEGER NOT NULL,
                status TEXT NOT NULL,
                tick_start INTEGER NOT NULL,
                tick_end INTEGER,
                started_at_unix_ms INTEGER NOT NULL,
                ended_at_unix_ms INTEGER
            );

            CREATE TABLE IF NOT EXISTS run_artifacts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                run_id TEXT NOT NULL,
                artifact_kind TEXT NOT NULL,
                artifact_path TEXT NOT NULL,
                tick_start INTEGER NOT NULL,
                tick_end INTEGER,
                completeness TEXT NOT NULL,
                notes TEXT,
                FOREIGN KEY(run_id) REFERENCES runs(run_id)
            );

            CREATE INDEX IF NOT EXISTS idx_run_artifacts_run_kind_tick
            ON run_artifacts(run_id, artifact_kind, tick_start, tick_end);",
        )?;
        Ok(())
    }
}

struct RawRunMetadataRow {
    run_id: String,
    scenario_id: String,
    scenario_hash: String,
    effective_seed: u64,
    engine_version: String,
    scenario_schema_version: u32,
    storage_schema_major: u16,
    storage_schema_minor: u16,
    status: String,
    tick_start: u64,
    tick_end: Option<u64>,
    started_at_unix_ms: u64,
    ended_at_unix_ms: Option<u64>,
}

impl RawRunMetadataRow {
    fn try_into_metadata(self) -> Result<RunMetadata, StorageIndexError> {
        let tick_range = match self.tick_end {
            Some(end) => TickRange::closed(self.tick_start, end)?,
            None => TickRange::open(self.tick_start),
        };
        RunMetadata::new(
            RunId::new(self.run_id)?,
            self.scenario_id,
            self.scenario_hash,
            self.effective_seed,
            self.engine_version,
            self.scenario_schema_version,
            StorageSchemaVersion {
                major: self.storage_schema_major,
                minor: self.storage_schema_minor,
            },
            RunStatus::from_str(&self.status)?,
            tick_range,
            self.started_at_unix_ms,
            self.ended_at_unix_ms,
        )
        .map_err(StorageIndexError::from)
    }
}

struct RawArtifactRow {
    run_id: String,
    artifact_kind: String,
    artifact_path: String,
    tick_start: u64,
    tick_end: Option<u64>,
    completeness: String,
    notes: Option<String>,
}

impl RawArtifactRow {
    fn try_into_artifact(self) -> Result<ArtifactRecord, StorageIndexError> {
        let tick_range = match self.tick_end {
            Some(end) => TickRange::closed(self.tick_start, end)?,
            None => TickRange::open(self.tick_start),
        };
        ArtifactRecord::new(
            RunId::new(self.run_id)?,
            ArtifactKind::from_str(&self.artifact_kind)?,
            self.artifact_path,
            tick_range,
            ArtifactCompleteness::from_str(&self.completeness)?,
            self.notes,
        )
        .map_err(StorageIndexError::from)
    }
}
