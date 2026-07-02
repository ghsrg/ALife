#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CommitSummary {
    pub ticks_committed: u64,
    pub events_emitted: usize,
}
