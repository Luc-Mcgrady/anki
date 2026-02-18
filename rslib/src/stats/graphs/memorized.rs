// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html
use std::collections::HashMap;

use fsrs::DEFAULT_PARAMETERS;
use fsrs::FSRS;
use itertools::izip;
use itertools::Itertools;

use super::GraphsContext;
use crate::prelude::*;
use crate::scheduler::fsrs::memory_state::fsrs_item_for_memory_state;

pub(crate) struct MemorizedContext {
    pub graph_context: GraphsContext,
    pub per_preset_fsrs: HashMap<u64, FSRS>,
}

impl MemorizedContext {
    pub fn historical_fsrs(&self) -> Result<HashMap<usize, f32>> {
        let gctx = &self.graph_context;
        let fsrs = FSRS::new(Some(&DEFAULT_PARAMETERS)).unwrap();

        let card_logs = gctx.revlog.clone().into_iter().chunk_by(|r| r.cid);

        let items = card_logs.into_iter().filter_map(|(_card_id, group)| {
            fsrs_item_for_memory_state(
                &fsrs,
                group.collect_vec(),
                gctx.next_day_start,
                0.9,
                0.into(),
            )
            .ok()
        });

        let items = items
            .filter(|item| item.is_some())
            .map(|item| item.unwrap())
            .collect_vec();
        let starting_states = items
            .iter()
            .map(|item| item.starting_state.clone())
            .collect_vec();
        let filtered_revlogs = items
            .iter()
            .map(|item| item.filtered_revlogs.clone())
            .collect_vec();
        let items = items.into_iter().map(|item| item.item).collect_vec();
        let memory_states = fsrs.historical_memory_state_batch(items, Some(starting_states))?;
        let memory_states = izip![filtered_revlogs, memory_states];

        let mut retention = HashMap::new();
        for (revlogs, memory_states) in memory_states {
            for (from_to, memory_state) in izip!(
                revlogs
                    .into_iter()
                    .map(|r| r.days_elapsed(gctx.next_day_start) as usize)
                    .chain([0])
                    .collect_vec()
                    .windows(2),
                memory_states
            ) {
                let start_day = from_to[1];
                let end_day = from_to[0];
                for i in start_day..end_day {
                    *retention.entry(i).or_default() +=
                        fsrs.current_retrievability(memory_state, i as u32, 0.2);
                }
            }
        }

        Ok(retention)
    }
}
