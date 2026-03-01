use benchfx::task::{run_concurrent, run_sequential};

fn ok_command() -> &'static str {
    // Use a command that is fast and produces no output.
    #[cfg(windows)]
    { "cmd /c exit 0" }

    #[cfg(not(windows))]
    { "true" }
}

#[test]
fn concurrent_returns_exact_number_of_results() {
    let cmd = ok_command();
    let iterations = 7;
    let concurrency = 3;

    let results = run_concurrent(cmd, iterations, concurrency);

    assert_eq!(results.len(), iterations);
}

#[test]
fn concurrent_when_concurrency_exceeds_iterations_still_returns_all_results() {
    let cmd = ok_command();
    let iterations = 5;
    let concurrency = 32; // should cap workers to iterations

    let results = run_concurrent(cmd, iterations, concurrency);

    assert_eq!(results.len(), iterations);
}

#[test]
fn concurrent_with_concurrency_1_matches_sequential_len() {
    let cmd = ok_command();
    let iterations = 10;

    let seq = run_sequential(cmd, iterations);
    let conc = run_concurrent(cmd, iterations, 1);

    assert_eq!(seq.len(), iterations);
    assert_eq!(conc.len(), iterations);
}

#[test]
fn concurrent_with_iterations_0_returns_empty() {
    let cmd = ok_command();

    let results = run_concurrent(cmd, 0, 8);

    assert!(results.is_empty());
}