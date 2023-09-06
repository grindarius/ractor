use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    numbers: Vec<u64>,
    /// Whether to print the factors as exponents (p^e) form
    #[arg(short, long)]
    exponents: bool,
}

fn main() {
    let args = Arguments::parse();

    println!("numbers: {:?}", args.numbers);
}
