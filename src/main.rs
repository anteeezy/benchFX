use crate::task::{TaskResult, run_once};
use crate::metrics::compute_metrics;

mod cli;
mod task;
mod metrics;

fn main() {
    let config = cli::parse_args();
    dbg!(&config);

    let mut results: Vec<TaskResult> = Vec::new();

    for _ in 0..config.iterations {
        let  t = run_once(&config.command);
        results.push(t);
    }

    println!("results.len() = {}", results.len());
    for t in &results {
        println!("{:?}", t);
    }

    let metrics = compute_metrics(&results);

    match metrics {
        Some(m) => {
            println!("successes: {}", m.success_count);
            println!("failures: {}", m.failure_count);
            println!("min latency: {:.3} ms", m.min_latency_ms);
            println!("max latency: {:.3} ms", m.max_latency_ms);
            println!("avg latency: {:.3} ms", m.avg_latency_ms);
        }
        None => {
            println!("No successful runs, cannot compute latency stats.");
        }
    }
}
