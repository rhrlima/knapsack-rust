# Knapsack Rust

Implementation of a few solvers for the Knapsack Problem, in Rust!

## The problem

Given a set of items, each with a weight and a value, determine which items to include in the collection so that the total weight is less than or equal to a given limit and the total value is as large as possible.

Source: [Wikipedia](https://en.wikipedia.org/wiki/Knapsack_problem)

Instances: [Binary Knapsack Problem](http://artemisa.unicauca.edu.co/~johnyortega/instances_01_KP/)

## How to run

To run the tests, you can run:
```bash
cargo test
```

Usage:
```bash
Usage: knapsack --algorithm <ALGORITHM> --instance-file <INSTANCE_FILE>

Options:
  -a, --algorithm <ALGORITHM>
  -i, --instance-file <INSTANCE_FILE>
  -h, --help                           Print help
  -V, --version                        Print version
```

## Instance Files

> [!IMPORTANT]  
> The instance files were modified from the original source, to also include the `optimal` profit in the first line.

```
N W O
v1 w1
v2 w2
...
vn wn
```

Where:
- N: Number of items
- W: Max weight allowed
- O: Optimal Profit
- vi: profit of item `i`
- wi: weight of item `i`

Example:
```
4 20 35
9 6
11 5
13 9
15 7
```

## Solvers

- Random Search
- Hill Climbing
- Genetic Algorithm
