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

Throughput:   {throughput}

Latency (ms)
  Min:        {min:.3}
  Avg:        {avg:.3}
  Max:        {max:.3}

========================
",
        command = config.command,
        iterations = config.iterations,
        successes = metrics.success_count,
        failures = metrics.failure_count,
        throughput = metrics.throughput,
        min = metrics.min_latency_ms,
        avg = metrics.avg_latency_ms,
        max = metrics.max_latency_ms,
    )
}

/// Print the summary to stdout (thin wrapper).
pub fn print_summary(config: &Config, metrics: &Metrics) {
    print!("{}", format_summary(config, metrics));
}
