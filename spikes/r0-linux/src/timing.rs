use std::fmt;
use std::sync::Mutex;
use std::time::{Duration, Instant};

pub const REQUIRED_OPENS: usize = 50;
pub const SHORTCUT_TO_PANEL_READY_P95_BUDGET: Duration = Duration::from_millis(150);
const P95_PERCENT: usize = 95;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    Pass,
    Fail,
    TooFewOpens { opens: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpenReport {
    pub opens: usize,
    pub p95: Duration,
    pub max: Duration,
    pub verdict: Verdict,
}

impl OpenReport {
    pub fn from_samples(samples: &[Duration]) -> Self {
        let mut sorted = samples.to_vec();
        sorted.sort();
        let opens = sorted.len();
        let nearest_rank = (opens * P95_PERCENT).div_ceil(100);
        let p95 = nearest_rank
            .checked_sub(1)
            .map_or(Duration::ZERO, |index| sorted[index]);
        let max = sorted.last().copied().unwrap_or(Duration::ZERO);
        Self {
            opens,
            p95,
            max,
            verdict: verdict_for(opens, p95),
        }
    }
}

fn verdict_for(opens: usize, p95: Duration) -> Verdict {
    if opens < REQUIRED_OPENS {
        Verdict::TooFewOpens { opens }
    } else if p95 <= SHORTCUT_TO_PANEL_READY_P95_BUDGET {
        Verdict::Pass
    } else {
        Verdict::Fail
    }
}

#[derive(Default)]
struct Opens {
    pending: Option<Instant>,
    samples: Vec<Duration>,
}

#[derive(Default)]
pub struct OpenTimer {
    opens: Mutex<Opens>,
}

impl OpenTimer {
    pub fn shortcut_fired(&self) {
        self.opens.lock().unwrap().pending = Some(Instant::now());
    }

    pub fn first_frame_acked(&self) -> Option<Duration> {
        let mut opens = self.opens.lock().unwrap();
        let elapsed = opens.pending.take()?.elapsed();
        opens.samples.push(elapsed);
        Some(elapsed)
    }

    pub fn report(&self) -> OpenReport {
        OpenReport::from_samples(&self.opens.lock().unwrap().samples)
    }
}

impl fmt::Display for OpenReport {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "opens={} p95={}ms max={}ms verdict={:?}",
            self.opens,
            self.p95.as_millis(),
            self.max.as_millis(),
            self.verdict
        )
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    fn millis(values: impl IntoIterator<Item = u64>) -> Vec<Duration> {
        values.into_iter().map(Duration::from_millis).collect()
    }

    #[test]
    fn p95_is_the_nearest_rank_sample() {
        let report = OpenReport::from_samples(&millis(1..=50));

        assert_eq!(report.opens, 50);
        assert_eq!(report.p95, Duration::from_millis(48));
        assert_eq!(report.max, Duration::from_millis(50));
    }

    #[test]
    fn a_p95_at_the_budget_passes() {
        let report = OpenReport::from_samples(&millis([150; 50]));

        assert_eq!(report.verdict, Verdict::Pass);
    }

    #[test]
    fn a_p95_over_the_budget_fails() {
        let report = OpenReport::from_samples(&millis([151; 50]));

        assert_eq!(report.verdict, Verdict::Fail);
    }

    #[test]
    fn fewer_than_the_required_opens_has_no_verdict() {
        let report = OpenReport::from_samples(&millis([1; 49]));

        assert_eq!(report.verdict, Verdict::TooFewOpens { opens: 49 });
    }

    #[test]
    fn an_ack_without_a_shortcut_records_nothing() {
        let timer = OpenTimer::default();

        assert_eq!(timer.first_frame_acked(), None);
        assert_eq!(timer.report().opens, 0);
    }

    #[test]
    fn a_shortcut_then_an_ack_records_one_open() {
        let timer = OpenTimer::default();

        timer.shortcut_fired();
        let elapsed = timer.first_frame_acked();

        assert!(elapsed.is_some());
        assert_eq!(timer.report().opens, 1);
    }

    #[test]
    fn a_second_shortcut_before_an_ack_drops_the_first() {
        let timer = OpenTimer::default();

        timer.shortcut_fired();
        timer.shortcut_fired();
        timer.first_frame_acked();

        assert_eq!(timer.report().opens, 1);
    }
}
