use clap::Parser;
use std::time::Instant;
use knapsack::problem::read_instance;
use knapsack::search::{random_search, hill_climbing, genetic_algorithm};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    algorithm: String,
    #[arg(short, long)]
    instance_file: String,
    #[arg(short, long)]
    verbose: bool
}

fn main() {
    let args = Args::parse();

    if args.verbose {
        println!("ALGORITHM {}", args.algorithm);
        println!("INSTANCE {}", args.instance_file);
    }

    let instance = match read_instance(&args.instance_file) {
        Ok(data) => data,
        Err(_) => panic!("Error while reading instance file"),
    };


    let max_it = 1e7 as usize;
    let now = Instant::now();
    let (_, best_profit) = match args.algorithm.as_str() {
        "RS" => random_search(&instance, max_it),
        "HC" => hill_climbing(&instance, 0.5, max_it),
        "GA" => genetic_algorithm(&instance, 100, 0.1, 0.8, max_it),
        _ => panic!("Unknown algorithm! Exiting..."),
    };

    println!("{:?} {}", now.elapsed().as_nanos(), best_profit);
}
