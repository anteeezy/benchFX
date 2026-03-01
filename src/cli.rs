use clap::Parser;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    Pretty,
    Json,
}

#[derive(Parser, Debug)]
#[command(name = "benchFX")]
#[command(about = "Benchmark arbitrary system commands with latency and throughput metrics")]
pub struct Config {
    #[arg(long, help = "Command to execute (wrap in quotes)")]
    pub command: String,
    #[arg(long, help = "Total number of executions")]
    pub iterations: usize,
    #[arg(long, default_value_t = 1, help = "Maximum concurrent workers")]
    pub concurrency: usize,
    #[arg(long, default_value_t = 1000, help = "Timeout per execution in milliseconds")]
    pub timeout: usize,
    #[arg(long, value_enum, default_value_t = OutputFormat::Pretty, help = "Output format: pretty (default) or json")]
    pub output: OutputFormat,
}

pub fn parse_args() -> Config {
    Config::parse()
}