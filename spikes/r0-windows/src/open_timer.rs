use std::sync::Mutex;
use std::time::{Duration, Instant};

pub const REQUIRED_OPENS: usize = 50;
pub const READY_P95_BUDGET: Duration = Duration::from_millis(150);

#[derive(Default)]
pub struct OpenTimer {
    state: Mutex<State>,
}

#[derive(Default)]
struct State {
    fired_at: Option<Instant>,
    samples: Vec<Duration>,
}

impl OpenTimer {
    pub fn shortcut_fired(&self) {
        self.state().fired_at = Some(Instant::now());
    }

    pub fn first_frame_acked(&self) -> Option<Duration> {
        let mut state = self.state();
        let elapsed = state.fired_at.take()?.elapsed();
        state.samples.push(elapsed);
        Some(elapsed)
    }

    pub fn summary(&self) -> String {
        let state = self.state();
        let Some(p95) = nearest_rank_p95(&state.samples) else {
            return "opens=0".to_owned();
        };
        let verdict = if state.samples.len() < REQUIRED_OPENS {
            format!("INCOMPLETE (need {REQUIRED_OPENS})")
        } else if p95 <= READY_P95_BUDGET {
            "PASS".to_owned()
        } else {
            "FAIL".to_owned()
        };
        format!(
            "opens={} p95_ms={:.1} budget_ms={} {verdict}",
            state.samples.len(),
            p95.as_secs_f64() * 1000.0,
            READY_P95_BUDGET.as_millis(),
        )
    }

    fn state(&self) -> std::sync::MutexGuard<'_, State> {
        self.state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

fn nearest_rank_p95(samples: &[Duration]) -> Option<Duration> {
    let mut sorted = samples.to_vec();
    sorted.sort();
    let rank = (sorted.len() * 95).div_ceil(100);
    rank.checked_sub(1).map(|index| sorted[index])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn millis(values: impl IntoIterator<Item = u64>) -> Vec<Duration> {
        values.into_iter().map(Duration::from_millis).collect()
    }

    #[test]
    fn p95_is_the_nearest_rank_sample() {
        assert_eq!(
            nearest_rank_p95(&millis(1..=100)),
            Some(Duration::from_millis(95))
        );
        assert_eq!(
            nearest_rank_p95(&millis(1..=20)),
            Some(Duration::from_millis(19))
        );
        assert_eq!(
            nearest_rank_p95(&millis((1..=50).rev())),
            Some(Duration::from_millis(48))
        );
    }

    #[test]
    fn p95_of_no_samples_is_none() {
        assert_eq!(nearest_rank_p95(&[]), None);
    }

    #[test]
    fn ack_without_a_shortcut_records_nothing() {
        let timer = OpenTimer::default();

        assert_eq!(timer.first_frame_acked(), None);
        assert_eq!(timer.summary(), "opens=0");
    }

    #[test]
    fn one_shortcut_and_one_ack_record_one_open() {
        let timer = OpenTimer::default();

        timer.shortcut_fired();
        let elapsed = timer.first_frame_acked();

        assert!(elapsed.is_some());
        assert!(
            timer.summary().starts_with("opens=1 "),
            "{}",
            timer.summary()
        );
        assert_eq!(timer.first_frame_acked(), None);
        assert!(
            timer.summary().starts_with("opens=1 "),
            "{}",
            timer.summary()
        );
    }

    #[test]
    fn a_shortcut_without_an_ack_is_dropped_by_the_next_shortcut() {
        let timer = OpenTimer::default();

        timer.shortcut_fired();
        timer.shortcut_fired();
        timer.first_frame_acked();

        assert!(
            timer.summary().starts_with("opens=1 "),
            "{}",
            timer.summary()
        );
    }

    #[test]
    fn summary_is_incomplete_below_the_required_opens() {
        let timer = OpenTimer::default();

        timer.shortcut_fired();
        timer.first_frame_acked();

        assert!(
            timer.summary().contains("INCOMPLETE"),
            "{}",
            timer.summary()
        );
    }
}
