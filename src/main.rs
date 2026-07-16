use clap::Parser;
use xcheck::{Cli, Command, graph, plain};

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let from = args.currencies.from.to_uppercase();
    let to = args.currencies.to.to_uppercase();

    match args.command {
        Some(Command::Stat { days }) => graph(days, &from, &to),
        None => plain(&from, &to),
    }
}
