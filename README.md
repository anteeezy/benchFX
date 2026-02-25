# BenchX

BenchX is a Rust CLI tool for benchmarking arbitrary system commands.

It executes a command repeatedly, measures latency distributions
(min / avg / p50 / p90 / p95 / p99), calculates throughput, and tracks failures.

BenchX is designed for clarity, determinism, and scriptable performance analysis.

---

## Features

- Execute any system command repeatedly
- High-resolution latency measurement
- Percentile reporting (nearest-rank method)
- Throughput calculation (runs/sec)
- Failure tracking
- JSON output for CI and automation

---

## Installation

Once published:

```bash
cargo install benchx
```

Build locally:

```bash
git clone https://github.com/anteeezy/benchx.git
cd benchx
cargo build --release
```

---

## Usage

Basic example:

```bash
cargo run -- --command "curl -s https://api.github.com" --iterations 5
```

```bash
cargo run -- --command "echo hi" --iterations 3
```

JSON output:

```bash
benchx --command "echo hi" --iterations 1000 --output json
```

---

## Example Output

```
========================
       BenchX
========================

Command:      echo hi
Iterations:   3
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

## Design Principles

- Wall-clock measurement for throughput
- Explicit percentile definition (nearest-rank)
- Clear separation of measurement and reporting
- Deterministic, scriptable output

---

## Roadmap

- [x] Throughput calculation
- [x] Percentile metrics
- [ ] JSON output
- [ ] Configurable concurrency
- [ ] Live progress display
- [ ] HTTP benchmarking mode

---

## License
