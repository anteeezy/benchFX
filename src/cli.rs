use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    command: String,
    #[arg(long)]
    iterations: usize,
    #[arg(long, default_value_t = 1)]
    concurrency: usize,
    #[arg(long, default_value_t = 1000)]
    timeout: usize,
}

#[derive(Debug)]
pub struct Config {
    pub command: String,
    pub iterations: usize,
    pub concurrency: usize,
    pub timeout: usize,
}

pub fn parse_args() -> Config {
    let args = Args::parse();

    Config {
        command: args.command,
        iterations: args.iterations,
        concurrency: args.concurrency,
        timeout: args.timeout,
    }
}