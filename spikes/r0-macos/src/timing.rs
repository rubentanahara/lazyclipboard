use std::sync::Mutex;
use std::time::{Duration, Instant};

pub const REQUIRED_OPENS: usize = 50;
pub const PANEL_READY_P95_BUDGET: Duration = Duration::from_millis(150);

#[derive(Default)]
pub struct OpenTimer {
    pending: Mutex<Option<Instant>>,
    samples: Mutex<Vec<Duration>>,
}

#[derive(Debug, PartialEq)]
pub struct OpenReport {
    pub opens: usize,
    pub p95: Duration,
    pub max: Duration,
}

impl OpenTimer {
    pub fn shortcut_fired(&self) {
        *self.pending.lock().expect("pending lock") = Some(Instant::now());
    }

    pub fn first_frame_acked(&self) -> Option<Duration> {
        let started = self.pending.lock().expect("pending lock").take()?;
        let elapsed = started.elapsed();
        self.samples.lock().expect("samples lock").push(elapsed);
        Some(elapsed)
    }

    pub fn report(&self) -> OpenReport {
        OpenReport::from_samples(&self.samples.lock().expect("samples lock"))
    }
}

impl OpenReport {
    pub fn from_samples(samples: &[Duration]) -> Self {
        let mut sorted = samples.to_vec();
        sorted.sort();
        let nearest_rank = (sorted.len() * 95).div_ceil(100).saturating_sub(1);
        Self {
            opens: sorted.len(),
            p95: sorted.get(nearest_rank).copied().unwrap_or_default(),
            max: sorted.last().copied().unwrap_or_default(),
        }
    }

    pub fn passes(&self) -> bool {
        self.opens >= REQUIRED_OPENS && self.p95 <= PANEL_READY_P95_BUDGET
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn millis(values: impl IntoIterator<Item = u64>) -> Vec<Duration> {
        values.into_iter().map(Duration::from_millis).collect()
    }

    #[test]
    fn p95_of_fifty_samples_is_the_forty_eighth_by_nearest_rank() {
        let report = OpenReport::from_samples(&millis(1..=50));
        assert_eq!(report.p95, Duration::from_millis(48));
        assert_eq!(report.max, Duration::from_millis(50));
    }

    #[test]
    fn a_p95_of_exactly_the_budget_passes_and_one_millisecond_over_fails() {
        let at_budget = millis(std::iter::repeat_n(150, 50));
        let over_budget = millis(std::iter::repeat_n(151, 50));
        assert!(OpenReport::from_samples(&at_budget).passes());
        assert!(!OpenReport::from_samples(&over_budget).passes());
    }

    #[test]
    fn fewer_than_the_required_opens_never_passes() {
        assert!(!OpenReport::from_samples(&millis([10; 49])).passes());
    }

    #[test]
    fn an_ack_without_a_pending_shortcut_records_nothing() {
        let timer = OpenTimer::default();
        assert_eq!(timer.first_frame_acked(), None);
        assert_eq!(timer.report().opens, 0);
    }
}
