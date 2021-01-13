use clap::Clap;
use sqml::echo;

#[derive(Clap)]
#[clap(version = env!("CARGO_PKG_VERSION"), author=env!("CARGO_PKG_AUTHORS"))]
struct Opts {
    query: String,
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("query: {}", echo(&opts.query));
}
