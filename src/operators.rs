use rand::Rng;
use super::problem::Instance;
use crate::problem::evaluate_solution;

#[allow(dead_code)]
pub fn one_bit_mutation(solution: &Vec<usize>) -> Vec<usize> {

    let mut rng = rand::thread_rng();

    let index = rng.gen_range(0..solution.len());

    let mut new_solution = solution.clone();
    new_solution[index] = if new_solution[index] == 0 {1} else {0};

    new_solution
}

pub fn bitflip_mutation(solution: &Vec<usize>, mut_rate: f32) -> Vec<usize> {

    let mut rng = rand::thread_rng();
    let mut new_solution = solution.clone();

    for i in 0..solution.len() {
        if rng.gen::<f32>() < mut_rate {
            new_solution[i] = if new_solution[i] == 0 {1} else {0};
        }
    }

    new_solution
}

pub fn one_point_crossover(parent1: &Vec<usize>, parent2: &Vec<usize>, cross_rate: f32) -> (Vec<usize>, Vec<usize>) {

    let mut rng = rand::thread_rng();

    if rng.gen::<f32>() < cross_rate {
        return (parent1.clone(), parent2.clone())
    }

    let index = rng.gen_range(1..parent1.len());

    let mut offspring1 = parent1.clone();
    let mut offspring2 = parent2.clone();

    for i in 0..index {
        offspring1[i] = parent2[i];
        offspring2[i] = parent1[i];
    }

    (offspring1, offspring2)
}


pub fn tournament_selection<'a>(instance: &Instance, population: &'a Vec<Vec<usize>>, t_size: usize) -> &'a Vec<usize> {

    let mut rng = rand::thread_rng();
    let mut best = &population[rng.gen_range(0..population.len())];
    let mut best_profit = evaluate_solution(best, instance);

    for _ in 0..t_size {
        let candidate = &population[rng.gen_range(0..population.len())];
        let profit = evaluate_solution(candidate, instance);
        if profit > best_profit {
            best = candidate;
            best_profit = profit;
        }
    }

    best
}
