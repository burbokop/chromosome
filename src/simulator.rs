

use rand::Rng;

use crate::selector::Selector;
use crate::chromosome::Chromosome;
use std::{cmp::Ordering::{Less, Greater}, ops::{Sub, Add}};

fn map_random_pairs<T: Clone, R : rand::RngCore, F: Fn(T, T, &mut R) -> (T, T)>(vec: Vec<T>, f: F, rng: &mut R) -> Vec<T> {
    let mut result = vec;
    result.sort_by(|_, _| if rng.gen_bool(0.5) { Less } else { Greater });
    for i in (0..result.len() - 1).step_by(2) {
        let (a, b) = f(result[i].clone(), result[i + 1].clone(), rng);
        result[i] = a; result[i + 1] = b;
    }
    return result;
}

/// Simulator makes blocking simulation of genetics
/// returns Chromosome if simulation succesful and solution found and None if no solution
pub trait Simulator<T, S: Selector<T>> {
    fn simulate<R : rand::RngCore>(self: Self, initial_chromosomes: Vec<Chromosome<T>>, selector: S, rng: &mut R) -> Option<Chromosome<T>>;
}

/// DefaultSimulator make default simulation
/// 
/// Cicle steps:
/// 1. check if current step chromosomes vec has ideal chromosome (is yes returns Some)
/// 2. select chromosomes by provided selector
/// 3. random recombine selected
/// 4. random mutate recombined
/// 
/// if steps count > `iteration_limit` return None
/// 
pub struct DefaultSimulator<T> {
    mutation_delta: T,
    mutation_chance: f64,
    iteration_limit: usize
}

impl<T> DefaultSimulator<T> {
    pub fn new(
        mutation_delta: T,
        mutation_chance: f64,
        iteration_limit: usize    
    ) -> Self {
        DefaultSimulator { 
            mutation_delta: mutation_delta,
            mutation_chance: mutation_chance,
            iteration_limit: iteration_limit
        }
    }
}

impl<T: Clone + Add<Output = T> + Sub<Output = T>, S: Selector<T>> Simulator<T, S> for DefaultSimulator<T> {
    fn simulate<R : rand::RngCore>(self: Self, initial_chromosomes: Vec<Chromosome<T>>, selector: S, rng: &mut R) -> Option<Chromosome<T>> {
        let mut chromosomes: Vec<Chromosome<T>> = initial_chromosomes;
        for i in 0..self.iteration_limit {
            let ideal= chromosomes.iter().find(|c| selector.is_ideal_chromosome(c));
            if ideal.is_some() { return ideal.cloned(); }

            let selected = selector.select_chromosome(&chromosomes, rng);
            let children = map_random_pairs(selected, |x, y, z| x.recombined_random_with(&y, z), rng);
            chromosomes = children.into_iter().map(|c| c.mutated(self.mutation_delta.clone(), self.mutation_chance, rng)).collect();
        }
        None
    }
}