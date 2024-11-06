use crate::{map_random_pairs, Chromosome, Selector, Superposition};
use alloc::vec::Vec;
use core::ops::{Add, Sub};

/// SimulationIter provides Simulating genetics by iterating. each `next()` call produces next generation. `.collect()` gives all generations up to final `ideal` one
#[derive(Debug)]
pub struct SimulationIter<T, D, S, R> {
    mutation_delta: Vec<D>,
    mutation_chance: f64,
    chromosomes: Vec<Chromosome<T>>,
    selector: S,
    rng: R,
    completed: bool,
}

impl<T, D, S, R> SimulationIter<T, D, S, R> {
    pub fn new(
        mutation_delta: Vec<D>,
        mutation_chance: f64,
        initial_chromosomes: Vec<Chromosome<T>>,
        selector: S,
        rng: R,
    ) -> Self {
        SimulationIter {
            mutation_delta,
            mutation_chance,
            chromosomes: initial_chromosomes,
            selector,
            rng,
            completed: false,
        }
    }
}

impl<
        T: Clone + Add<Output = T> + Sub<Output = T>,
        D: Superposition<T> + Clone,
        S: Selector<T>,
        R: rand::RngCore,
    > Iterator for SimulationIter<T, D, S, R>
{
    type Item = Vec<Chromosome<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.completed {
            return None;
        }

        let ideal = self
            .chromosomes
            .iter()
            .find(|c| self.selector.is_ideal_chromosome(c));
        if let Some(ideal) = ideal {
            self.completed = true;
            return Some(vec![ideal.clone()]);
        }

        let selected = self
            .selector
            .select_chromosome(&self.chromosomes, &mut self.rng);
        let children = map_random_pairs(
            selected,
            |x, y, z| x.recombined_random_with(&y, z),
            &mut self.rng,
        );
        self.chromosomes = children
            .into_iter()
            .map(|c| c.mutated(&self.mutation_delta, self.mutation_chance, &mut self.rng))
            .collect();

        Some(self.chromosomes.clone())
    }
}
