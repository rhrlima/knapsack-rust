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