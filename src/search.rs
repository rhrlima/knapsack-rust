use rand::Rng;
use crate::problem::{evaluate_solution, generate_solution, Instance};

pub fn random_search(instance: &Instance) -> Vec<usize> {

    let mut best = vec![0; instance.num_items as usize];
    let mut best_profit = 0.0;

    let mut count = 0;
    while best_profit < instance.optimal {
        let solution = generate_solution(instance.num_items as usize);
        let profit = evaluate_solution(&solution, instance);

        println!("{:3} {:3} {:?}", count, profit, solution);

        if profit > best_profit {
            best_profit = profit;
            best = solution;
        }

        count += 1;
    }

    best
}

#[allow(dead_code)]
fn one_bit_mutation(solution: &Vec<usize>) -> Vec<usize> {

    let mut rng = rand::thread_rng();

    let index = rng.gen_range(0..solution.len());

    let mut new_solution = solution.clone();
    new_solution[index] = if new_solution[index] == 0 {1} else {0};

    new_solution
}

fn bitflip_mutation(solution: &Vec<usize>, mut_rate: f32) -> Vec<usize> {

    let mut rng = rand::thread_rng();
    let mut new_solution = solution.clone();

    for i in 0..solution.len() {
        if rng.gen::<f32>() < mut_rate {
            new_solution[i] = if new_solution[i] == 0 {1} else {0};
        }
    }

    new_solution
}

pub fn hill_climbing(instance: &Instance) -> Vec<usize> {

    let mut best = generate_solution(instance.num_items as usize);
    let mut best_profit = evaluate_solution(&best, instance);

    let mut count = 0;
    while best_profit < instance.optimal {
        let solution = bitflip_mutation(&best, 0.5);
        let profit = evaluate_solution(&solution, instance);

        println!("{:3} {:3} {:?} | {:3} {:?}", count, profit, solution, best_profit, best);

        if best_profit < profit {
            println!("NEW BEST FOUND {:3} {:?}", profit, solution);
            best = solution;
            best_profit = profit
        }

        count += 1;
    }

    best
}
