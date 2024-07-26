use std::env;
use knapsack::problem::{evaluate_solution, read_instance};
use knapsack::search::random_search;

fn main() {
    let args: Vec<String> = env::args().collect();

    let instance = match read_instance(&args[1]) {
        Ok(data) => data,
        Err(_) => panic!("Error while reading instance file"),
    };

    println!("INSTANCE\n{}\n{:?}", args[1], instance);

    let best = random_search(&instance);

    println!("SOLUTION {} {:?}", evaluate_solution(&best, &instance), best);
}
