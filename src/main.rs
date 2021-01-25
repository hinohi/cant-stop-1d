use clap::Clap;

#[derive(Debug, Clap)]
struct Opts {
    #[clap(long, default_value = "1")]
    dice_min: usize,
    #[clap(short, long, default_value = "6")]
    dice_max: usize,
    #[clap(short, long)]
    goal: usize,
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
}
