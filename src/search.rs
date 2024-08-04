use crate::problem::{evaluate_solution, generate_solution, Instance};
use crate::operators::{bitflip_mutation, one_point_crossover, tournament_selection};

pub fn random_search(instance: &Instance) -> Vec<usize> {

    let mut best = vec![0; instance.num_items as usize];
    let mut best_profit = 0.0;

    // let mut count = 0;
    while best_profit < instance.optimal {

        let solution = generate_solution(instance.num_items as usize);
        let profit = evaluate_solution(&solution, instance);

        // println!("NEW BEST {:9} {:4} {:?}", count, profit, solution);

        if profit > best_profit {
            best_profit = profit;
            best = solution;
        }

        // count += 1;
    }

    best
}

pub fn hill_climbing(instance: &Instance, mut_rate: f32) -> Vec<usize> {

    let mut best = vec![0; instance.num_items as usize];
    let mut best_profit = 0.0;

    // let mut count = 0;
    while best_profit < instance.optimal {

        let solution = bitflip_mutation(&best, mut_rate);
        let profit = evaluate_solution(&solution, instance);

        if best_profit < profit {
            // println!("NEW BEST {:9} {:4} {:?}", count, profit, solution);
            best = solution;
            best_profit = profit
        }

        // count += 1;
    }

    best
}

pub fn genetic_algorithm(instance: &Instance, pop_size: usize, mut_rate: f32, cross_rate: f32) -> Vec<usize> {

    let mut population:Vec<Vec<usize>> = Vec::new();
    let mut profits: Vec<f32> = Vec::new();

    for i in 0..pop_size {
        population.push(generate_solution(instance.num_items as usize));
        profits.push(evaluate_solution(&population[i], &instance));
    }

    let (mut best, mut best_profit) = get_best_solution(instance, &population);

    let mut generations = 0;
    loop {
        if best_profit >= instance.optimal {
            break;
        }

        let mut offspring_pop: Vec<Vec<usize>> = Vec::new();
        for _ in 0..pop_size/2 {
            let parent1 = tournament_selection(instance, &population, 5);
            let parent2 = tournament_selection(instance, &population, 5);
        
            let (offspring1, offspring2) = one_point_crossover(parent1, parent2, cross_rate);
            let offspring1 = bitflip_mutation(&offspring1, mut_rate);
            let offspring2 = bitflip_mutation(&offspring2, mut_rate);
    
            offspring_pop.push(offspring1);
            offspring_pop.push(offspring2);
        }
        population = offspring_pop;

        (best, best_profit) = get_best_solution(instance, &population);
        println!("GEN {:4} BEST {:4} {:?}", generations, best_profit, best);

        generations += 1;
    }

    best.clone()
}

fn get_best_solution(instance: &Instance, population: &Vec<Vec<usize>>) -> (Vec<usize>, f32) {
    let mut best = &population[0];
    let mut best_profit = evaluate_solution(&population[0], instance);

    for i in 1..population.len() {
        let profit = evaluate_solution(&population[i], instance);
        if profit > best_profit {
            best = &population[i];
            best_profit = profit;
        }
    }

    (best.clone(), best_profit)
}
