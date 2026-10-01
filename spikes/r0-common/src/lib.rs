use std::fmt;
use std::sync::{Mutex, MutexGuard, PoisonError};
use std::time::{Duration, Instant};

pub const REQUIRED_OPENS: usize = 50;
pub const SHORTCUT_TO_PANEL_READY_P95_BUDGET: Duration = Duration::from_millis(150);

const P95_PERCENT: usize = 95;

#[derive(Debug, PartialEq, Eq)]
pub enum Verdict {
    Pass,
    Fail,
    TooFewOpens { opens: usize },
}

pub struct OpenReport {
    pub opens: usize,
    pub p95: Duration,
    pub max: Duration,
    pub verdict: Verdict,
}

#[derive(Default)]
struct OpenState {
    pending: Option<Instant>,
    samples: Vec<Duration>,
}

#[derive(Default)]
pub struct OpenTimer {
    state: Mutex<OpenState>,
}

impl OpenTimer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn shortcut_fired(&self) {
        let mut state = self.lock_state();
        state.pending = Some(Instant::now());
    }

    pub fn first_frame_acked(&self) -> Option<Duration> {
        let mut state = self.lock_state();
        let acked_at = Instant::now();
        let elapsed = acked_at.duration_since(state.pending.take()?);
        state.samples.push(elapsed);
        Some(elapsed)
    }

    pub fn report(&self) -> OpenReport {
        OpenReport::from_samples(&self.lock_state().samples)
    }

    fn lock_state(&self) -> MutexGuard<'_, OpenState> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

impl OpenReport {
    pub fn from_samples(samples: &[Duration]) -> Self {
        let mut sorted = samples.to_vec();
        sorted.sort();
        let opens = sorted.len();
        let p95 = nearest_rank_p95(&sorted);
        Self {
            opens,
            p95,
            max: sorted.last().copied().unwrap_or_default(),
            verdict: verdict(opens, p95),
        }
    }
}

fn nearest_rank_p95(sorted: &[Duration]) -> Duration {
    let rank = (sorted.len() * P95_PERCENT).div_ceil(100);
    rank.checked_sub(1)
        .and_then(|index| sorted.get(index))
        .copied()
        .unwrap_or_default()
}

fn verdict(opens: usize, p95: Duration) -> Verdict {
    if opens < REQUIRED_OPENS {
        return Verdict::TooFewOpens { opens };
    }
    if p95 <= SHORTCUT_TO_PANEL_READY_P95_BUDGET {
        Verdict::Pass
    } else {
        Verdict::Fail
    }
}

impl fmt::Display for Verdict {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Verdict::Pass => write!(formatter, "PASS"),
            Verdict::Fail => write!(formatter, "FAIL"),
            Verdict::TooFewOpens { .. } => {
                write!(formatter, "TOO FEW OPENS (need {REQUIRED_OPENS})")
            }
        }
    }
}

impl fmt::Display for OpenReport {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "shortcut_to_panel_ready opens={} p95={:?} max={:?} budget={:?} {}",
            self.opens, self.p95, self.max, SHORTCUT_TO_PANEL_READY_P95_BUDGET, self.verdict
        )
    }
}
