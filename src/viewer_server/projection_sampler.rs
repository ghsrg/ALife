use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjectionDecision {
    Emit,
    EmitForced,
    EmitHeartbeat,
    Skip,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(default)]
pub struct ViewerProjectionConfig {
    pub target_frames_per_second: u32,
    pub minimum_frames_per_second: u32,
    pub render_target_frames_per_second: u32,
    pub maximum_frame_age_ms: u64,
    pub drop_intermediate_frames: bool,
    pub latest_frame_only: bool,
    pub force_frame_on_start: bool,
    pub force_frame_on_pause: bool,
    pub force_frame_on_step: bool,
    pub force_frame_on_resume_if_stale: bool,
    pub force_frame_on_terminal_state: bool,
}

impl Default for ViewerProjectionConfig {
    fn default() -> Self {
        Self {
            target_frames_per_second: 10,
            minimum_frames_per_second: 1,
            render_target_frames_per_second: 30,
            maximum_frame_age_ms: 1000,
            drop_intermediate_frames: true,
            latest_frame_only: true,
            force_frame_on_start: true,
            force_frame_on_pause: true,
            force_frame_on_step: true,
            force_frame_on_resume_if_stale: true,
            force_frame_on_terminal_state: true,
        }
    }
}

pub struct ViewerProjectionSampler {
    config: ViewerProjectionConfig,
    last_emit_at: Option<Instant>,
    projection_sequence: u64,
}

impl ViewerProjectionSampler {
    pub fn new(config: ViewerProjectionConfig) -> Self {
        Self {
            config,
            last_emit_at: None,
            projection_sequence: 0,
        }
    }

    pub fn on_committed_tick(&mut self, _committed_tick: u64, now: Instant) -> ProjectionDecision {
        let interval = Duration::from_millis(
            1000_u64.saturating_div(self.config.target_frames_per_second.max(1) as u64),
        );
        let should_emit = self
            .last_emit_at
            .map(|last| now.duration_since(last) >= interval)
            .unwrap_or(true);
        if should_emit {
            self.record_emit(now);
            ProjectionDecision::Emit
        } else {
            ProjectionDecision::Skip
        }
    }

    pub fn on_wall_clock_idle(&mut self, now: Instant) -> ProjectionDecision {
        let max_age = Duration::from_millis(self.config.maximum_frame_age_ms);
        let minimum_interval = Duration::from_millis(
            1000_u64.saturating_div(self.config.minimum_frames_per_second.max(1) as u64),
        );
        let heartbeat_due = self
            .last_emit_at
            .map(|last| now.duration_since(last) >= max_age.max(minimum_interval))
            .unwrap_or(false);
        if heartbeat_due {
            self.record_emit(now);
            ProjectionDecision::EmitHeartbeat
        } else {
            ProjectionDecision::Skip
        }
    }

    pub fn on_pause(&mut self, now: Instant) -> ProjectionDecision {
        self.force(now, self.config.force_frame_on_pause)
    }

    pub fn on_step(&mut self, now: Instant) -> ProjectionDecision {
        self.force(now, self.config.force_frame_on_step)
    }

    pub fn on_terminal(&mut self, now: Instant) -> ProjectionDecision {
        self.force(now, self.config.force_frame_on_terminal_state)
    }

    fn force(&mut self, now: Instant, enabled: bool) -> ProjectionDecision {
        if enabled {
            self.record_emit(now);
            ProjectionDecision::EmitForced
        } else {
            ProjectionDecision::Skip
        }
    }

    fn record_emit(&mut self, now: Instant) {
        self.last_emit_at = Some(now);
        self.projection_sequence += 1;
    }
}
