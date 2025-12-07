use std::time::Instant;
use std::process::Command; 

#[derive(Debug)]
pub struct TaskResult {
    pub success: bool,
    pub duration_ms: f64,
}

pub fn run_once(command: &str) -> TaskResult {
    let start = Instant::now();

    let parts: Vec<&str> = command.split_whitespace().collect();

    if parts.is_empty() {
        let duration = start.elapsed();
        let duration_ms = duration.as_secs_f64() * 1000.0;
        return TaskResult {
            success: false,
            duration_ms: duration_ms,
        };
    }

    let program = parts[0];
    let args = &parts[1..];

    let output_result = Command::new(program).args(args).output();

    let success = match output_result {
        Ok(output) => output.status.success(),
        Err(_) => false,
    };

    let duration = start.elapsed();
    let duration_ms = duration.as_secs_f64() * 1000.0;

    TaskResult {
        success: success,
        duration_ms: duration_ms,
    }
}