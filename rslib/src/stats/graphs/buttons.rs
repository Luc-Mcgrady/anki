// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use std::i64;

use anki_proto::stats::graphs_response::buttons::ButtonCounts;
use anki_proto::stats::graphs_response::Buttons;

use super::GraphsContext;
use super::TimestampSecs;
use crate::revlog::RevlogEntry;
use crate::revlog::RevlogReviewKind;

impl GraphsContext {
    pub(super) fn buttons(&self) -> Buttons {
        let mut all_time = ButtonCounts {
            learning: vec![0; 4],
            young: vec![0; 4],
            mature: vec![0; 4],
        };
        let mut conditional_buckets = vec![
            (
                None,
                self.next_day_start.adding_secs(-86_400 * 365),
                all_time.clone(),
            ),
            (
                None,
                self.next_day_start.adding_secs(-86_400 * 90),
                all_time.clone(),
            ),
            (
                None,
                self.next_day_start.adding_secs(-86_400 * 30),
                all_time.clone(),
            ),
            (
                None,
                self.next_day_start.adding_secs(-86_400 * 7),
                all_time.clone(),
            ),
            (
                Some(self.next_day_start.adding_secs(-86_400 * 1)),
                self.next_day_start.adding_secs(-86_400 * 2),
                all_time.clone(),
            ),
            (
                None,
                self.next_day_start.adding_secs(-86_400 * 1),
                all_time.clone(),
            ),
        ];
        'outer: for review in &self.revlog {
            let Some(interval_bucket) = interval_bucket(review) else {
                continue;
            };
            let Some(button_idx) = button_index(review.button_chosen) else {
                continue;
            };
            let review_secs = review.id.as_secs();
            increment_button_counts(&mut all_time, interval_bucket, button_idx);
            for (minstamp, maxstamp, bucket) in &mut conditional_buckets {
                if &review_secs < maxstamp {
                    continue 'outer;
                }
                let minstamp = minstamp.unwrap_or(TimestampSecs::from(i64::MAX));
                if review_secs < minstamp {
                    increment_button_counts(bucket, interval_bucket, button_idx);
                }
            }
        }
        Buttons {
            today: Some(conditional_buckets.pop().unwrap().2),
            yesterday: Some(conditional_buckets.pop().unwrap().2),
            one_week: Some(conditional_buckets.pop().unwrap().2),
            one_month: Some(conditional_buckets.pop().unwrap().2),
            three_months: Some(conditional_buckets.pop().unwrap().2),
            one_year: Some(conditional_buckets.pop().unwrap().2),
            all_time: Some(all_time),
        }
    }
}

#[derive(Clone, Copy)]
enum IntervalBucket {
    Learning,
    Young,
    Mature,
}

fn increment_button_counts(counts: &mut ButtonCounts, bucket: IntervalBucket, button_idx: usize) {
    match bucket {
        IntervalBucket::Learning => counts.learning[button_idx] += 1,
        IntervalBucket::Young => counts.young[button_idx] += 1,
        IntervalBucket::Mature => counts.mature[button_idx] += 1,
    }
}

fn interval_bucket(review: &RevlogEntry) -> Option<IntervalBucket> {
    match review.review_kind {
        RevlogReviewKind::Learning | RevlogReviewKind::Relearning | RevlogReviewKind::Filtered => {
            Some(IntervalBucket::Learning)
        }
        RevlogReviewKind::Review => Some(if review.last_interval < 21 {
            IntervalBucket::Young
        } else {
            IntervalBucket::Mature
        }),
        RevlogReviewKind::Manual | RevlogReviewKind::Rescheduled => None,
    }
}

fn button_index(button_chosen: u8) -> Option<usize> {
    if (1..=4).contains(&button_chosen) {
        Some((button_chosen - 1) as usize)
    } else {
        None
    }
}
