use rand::Rng;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Instance {    
    pub num_items: u32,
    pub max_weight: f32,
    pub profits: Vec<f32>,
    pub weights: Vec<f32>,
    pub optimal: f32
}

pub fn read_instance(filename: &str) -> std::io::Result<Instance> {

    // using ? here because the error message is clear enough
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    // read the first line and split it by space
    let first_line = lines.next()
        .ok_or_else(|| file_error())??;

    let mut iter = first_line.split_whitespace();

    let num_items = iter.next()
        .ok_or_else(|| file_error())?
        .parse::<u32>()
        .map_err(|_| file_error())?;

    let max_weight = iter.next()
        .ok_or_else(|| file_error())?
        .parse::<f32>()
        .map_err(|_| file_error())?;

    let optimal = iter.next()
        .ok_or_else(|| file_error())?
        .parse::<f32>()
        .map_err(|_| file_error())?;

    let mut profits: Vec<f32> = Vec::new();
    let mut weights: Vec<f32> = Vec::new();
    for line in lines {
        let line_content = line?;
        let mut iter = line_content.split_whitespace();

        let profit = iter.next()
            .ok_or_else(|| file_error())?
            .parse::<f32>()
            .map_err(|_| file_error())?;

        let weight = iter.next()
            .ok_or_else(|| file_error())?
            .parse::<f32>()
            .map_err(|_| file_error())?;

        profits.push(profit);
        weights.push(weight);
    }

    // simple check if the correct number of items is provided
    if profits.len() != num_items as usize || weights.len() != num_items as usize {
        return Err(file_error())
    }

    Ok(Instance {
        num_items,
        max_weight,
        profits,
        weights,
        optimal
    })
}

fn file_error() -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, "Invalid Instance file")
}

pub fn generate_solution(size: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(0..2)).collect()
}

pub fn evaluate_solution(solution: &Vec<usize>, instance: &Instance) -> f32 {

    let mut profit = 0.0;
    let mut weight = 0.0;
    for i in 0..solution.len() {
        profit += solution[i] as f32 * instance.profits[i];
        weight += solution[i] as f32 * instance.weights[i];
    }

    if weight > instance.max_weight {
        return -1.0
    }

    profit
}