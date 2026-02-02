use crate::task::TaskResult;

#[derive(Debug)]
pub struct Metrics {
    pub success_count: usize,
    pub failure_count: usize,
    pub min_latency_ms: f64,
    pub max_latency_ms: f64,
    pub avg_latency_ms: f64,
    pub throughput: f64,
}

pub fn compute_metrics(results: &[TaskResult], throughput: f64) -> Option<Metrics> {
    let success_count = results.iter().filter(|r| r.success).count();
    let failure_count = results.len() - success_count;

    let mut latencies = Vec::new();
    for r in results {
        if r.success {
            latencies.push(r.duration_ms);
        }
    }

    if latencies.is_empty() {
        return None;
    }

    let mut min_latency = latencies[0];
    let mut max_latency = latencies[0];
    let mut sum = 0.0;

    for &lat in &latencies {
        if lat < min_latency {
            min_latency = lat;
        }
        if lat > max_latency {
            max_latency = lat;
        }
        sum += lat;
    }

    let avg_latency = sum / latencies.len() as f64;

    Some(Metrics {
        success_count,
        failure_count,
        min_latency_ms: min_latency,
        max_latency_ms: max_latency,
        avg_latency_ms: avg_latency,
        throughput
    })
}