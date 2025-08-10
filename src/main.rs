use clap::Parser;

#[derive(Parser)]
#[clap(name = "myapp", version, about)]
struct Args {
    #[clap(short, long)]
    name: String,

    #[clap(short, long, default_value_t = 1)]
    count: u32,
}

fn main() {
    let args = Args::parse();
    for _ in 0..args.count {
        println!("Hello, {}!", args.name);
    }
}