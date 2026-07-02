use crate::core::ids::{CellId, EventId};
use crate::core::units::Tick;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventKind {
    RunStarted,
    TickCommitted,
    MandatoryCostFailed,
    LifecycleChanged,
    CapacityWarning,
    HeatWarning,
    WasteWarning,
    CellDead,
    SnapshotEmitted,
    RunFinished,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Event {
    pub id: EventId,
    pub tick: Tick,
    pub kind: EventKind,
    pub cell_id: Option<CellId>,
}

#[derive(Clone, Debug, Default)]
pub struct EventBuffer {
    events: Vec<Event>,
    next_event_id: u32,
}

impl EventBuffer {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            events: Vec::with_capacity(capacity),
            next_event_id: 1,
        }
    }

    pub fn push(&mut self, tick: Tick, kind: EventKind, cell_id: Option<CellId>) -> EventId {
        let id = EventId::from_raw(self.next_event_id);
        self.next_event_id += 1;
        self.events.push(Event {
            id,
            tick,
            kind,
            cell_id,
        });
        id
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn iter_ordered(&self) -> impl Iterator<Item = &Event> {
        self.events.iter()
    }
}
