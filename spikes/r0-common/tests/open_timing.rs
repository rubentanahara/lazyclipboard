use r0_common::{OpenReport, OpenTimer, Verdict};
use std::time::Duration;

fn millis(values: impl Iterator<Item = u64>) -> Vec<Duration> {
    values.map(Duration::from_millis).collect()
}

#[test]
fn p95_of_one_to_fifty_ms_is_the_48th_sample() {
    let report = OpenReport::from_samples(&millis(1..=50));

    assert_eq!(report.p95, Duration::from_millis(48));
}

#[test]
fn p95_of_fifty_opens_with_three_slow_ones_is_slow() {
    let samples = millis(std::iter::repeat_n(10, 47).chain([200, 200, 200]));

    assert_eq!(
        OpenReport::from_samples(&samples).p95,
        Duration::from_millis(200)
    );
}

#[test]
fn p95_of_fifty_opens_ignores_one_outlier() {
    let samples = millis(std::iter::repeat_n(10, 49).chain([500]));

    assert_eq!(
        OpenReport::from_samples(&samples).p95,
        Duration::from_millis(10)
    );
}

#[test]
fn fifty_opens_at_150_ms_pass() {
    let report = OpenReport::from_samples(&millis(std::iter::repeat_n(150, 50)));

    assert_eq!(report.verdict, Verdict::Pass);
}

#[test]
fn fifty_opens_at_151_ms_fail() {
    let report = OpenReport::from_samples(&millis(std::iter::repeat_n(151, 50)));

    assert_eq!(report.verdict, Verdict::Fail);
}

#[test]
fn forty_nine_opens_are_too_few_whatever_their_speed() {
    let report = OpenReport::from_samples(&millis(std::iter::repeat_n(1, 49)));

    assert_eq!(report.verdict, Verdict::TooFewOpens { opens: 49 });
}

#[test]
fn no_opens_are_too_few() {
    let report = OpenReport::from_samples(&[]);

    assert_eq!(report.verdict, Verdict::TooFewOpens { opens: 0 });
}

#[test]
fn report_counts_opens_and_keeps_the_slowest() {
    let report = OpenReport::from_samples(&millis(1..=50));

    assert_eq!(report.opens, 50);
    assert_eq!(report.max, Duration::from_millis(50));
}

#[test]
fn an_ack_after_a_shortcut_records_one_open() {
    let timer = OpenTimer::new();

    timer.shortcut_fired();

    assert!(timer.first_frame_acked().is_some());
    assert_eq!(timer.report().opens, 1);
}

#[test]
fn an_ack_without_a_shortcut_records_nothing() {
    let timer = OpenTimer::new();

    assert!(timer.first_frame_acked().is_none());
    assert_eq!(timer.report().opens, 0);
}

#[test]
fn a_second_ack_for_the_same_open_records_nothing() {
    let timer = OpenTimer::new();
    timer.shortcut_fired();
    timer.first_frame_acked();

    assert!(timer.first_frame_acked().is_none());
    assert_eq!(timer.report().opens, 1);
}

#[test]
fn a_shortcut_that_never_acks_is_not_counted() {
    let timer = OpenTimer::new();
    timer.shortcut_fired();
    timer.shortcut_fired();
    timer.first_frame_acked();

    assert_eq!(timer.report().opens, 1);
}

#[test]
fn the_timer_can_live_in_shared_app_state() {
    fn assert_send_sync<T: Send + Sync>() {}

    assert_send_sync::<OpenTimer>();
}

#[test]
fn a_passing_report_prints_as_one_line() {
    let report = OpenReport::from_samples(&millis(1..=50));

    assert_eq!(
        report.to_string(),
        "shortcut_to_panel_ready opens=50 p95=48ms max=50ms budget=150ms PASS"
    );
}

#[test]
fn a_report_with_too_few_opens_says_how_many_are_needed() {
    let report = OpenReport::from_samples(&millis(1..=10));

    assert_eq!(
        report.to_string(),
        "shortcut_to_panel_ready opens=10 p95=10ms max=10ms budget=150ms TOO FEW OPENS (need 50)"
    );
}

#[test]
fn a_default_timer_records_opens_like_a_new_one() {
    let timer = OpenTimer::default();

    timer.shortcut_fired();
    timer.first_frame_acked();

    assert_eq!(timer.report().opens, 1);
}
