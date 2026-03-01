use crate::task::{TaskResult, run_sequential, run_concurrent};
use crate::metrics::compute_metrics;
use crate::report::print_summary;
use crate::report::output_json;
use std::time::{Instant};

mod cli;
mod task;
mod metrics;
mod report;

fn main() {
    let config = cli::parse_args();

    let now = Instant::now();
    
    let results: Vec<TaskResult> = if config.concurrency > 1 {
        run_concurrent(&config.command, config.iterations, config.concurrency)
    } else {
        run_sequential(&config.command, config.iterations)
    };
    
    let duration = now.elapsed();
    let duration_sec = duration.as_secs_f64();
    let throughput = results.len() as f64 / duration_sec;
    let Some(m) = compute_metrics(&results, throughput) else {
        eprintln!("No successful runs — cannot compute metrics.");
        std::process::exit(1);
    };

    match &config.output {
        cli::OutputFormat::Pretty => {
            print_summary(&config, &m);
        },
        cli::OutputFormat::Json => {
            output_json(&config, &m);
        },
    }
}