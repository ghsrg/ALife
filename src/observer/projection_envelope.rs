#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProjectionSchemaVersion {
    family: &'static str,
    major: u16,
    minor: u16,
}

impl ProjectionSchemaVersion {
    pub const fn new(family: &'static str, major: u16, minor: u16) -> Self {
        Self {
            family,
            major,
            minor,
        }
    }

    pub const fn family(&self) -> &'static str {
        self.family
    }

    pub const fn major(&self) -> u16 {
        self.major
    }

    pub const fn minor(&self) -> u16 {
        self.minor
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProjectionKind {
    Frame,
    Entity,
    Inspector,
    Metrics,
    OrganismView,
    Lineage,
    Coverage,
    BehaviorProfile,
    BalanceFinding,
    Classification,
    DebugTrace,
}

const PROJECTION_KINDS: &[ProjectionKind] = &[
    ProjectionKind::Frame,
    ProjectionKind::Entity,
    ProjectionKind::Inspector,
    ProjectionKind::Metrics,
    ProjectionKind::OrganismView,
    ProjectionKind::Lineage,
    ProjectionKind::Coverage,
    ProjectionKind::BehaviorProfile,
    ProjectionKind::BalanceFinding,
    ProjectionKind::Classification,
    ProjectionKind::DebugTrace,
];

impl ProjectionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            ProjectionKind::Frame => "FrameProjection",
            ProjectionKind::Entity => "EntityProjection",
            ProjectionKind::Inspector => "InspectorProjection",
            ProjectionKind::Metrics => "MetricsProjection",
            ProjectionKind::OrganismView => "OrganismViewProjection",
            ProjectionKind::Lineage => "LineageProjection",
            ProjectionKind::Coverage => "CoverageProjection",
            ProjectionKind::BehaviorProfile => "BehaviorProfileProjection",
            ProjectionKind::BalanceFinding => "BalanceFindingProjection",
            ProjectionKind::Classification => "ClassificationProjection",
            ProjectionKind::DebugTrace => "DebugTraceProjection",
        }
    }

    pub const fn all_canonical() -> &'static [ProjectionKind] {
        PROJECTION_KINDS
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProjectionEntityKind {
    World,
    Cell,
    Resource,
    Material,
    Field,
    Process,
    Joint,
    Genome,
    Lineage,
    OrganismView,
    Run,
    Coverage,
    Classification,
    BehaviorProfile,
    BalanceFinding,
    DebugTrace,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProjectionSource {
    Live,
    Recorded,
    Historical,
    Fixture,
    Debug,
    Storage,
    AnalyzerReport,
    Offline,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProjectionCompletenessState {
    Full,
    Bounded,
    Sampled,
    Partial,
    DebugSelected,
    Stale,
    Unavailable,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProjectionCompleteness {
    state: ProjectionCompletenessState,
    missing_fields: Vec<&'static str>,
    reason: Option<&'static str>,
}

impl ProjectionCompleteness {
    pub fn full() -> Self {
        Self::new(ProjectionCompletenessState::Full, Vec::new(), None)
    }

    pub fn bounded(reason: &'static str) -> Self {
        Self::new(
            ProjectionCompletenessState::Bounded,
            Vec::new(),
            Some(reason),
        )
    }

    pub fn sampled(reason: &'static str) -> Self {
        Self::new(
            ProjectionCompletenessState::Sampled,
            Vec::new(),
            Some(reason),
        )
    }

    pub fn partial(missing_fields: Vec<&'static str>, reason: &'static str) -> Self {
        Self::new(
            ProjectionCompletenessState::Partial,
            missing_fields,
            Some(reason),
        )
    }

    pub fn debug_selected(reason: &'static str) -> Self {
        Self::new(
            ProjectionCompletenessState::DebugSelected,
            Vec::new(),
            Some(reason),
        )
    }

    pub fn stale(reason: &'static str) -> Self {
        Self::new(ProjectionCompletenessState::Stale, Vec::new(), Some(reason))
    }

    pub fn unavailable(reason: &'static str) -> Self {
        Self::new(
            ProjectionCompletenessState::Unavailable,
            Vec::new(),
            Some(reason),
        )
    }

    pub const fn state(&self) -> ProjectionCompletenessState {
        self.state
    }

    pub fn missing_fields(&self) -> &[&'static str] {
        &self.missing_fields
    }

    pub const fn reason(&self) -> Option<&'static str> {
        self.reason
    }

    fn new(
        state: ProjectionCompletenessState,
        mut missing_fields: Vec<&'static str>,
        reason: Option<&'static str>,
    ) -> Self {
        missing_fields.sort_unstable();
        missing_fields.dedup();
        Self {
            state,
            missing_fields,
            reason,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProjectionBuildContext {
    pub run_id: Option<String>,
    pub config_hash: Option<u64>,
    pub engine_version: Option<String>,
    pub source: ProjectionSource,
    pub completeness: ProjectionCompleteness,
    pub generated_at_unix_ms: u64,
}

impl ProjectionBuildContext {
    pub fn runner_live(
        run_id: impl Into<String>,
        config_hash: u64,
        engine_version: impl Into<String>,
        completeness: ProjectionCompleteness,
        generated_at_unix_ms: u64,
    ) -> Self {
        Self {
            run_id: Some(run_id.into()),
            config_hash: Some(config_hash),
            engine_version: Some(engine_version.into()),
            source: ProjectionSource::Live,
            completeness,
            generated_at_unix_ms,
        }
    }

    pub fn fixture(completeness: ProjectionCompleteness, generated_at_unix_ms: u64) -> Self {
        Self {
            run_id: None,
            config_hash: None,
            engine_version: None,
            source: ProjectionSource::Fixture,
            completeness,
            generated_at_unix_ms,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProjectionEnvelope {
    pub schema_version: ProjectionSchemaVersion,
    pub projection_kind: ProjectionKind,
    pub run_id: Option<String>,
    pub tick: Option<u64>,
    pub config_hash: Option<u64>,
    pub engine_version: Option<String>,
    pub source: ProjectionSource,
    pub completeness: ProjectionCompleteness,
    pub generated_at_unix_ms: u64,
}

impl ProjectionEnvelope {
    pub fn new(
        schema_version: ProjectionSchemaVersion,
        projection_kind: ProjectionKind,
        tick: Option<u64>,
        context: ProjectionBuildContext,
    ) -> Self {
        Self {
            schema_version,
            projection_kind,
            run_id: context.run_id,
            tick,
            config_hash: context.config_hash,
            engine_version: context.engine_version,
            source: context.source,
            completeness: context.completeness,
            generated_at_unix_ms: context.generated_at_unix_ms,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnvelopedProjection<T> {
    pub envelope: ProjectionEnvelope,
    pub payload: T,
}

impl<T> EnvelopedProjection<T> {
    pub const fn new(envelope: ProjectionEnvelope, payload: T) -> Self {
        Self { envelope, payload }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SchemaExportDisposition {
    RustTypedContractOnly,
}

pub const fn schema_export_disposition() -> SchemaExportDisposition {
    SchemaExportDisposition::RustTypedContractOnly
}
