![CI](https://github.com/anteeezy/benchfx/actions/workflows/rust.yml/badge.svg)
![Rust](https://img.shields.io/badge/rust-stable-orange.svg)

# BenchFX

BenchFX is a Rust CLI tool for benchmarking arbitrary system commands.

It executes a command repeatedly, measures latency distributions  
(min / avg / p50 / p90 / p95 / p99), calculates throughput, and tracks failures.

BenchFX is designed for clarity, determinism, and scriptable performance analysis.

---

## Features

- Execute any system command repeatedly
- High-resolution latency measurement
- Percentile reporting (nearest-rank method)
- Throughput calculation (runs/sec)
- Concurrent execution with configurable worker count
- Failure tracking
- JSON output for CI and automation

---

## Installation

Once published:

```bash
cargo install benchfx
```

Build locally:

```bash
git clone https://github.com/anteeezy/benchfx.git
cd benchfx
cargo build --release
```

---

## Usage

```bash
benchfx --command <COMMAND> --iterations <N> [OPTIONS]
```

### Required Arguments

```bash
--command <COMMAND>        Command to execute (wrap in quotes)
--iterations <N>           Total number of executions
```

### Optional Arguments

```bash
--concurrency <N>          Maximum concurrent workers (default: 1)
--timeout <MS>             Timeout per execution in milliseconds (default: 1000)
--output <FORMAT>          Output format: pretty | json (default: pretty)
-h, --help                 Print help information
```

---

### Concurrency Semantics

`--concurrency N` specifies the maximum number of concurrent workers.  
Total iterations remain constant and are evenly distributed across workers.

Example:

```bash
benchfx --command "echo hi" --iterations 100 --concurrency 4
```

Runs 100 total executions with up to 4 concurrent workers.

---

## Example Output (Pretty)

```
========================
       BenchFX
========================

Command:      echo hi
Iterations:   3
Concurrency:  1
Successes:    3
Failures:     0

Throughput:   59.14 runs/sec

Latency (ms):
  min:        16.472
  avg:        16.907
  p50:        16.527
  p90:        17.722
  p95:        17.722
  p99:        17.722
  max:        17.722

========================
```

---

## Example Output (JSON)

```json
{
  "target": {
    "command": "echo hi"
  },
  "iterations": 3,
  "concurrency": 1,
  "successes": 3,
  "failures": 0,
  "throughput_runs_per_sec": 58.418,
  "latency_ms": {
    "min": 16.356,
    "avg": 17.115,
    "p50": 16.634,
    "p90": 18.355,
    "p95": 18.355,
    "p99": 18.355,
    "max": 18.355
  }
}
```

---

## Design Principles

- Wall-clock measurement for throughput
- Explicit percentile definition (nearest-rank)
- Clear separation of measurement and reporting
- Deterministic, scriptable output
- Minimal overhead and predictable behavior

---

## Roadmap

- [x] Throughput calculation
- [x] Percentile metrics
- [x] JSON output
- [x] Configurable concurrency
- [ ] Live progress display
- [ ] HTTP benchmarking mode
- [ ] Duration-based execution mode

---

## License

MIT (recommended) or your license of choice.
