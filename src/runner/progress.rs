use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProgressInterval(Duration);

impl Default for ProgressInterval {
    fn default() -> Self {
        Self(Duration::from_millis(2000))
    }
}

impl ProgressInterval {
    pub fn from_millis(ms: u64) -> Result<Self, &'static str> {
        if ms == 0 {
            return Err("--progress-interval-ms must be greater than 0");
        }
        Ok(Self(Duration::from_millis(ms)))
    }

    pub const fn as_duration(self) -> Duration {
        self.0
    }

    pub fn as_millis(self) -> u128 {
        self.0.as_millis()
    }
}

#[derive(Debug, Clone)]
pub struct ProgressSnapshot {
    pub elapsed_ms: u128,
    pub tick: u64,
    pub max_ticks: u64,
    pub ticks_per_second: f32,
    pub cells: usize,
    pub alive_cells: Option<usize>,
    pub dead_cells: Option<usize>,
    pub heat: f32,
    pub waste: f32,
    pub state: String,
    pub collapse_reason: Option<String>,
    pub snapshot_builds: u64,
    pub genome_refreshes: u32,
    pub resource_decay_elapsed_ticks: u64,
}

pub fn format_progress_table(snapshot: &ProgressSnapshot) -> String {
    let elapsed_s = snapshot.elapsed_ms as f32 / 1000.0;
    let alive = snapshot
        .alive_cells
        .map(|value| value.to_string())
        .unwrap_or_else(|| "-".to_string());
    let dead = snapshot
        .dead_cells
        .map(|value| value.to_string())
        .unwrap_or_else(|| "-".to_string());
    let state = snapshot
        .collapse_reason
        .as_deref()
        .unwrap_or(snapshot.state.as_str());

    let tick_progress = format!("{}/{}", snapshot.tick, snapshot.max_ticks);
    format!(
        "\
+-----------+-------------+---------+-------+-------+------+-------+-------+-----------+\n\
| elapsed_s | tick        | tps     | cells | alive | dead | heat  | waste | state     |\n\
+-----------+-------------+---------+-------+-------+------+-------+-------+-----------+\n\
| {elapsed_s:<9.2} | {tick_progress:<11} | {tps:<7.1} | {cells:<5} | {alive:<5} | {dead:<4} | {heat:<5.2} | {waste:<5.2} | {state:<9} |\n\
+-----------+-------------+---------+-------+-------+------+-------+-------+-----------+\n\
| snapshots | genome | decay_dt |\n\
| {snapshot_builds:<9} | {genome_refreshes:<6} | {decay_dt:<8} |",
        tps = snapshot.ticks_per_second,
        cells = snapshot.cells,
        heat = snapshot.heat,
        waste = snapshot.waste,
        snapshot_builds = snapshot.snapshot_builds,
        genome_refreshes = snapshot.genome_refreshes,
        decay_dt = snapshot.resource_decay_elapsed_ticks,
    )
}
