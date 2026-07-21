use crate::core::genome::{GenomeId, GenomeOutputId, GenomeTemplateId};
use crate::core::ids::{CellId, LineageEventId};
use crate::core::units::Tick;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LineageEventKind {
    FounderCell,
    GenomeCopied,
    CellDivided,
    CellDied,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FounderCellLineage {
    pub cell_id: CellId,
    pub genome_id: Option<GenomeId>,
    pub genome_template_id: Option<GenomeTemplateId>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GenomeMutationDelta {
    pub output_id: GenomeOutputId,
    pub before: f32,
    pub after: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GenomeCopyLineage {
    pub cell_id: CellId,
    pub parent_genome_id: GenomeId,
    pub copied_genome_id: GenomeId,
    pub genome_template_id: GenomeTemplateId,
    pub carrier_material_id: String,
    pub carrier_amount: f32,
    pub carrier_integrity: f32,
    pub mutation_deltas: Vec<GenomeMutationDelta>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DivisionLineage {
    pub parent_cell_id: CellId,
    pub daughter_a_cell_id: CellId,
    pub daughter_b_cell_id: CellId,
    pub parent_genome_id: Option<GenomeId>,
    pub daughter_a_genome_id: Option<GenomeId>,
    pub daughter_b_genome_id: Option<GenomeId>,
    pub split_ratio: f32,
    pub partition_loss_fraction: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CellDeathLineage {
    pub cell_id: CellId,
    pub genome_id: Option<GenomeId>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LineageEventPayload {
    FounderCell(FounderCellLineage),
    GenomeCopied(GenomeCopyLineage),
    CellDivided(DivisionLineage),
    CellDied(CellDeathLineage),
}

#[derive(Clone, Debug, PartialEq)]
pub struct LineageEvent {
    id: LineageEventId,
    tick: Tick,
    payload: LineageEventPayload,
}

impl LineageEvent {
    pub const fn id(&self) -> LineageEventId {
        self.id
    }

    pub const fn tick(&self) -> Tick {
        self.tick
    }

    pub const fn kind(&self) -> LineageEventKind {
        match self.payload {
            LineageEventPayload::FounderCell(_) => LineageEventKind::FounderCell,
            LineageEventPayload::GenomeCopied(_) => LineageEventKind::GenomeCopied,
            LineageEventPayload::CellDivided(_) => LineageEventKind::CellDivided,
            LineageEventPayload::CellDied(_) => LineageEventKind::CellDied,
        }
    }

    pub const fn as_founder_cell(&self) -> Option<&FounderCellLineage> {
        match &self.payload {
            LineageEventPayload::FounderCell(payload) => Some(payload),
            _ => None,
        }
    }

    pub const fn as_genome_copied(&self) -> Option<&GenomeCopyLineage> {
        match &self.payload {
            LineageEventPayload::GenomeCopied(payload) => Some(payload),
            _ => None,
        }
    }

    pub const fn as_cell_divided(&self) -> Option<&DivisionLineage> {
        match &self.payload {
            LineageEventPayload::CellDivided(payload) => Some(payload),
            _ => None,
        }
    }

    pub const fn as_cell_died(&self) -> Option<&CellDeathLineage> {
        match &self.payload {
            LineageEventPayload::CellDied(payload) => Some(payload),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LineageEventLog {
    events: Vec<LineageEvent>,
    next_event_id: u32,
}

impl LineageEventLog {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            events: Vec::with_capacity(capacity),
            next_event_id: 0,
        }
    }

    pub fn iter_ordered(&self) -> impl Iterator<Item = &LineageEvent> {
        self.events.iter()
    }

    pub fn push_founder_cell(
        &mut self,
        tick: Tick,
        cell_id: CellId,
        genome_id: Option<GenomeId>,
        genome_template_id: Option<GenomeTemplateId>,
    ) {
        self.push(
            tick,
            LineageEventPayload::FounderCell(FounderCellLineage {
                cell_id,
                genome_id,
                genome_template_id,
            }),
        );
    }

    pub fn push_genome_copied(&mut self, tick: Tick, lineage: GenomeCopyLineage) {
        self.push(tick, LineageEventPayload::GenomeCopied(lineage));
    }

    pub fn push_cell_divided(&mut self, tick: Tick, lineage: DivisionLineage) {
        self.push(tick, LineageEventPayload::CellDivided(lineage));
    }

    pub fn push_cell_died(&mut self, tick: Tick, cell_id: CellId, genome_id: Option<GenomeId>) {
        self.push(
            tick,
            LineageEventPayload::CellDied(CellDeathLineage { cell_id, genome_id }),
        );
    }

    fn push(&mut self, tick: Tick, payload: LineageEventPayload) {
        let id = LineageEventId::from_raw(self.next_event_id);
        self.next_event_id += 1;
        self.events.push(LineageEvent { id, tick, payload });
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CellLineageRecord {
    pub cell_id: CellId,
    pub parent_cell_id: Option<CellId>,
    pub generation: u32,
    pub genome_id: Option<GenomeId>,
    pub born_tick: Tick,
    pub death_tick: Option<Tick>,
    pub alive: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GenomeLineageRecord {
    pub genome_id: GenomeId,
    pub parent_genome_id: Option<GenomeId>,
    pub template_id: Option<GenomeTemplateId>,
    pub mutation_count: usize,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LineageReplaySummary {
    cells: Vec<CellLineageRecord>,
    genomes: Vec<GenomeLineageRecord>,
}

impl LineageReplaySummary {
    pub fn cell(&self, cell_id: CellId) -> Option<&CellLineageRecord> {
        self.cells.iter().find(|record| record.cell_id == cell_id)
    }

    pub fn genome(&self, genome_id: GenomeId) -> Option<&GenomeLineageRecord> {
        self.genomes
            .iter()
            .find(|record| record.genome_id == genome_id)
    }
}

pub fn build_lineage_replay_summary(log: &LineageEventLog) -> LineageReplaySummary {
    let mut summary = LineageReplaySummary::default();

    for event in log.iter_ordered() {
        match &event.payload {
            LineageEventPayload::FounderCell(payload) => {
                upsert_cell(
                    &mut summary.cells,
                    CellLineageRecord {
                        cell_id: payload.cell_id,
                        parent_cell_id: None,
                        generation: 0,
                        genome_id: payload.genome_id,
                        born_tick: event.tick,
                        death_tick: None,
                        alive: true,
                    },
                );
                if let Some(genome_id) = payload.genome_id {
                    upsert_genome(
                        &mut summary.genomes,
                        GenomeLineageRecord {
                            genome_id,
                            parent_genome_id: None,
                            template_id: payload.genome_template_id.clone(),
                            mutation_count: 0,
                        },
                    );
                }
            }
            LineageEventPayload::GenomeCopied(payload) => {
                upsert_genome(
                    &mut summary.genomes,
                    GenomeLineageRecord {
                        genome_id: payload.copied_genome_id,
                        parent_genome_id: Some(payload.parent_genome_id),
                        template_id: Some(payload.genome_template_id.clone()),
                        mutation_count: payload.mutation_deltas.len(),
                    },
                );
            }
            LineageEventPayload::CellDivided(payload) => {
                let parent_generation = summary
                    .cell(payload.parent_cell_id)
                    .map(|record| record.generation)
                    .unwrap_or(0);
                if let Some(record) = summary
                    .cells
                    .iter_mut()
                    .find(|record| record.cell_id == payload.daughter_a_cell_id)
                {
                    record.genome_id = payload.daughter_a_genome_id;
                    record.alive = true;
                } else {
                    summary.cells.push(CellLineageRecord {
                        cell_id: payload.daughter_a_cell_id,
                        parent_cell_id: Some(payload.parent_cell_id),
                        generation: parent_generation,
                        genome_id: payload.daughter_a_genome_id,
                        born_tick: event.tick,
                        death_tick: None,
                        alive: true,
                    });
                }
                upsert_cell(
                    &mut summary.cells,
                    CellLineageRecord {
                        cell_id: payload.daughter_b_cell_id,
                        parent_cell_id: Some(payload.parent_cell_id),
                        generation: parent_generation + 1,
                        genome_id: payload.daughter_b_genome_id,
                        born_tick: event.tick,
                        death_tick: None,
                        alive: true,
                    },
                );
            }
            LineageEventPayload::CellDied(payload) => {
                if let Some(record) = summary
                    .cells
                    .iter_mut()
                    .find(|record| record.cell_id == payload.cell_id)
                {
                    record.alive = false;
                    record.death_tick = Some(event.tick);
                    record.genome_id = payload.genome_id;
                } else {
                    summary.cells.push(CellLineageRecord {
                        cell_id: payload.cell_id,
                        parent_cell_id: None,
                        generation: 0,
                        genome_id: payload.genome_id,
                        born_tick: event.tick,
                        death_tick: Some(event.tick),
                        alive: false,
                    });
                }
            }
        }
    }

    summary
}

fn upsert_cell(records: &mut Vec<CellLineageRecord>, next: CellLineageRecord) {
    if let Some(record) = records
        .iter_mut()
        .find(|record| record.cell_id == next.cell_id)
    {
        *record = next;
    } else {
        records.push(next);
    }
}

fn upsert_genome(records: &mut Vec<GenomeLineageRecord>, next: GenomeLineageRecord) {
    if let Some(record) = records
        .iter_mut()
        .find(|record| record.genome_id == next.genome_id)
    {
        *record = next;
    } else {
        records.push(next);
    }
}
