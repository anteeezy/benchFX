use benchx::cli::{Config, OutputFormat};
use benchx::metrics::Metrics;
use benchx::report::{format_summary, format_json};

#[test]
fn summary_contains_core_fields() {
    let config = Config {
        command: "echo hi".to_string(),
        iterations: 5,
        concurrency: 1,
        timeout: 1000,
        output: OutputFormat::Pretty,
    };

    let metrics = Metrics {
        success_count: 4,
        failure_count: 1,
        min_latency_ms: 10.0,
        max_latency_ms: 50.0,
        avg_latency_ms: 30.0,
        throughput: 20.0,
        p50_latency_ms: 31.0,
        p90_latency_ms: 40.0,
        p95_latency_ms: 45.0,
        p99_latency_ms: 50.0,
    };

    let output = format_summary(&config, &metrics);

    // Header (allow either style as you iterate)
    assert!(output.contains("BenchX"));

    // Core config fields
    assert!(output.contains("Command:"));
    assert!(output.contains("echo hi"));
    assert!(output.contains("Iterations:"));
    assert!(output.contains("5"));

    // Success/failure counts
    assert!(output.contains("Successes:"));
    assert!(output.contains("4"));
    assert!(output.contains("Failures:"));
    assert!(output.contains("1"));

    // Latency block (avoid exact spacing)
    assert!(output.contains("Latency"));
    assert!(output.contains("min:"));
    assert!(output.contains("10.000"));
    assert!(output.contains("avg:"));
    assert!(output.contains("30.000"));
    assert!(output.contains("max:"));
    assert!(output.contains("50.000"));

    // Percentiles
    assert!(output.contains("p50:"));
    assert!(output.contains("31.000"));
    assert!(output.contains("p90:"));
    assert!(output.contains("40.000"));
    assert!(output.contains("p95:"));
    assert!(output.contains("45.000"));
    assert!(output.contains("p99:"));
    assert!(output.contains("50.000"));

    // Throughput (if you print it)
    assert!(output.contains("Throughput:"));
    // if you format with 2 decimals + units:
    // assert!(output.contains("20.00"));
    // assert!(output.contains("runs/sec"));
}

use serde_json::Value;

#[test]
fn json_contains_expected_fields_and_values() {
    let config = Config {
        command: "echo hi".to_string(),
        iterations: 5,
        concurrency: 1,
        timeout: 1000,
        output: OutputFormat::Json,
    };

    let metrics = Metrics {
        success_count: 4,
        failure_count: 1,
        min_latency_ms: 10.0,
        max_latency_ms: 50.0,
        avg_latency_ms: 30.0,
        throughput: 20.0,
        p50_latency_ms: 31.0,
        p90_latency_ms: 40.0,
        p95_latency_ms: 45.0,
        p99_latency_ms: 50.0,
    };

    let json_str = format_json(&config, &metrics);

    let v: Value = serde_json::from_str(&json_str).expect("JSON should parse");

    // target.command
    assert_eq!(v["target"]["command"], "echo hi");

    // top-level counts
    assert_eq!(v["iterations"], 5);
    assert_eq!(v["successes"], 4);
    assert_eq!(v["failures"], 1);

    // throughput
    let thr = v["throughput_runs_per_sec"]
        .as_f64()
        .expect("throughput_runs_per_sec should be a number");
    assert!((thr - 20.0).abs() < 1e-9);

    // latency_ms fields
    assert!((v["latency_ms"]["min"].as_f64().unwrap() - 10.0).abs() < 1e-9);
    assert!((v["latency_ms"]["avg"].as_f64().unwrap() - 30.0).abs() < 1e-9);
    assert!((v["latency_ms"]["p50"].as_f64().unwrap() - 31.0).abs() < 1e-9);
    assert!((v["latency_ms"]["p90"].as_f64().unwrap() - 40.0).abs() < 1e-9);
    assert!((v["latency_ms"]["p95"].as_f64().unwrap() - 45.0).abs() < 1e-9);
    assert!((v["latency_ms"]["p99"].as_f64().unwrap() - 50.0).abs() < 1e-9);
    assert!((v["latency_ms"]["max"].as_f64().unwrap() - 50.0).abs() < 1e-9);
}
