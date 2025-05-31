use crate::{
    prelude::*,
    scheduler::fsrs::memory_state::{self, fsrs_items_for_memory_states},
};
use anki_proto::stats::RevlogEntry;
use fsrs::{DEFAULT_PARAMETERS, FSRS};
use itertools::{izip, Itertools};

use super::{retention, GraphsContext};

#[derive(Debug, Clone)]
pub struct HistoricalReviewLog {
    pub cid: i64,
    pub day: i64,
    pub rating: u32,
}

#[derive(Debug, Clone)]
pub struct RangeBounds {
    pub from: u64,
    pub to: u64,
}

pub fn convert_review_log_for_historical(log: RevlogEntry) -> HistoricalReviewLog {
    HistoricalReviewLog {
        cid: log.cid,
        day: log.id,
        rating: log.button_chosen,
    }
}

impl GraphsContext {
    pub fn historical_fsrs(&self) -> Result<Vec<f32>> {
        let fsrs = FSRS::new(Some(&DEFAULT_PARAMETERS)).unwrap();
        let items = fsrs_items_for_memory_states(
            &fsrs,
            self.revlog.clone(),
            self.next_day_start,
            0.9,
            0.into(),
        )?;
        let memory_states = items
        .into_iter()
        .filter_map(|(_cid, item)| {
            item.as_ref().map(|item| {
                (
                    item.filtered_revlogs.clone(),
                    fsrs.historical_memory_states(item.item.clone(), None),
                )
            })
        })
        .collect_vec();
    
    let mut retention = vec![0.; self.days_elapsed as usize];
    for (revlogs, memory_states) in memory_states {
        if let Ok(memory_states) = memory_states {
            for (from_to, memory_state) in izip!(revlogs.windows(2), memory_states) {
                let start_day = from_to[0].days_elapsed(self.next_day_start) as usize;
                let end_day = from_to[1].days_elapsed(self.next_day_start) as usize;
                for (i, day) in retention
                .iter_mut()
                .take(end_day)
                .skip(start_day)
                .enumerate()
                {
                    *day+=fsrs.current_retrievability(memory_state, i as u32, 0.2);
                }
            }
            }
        }
        
        dbg!(&retention);
        
        Ok(retention)
    }
}
