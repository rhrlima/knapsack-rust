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
    instance_file: String
}

fn main() {
    let args = Args::parse();

    println!("ARGS {:?}", args);

    let instance = match read_instance(&args.instance_file) {
        Ok(data) => data,
        Err(_) => panic!("Error while reading instance file"),
    };

    let now = Instant::now(); 
    match args.algorithm.as_str() {
        "RS" => random_search(&instance),
        "HC" => hill_climbing(&instance, 0.5),
        "GA" => genetic_algorithm(&instance, 100, 0.1, 0.8),
        _ => panic!("Unknown algorithm! Exiting..."),
    };
    println!("{} {:?}", args.algorithm, now.elapsed().as_nanos());
}
