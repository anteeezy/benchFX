use clap::Parser;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    Pretty,
    Json,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg(long)]
    pub command: String,
    #[arg(long)]
    pub iterations: usize,
    #[arg(long, default_value_t = 1)]
    pub concurrency: usize,
    #[arg(long, default_value_t = 1000)]
    pub timeout: usize,
    #[arg(long, value_enum, default_value_t = OutputFormat::Pretty)]
    pub output: OutputFormat,
}

pub fn parse_args() -> Config {
    Config::parse()
}