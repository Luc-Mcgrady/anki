// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html
use std::collections::HashMap;

use fsrs::DEFAULT_PARAMETERS;
use fsrs::FSRS;
use itertools::izip;
use itertools::Itertools;

use super::GraphsContext;
use crate::prelude::*;
use crate::scheduler::fsrs::memory_state::fsrs_items_for_memory_states;

impl GraphsContext {
    pub fn historical_fsrs(&self) -> Result<HashMap<usize, f32>> {
        let fsrs = FSRS::new(Some(&DEFAULT_PARAMETERS)).unwrap();
        let items = fsrs_items_for_memory_states(
            &fsrs,
            self.revlog.clone(),
            self.next_day_start,
            0.9,
            0.into(),
        )?;
        let memory_states = items.into_iter().filter_map(|(_cid, item)| {
            item.map(|item| {
                (
                    item.filtered_revlogs,
                    fsrs.historical_memory_states(item.item, item.starting_state),
                )
            })
        });

        let mut retention = HashMap::new();
        for (revlogs, memory_states) in memory_states {
            let memory_states = memory_states?;
            for (from_to, memory_state) in izip!(
                revlogs
                    .into_iter()
                    .map(|r| r.days_elapsed(self.next_day_start) as usize)
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
