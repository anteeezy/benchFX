use benchx::metrics::compute_metrics;
use benchx::task::TaskResult;

#[test]
fn metrics_all_success() {
    let results = vec![
        TaskResult { success: true, duration_ms: 100.0 },
        TaskResult { success: true, duration_ms: 200.0 },
        TaskResult { success: true, duration_ms: 300.0 },
    ];
    let throughput: f64 = 10.0;

    let m = compute_metrics(&results, throughput).expect("expected metrics");

    assert_eq!(m.success_count, 3);
    assert_eq!(m.failure_count, 0);
    assert_eq!(m.min_latency_ms, 100.0);
    assert_eq!(m.max_latency_ms, 300.0);
    assert!((m.avg_latency_ms - 200.0).abs() < 1e-6);
    assert_eq!(m.p50_latency_ms, 200.0);
    assert_eq!(m.p90_latency_ms, 300.0);
    assert_eq!(m.p95_latency_ms, 300.0);
    assert_eq!(m.p99_latency_ms, 300.0);
    assert_eq!(m.throughput, throughput);
}

#[test]
fn metrics_mixed_success_and_failure() {
    let results = vec![
        TaskResult { success: true,  duration_ms: 100.0 },
        TaskResult { success: false, duration_ms: 999.0 },
        TaskResult { success: true,  duration_ms: 300.0 },
    ];
    let throughput: f64 = 10.0;

    let m = compute_metrics(&results, throughput).expect("expected metrics");

    assert_eq!(m.success_count, 2);
    assert_eq!(m.failure_count, 1);
    assert_eq!(m.min_latency_ms, 100.0);
    assert_eq!(m.max_latency_ms, 300.0);
    assert!((m.avg_latency_ms - 200.0).abs() < 1e-6);
    assert_eq!(m.p50_latency_ms, 100.0);
    assert_eq!(m.p90_latency_ms, 300.0);
    assert_eq!(m.p95_latency_ms, 300.0);
    assert_eq!(m.p99_latency_ms, 300.0);
    assert_eq!(m.throughput, throughput);
}

#[test]
fn metrics_no_success_returns_none() {
    let results = vec![
        TaskResult { success: false, duration_ms: 100.0 },
        TaskResult { success: false, duration_ms: 200.0 },
    ];
    let throughput: f64 = 10.0;

    let m = compute_metrics(&results, throughput);
    assert!(m.is_none());
}

#[test]
fn metrics_empty_results_returns_none() {
    let results: Vec<TaskResult> = Vec::new();
    let throughput: f64 = 10.0;

    let m = compute_metrics(&results, throughput);
    assert!(m.is_none());
}

#[test]
fn metrics_percentiles_even_n_nearest_rank_behavior() {
    let results = vec![
        TaskResult { success: true, duration_ms: 10.0 },
        TaskResult { success: true, duration_ms: 20.0 },
        TaskResult { success: true, duration_ms: 30.0 },
        TaskResult { success: true, duration_ms: 40.0 },
    ];
    let m = compute_metrics(&results, 1.0).unwrap();

    // nearest-rank p50: ceil(0.5 * 4) - 1 = 1 -> 20.0
    assert_eq!(m.p50_latency_ms, 20.0);
}