#[allow(dead_code)]
use std::time::Instant;
use std::process::Command; 
use std::thread; 

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
            duration_ms,
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
        success,
        duration_ms,
    }
}

pub fn run_sequential(command: &str, iterations: usize) -> Vec<TaskResult> {
    let mut results: Vec<TaskResult> = Vec::with_capacity(iterations);
    for _ in 0..iterations {
        let  t = run_once(command);
        results.push(t);
    }
    results
}

pub fn run_concurrent(command: &str, iterations: usize, concurrency: usize) -> Vec<TaskResult> {
    if concurrency <= 1 || iterations <= 1 {
        return run_sequential(command, iterations);
    }

    let workers= iterations.min(concurrency);

    let base = iterations / workers; 
    let remainder = iterations % workers; 

    let mut handles: Vec<thread::JoinHandle<Vec<TaskResult>>> = Vec::with_capacity(workers); 

    for index in 0..workers {
        let my_iters = base + if index < remainder { 1 } else { 0 };

        let cmd_string = command.to_string(); 

        let handle = thread::spawn(move || {
            let mut local_results = Vec::with_capacity(my_iters);
            for _ in 0..my_iters {
                let t = run_once(&cmd_string);
                local_results.push(t);
            }
            local_results
        });
        handles.push(handle);
    }

    let mut joined_results = Vec::with_capacity(iterations);
    
    for handle in handles {
        let local = handle.join().expect("worker thread panicked");
        joined_results.extend(local);
    }

    joined_results
}