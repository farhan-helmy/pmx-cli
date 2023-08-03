use clap::Parser;

/// Simple program to show the name of PMX
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Show the name of PMX
    #[arg(short, long)]
    name: bool,
}

fn main() {
    let args = Args::parse();

    if args.name {
        println!("Dato' Seri Anwar Ibrahim");
    }
}
