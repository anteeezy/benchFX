use crate::cli::Config;
use crate::metrics::Metrics;

/// Build the summary as a string (easy to test).
pub fn format_summary(config: &Config, metrics: &Metrics) -> String {
    format!(
        "\
========================
      BenchX Summary     
========================

Command:      {command}
Iterations:   {iterations}
Successes:    {successes}
Failures:     {failures}

Throughput:   {throughput:.2} runs/sec

Latency (ms)
  min:        {min:.3}
  avg:        {avg:.3}
  max:        {max:.3}
  
  p50:        {p50:.3}
  p90:        {p90:.3}
  p95:        {p95:.3}
  p99:        {p99:.3}

========================
",
        command = config.command,
        iterations = config.iterations,
        successes = metrics.success_count,
        failures = metrics.failure_count,
        throughput = metrics.throughput,
        min = metrics.min_latency_ms,
        avg = metrics.avg_latency_ms,
        p50 = metrics.p50_latency_ms,
        p90 = metrics.p90_latency_ms,
        p95 = metrics.p95_latency_ms,
        p99 = metrics.p99_latency_ms,
        max = metrics.max_latency_ms,
    )
}

/// Print the summary to stdout (thin wrapper).
pub fn print_summary(config: &Config, metrics: &Metrics) {
    print!("{}", format_summary(config, metrics));
}
