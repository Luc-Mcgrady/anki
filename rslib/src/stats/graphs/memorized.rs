use fsrs::DEFAULT_PARAMETERS;
use fsrs::FSRS;
use itertools::izip;
use itertools::Itertools;

use super::GraphsContext;
use crate::prelude::*;
use crate::scheduler::fsrs::memory_state::fsrs_items_for_memory_states;

impl GraphsContext {
    pub fn historical_fsrs(&self) -> Result<Vec<f32>> {
        dbg!("items_for_memory_states");
        let fsrs = FSRS::new(Some(&DEFAULT_PARAMETERS)).unwrap();
        let items = fsrs_items_for_memory_states(
            &fsrs,
            self.revlog.clone(),
            self.next_day_start,
            0.9,
            0.into(),
        )?;
        dbg!("historical_memory_states");
        let memory_states = items.into_iter().filter_map(|(_cid, item)| {
            item.as_ref().map(|item| {
                (
                    item.filtered_revlogs.clone(),
                    fsrs.historical_memory_states(item.item.clone(), None),
                )
            })
        });

        dbg!("historical_retention");
        let mut retention = vec![0.; self.days_elapsed as usize];
        for (revlogs, memory_states) in memory_states {
            if let Ok(memory_states) = memory_states {
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
                    for (i, day) in retention
                        .iter_mut()
                        .take(end_day)
                        .skip(start_day)
                        .enumerate()
                    {
                        *day += fsrs.current_retrievability(memory_state, i as u32, 0.2);
                    }
                }
            }
        }

        dbg!(&retention);

        Ok(retention)
    }
}
