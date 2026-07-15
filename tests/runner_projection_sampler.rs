use alife::viewer_server::projection_sampler::{
    ProjectionDecision, ViewerProjectionConfig, ViewerProjectionSampler,
};
use std::time::{Duration, Instant};

#[test]
fn sampler_allows_target_fps_but_drops_intermediate_frames() {
    let config = ViewerProjectionConfig {
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
    };
    let mut sampler = ViewerProjectionSampler::new(config);
    let now = Instant::now();

    assert_eq!(
        sampler.on_committed_tick(1, now),
        ProjectionDecision::Emit
    );
    assert_eq!(
        sampler.on_committed_tick(2, now + Duration::from_millis(50)),
        ProjectionDecision::Skip
    );
    assert_eq!(
        sampler.on_committed_tick(3, now + Duration::from_millis(100)),
        ProjectionDecision::Emit
    );
}

#[test]
fn sampler_forces_pause_step_and_terminal_frames() {
    let mut sampler = ViewerProjectionSampler::new(ViewerProjectionConfig::default());
    let now = Instant::now();

    assert_eq!(sampler.on_pause(now), ProjectionDecision::EmitForced);
    assert_eq!(sampler.on_step(now), ProjectionDecision::EmitForced);
    assert_eq!(sampler.on_terminal(now), ProjectionDecision::EmitForced);
}

#[test]
fn sampler_emits_heartbeat_when_no_new_committed_tick_arrives() {
    let mut sampler = ViewerProjectionSampler::new(ViewerProjectionConfig::default());
    let now = Instant::now();

    assert_eq!(
        sampler.on_committed_tick(1, now),
        ProjectionDecision::Emit
    );
    assert_eq!(
        sampler.on_wall_clock_idle(now + Duration::from_millis(999)),
        ProjectionDecision::Skip
    );
    assert_eq!(
        sampler.on_wall_clock_idle(now + Duration::from_millis(1000)),
        ProjectionDecision::EmitHeartbeat
    );
}
