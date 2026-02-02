use benchx::cli::Config;
use benchx::metrics::Metrics;
use benchx::report::format_summary;

#[test]
fn summary_contains_core_fields() {
    let config = Config {
        command: "echo hi".to_string(),
        iterations: 5,
        concurrency: 1,
        timeout: 1000,
    };

    let metrics = Metrics {
        success_count: 4,
        failure_count: 1,
        min_latency_ms: 10.0,
        max_latency_ms: 50.0,
        avg_latency_ms: 30.0,
        throughput: 20.0,
    };

    let output = format_summary(&config, &metrics);

    // Header
    assert!(output.contains("BenchX Summary"));

    // Core config fields
    assert!(output.contains("Command:      echo hi"));
    assert!(output.contains("Iterations:   5"));

    // Success/failure counts
    assert!(output.contains("Successes:    4"));
    assert!(output.contains("Failures:     1"));

    // Latency numbers (formatted with 3 decimals)
    assert!(output.contains("Min:        10.000"));
    assert!(output.contains("Avg:        30.000"));
    assert!(output.contains("Max:        50.000"));
}