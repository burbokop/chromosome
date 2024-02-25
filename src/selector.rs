use alloc::vec::Vec;

use crate::{cascade_sum::cascade_sum, chromosome::Chromosome};

fn invert_normalize<T, I: Iterator<Item = T>>(values: I) -> impl Iterator<Item = f64>
where
    T: Into<f64> + Clone,
    I: Clone,
{
    let coefficients = values.map(|v| -> f64 { 1_f64 / v.into() });
    let sum: f64 = coefficients.clone().sum();
    coefficients.map(move |v| v / sum)
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    #[test]
    fn invert_normalize_test() {
        use crate::selector::invert_normalize;
        assert_eq!(
            invert_normalize(vec![2, 4, 8, 8].into_iter()).collect::<Vec<_>>(),
            vec![0.5, 0.25, 0.125, 0.125]
        );
    }
}

/// Genetic selector
pub trait Selector<T> {
    /// select_chromosome - selects not worst chromosomes with some random from vec
    fn select_chromosome<R: rand::RngCore>(
        &self,
        chromosomes: &[Chromosome<T>],
        rng: &mut R,
    ) -> Vec<Chromosome<T>>;
    /// return true if chromosome is ideal (solution of problem)
    fn is_ideal_chromosome(&self, chromosome: &Chromosome<T>) -> bool;
}

/// Fitness trait provides interface for finging fitness value for chromosome
///
/// fitness value - the smaller it is, the closer the chromosome is to the ideal (more adapted). if 0 is an ideal chromosome (solution of the problem)
///
pub trait Fitness {
    type Value;
    fn fitness(&self, chromosome: &Chromosome<Self::Value>) -> Self::Value;
    fn is_ideal_fitness(&self, fitness: Self::Value) -> bool;
}

/// FitnessSelector selects chromosomes by fitness values
///
/// Math:
///
/// S = 1 / f(c0) + 1 / f(c1) + ... + 1 / f(cn)
/// chances = { 1 / f(c0) / S, 1 / f(c1) / S, ..., 1 / f(cn) / S }
/// where f is fitness function
///
/// then selector random select N chromosomes with its select chance
/// where N is len of input chromosomes vec
///
#[derive(Debug)]
pub struct FitnessSelector<F> {
    fitness: F,
}

impl<F: Fitness> From<F> for FitnessSelector<F> {
    fn from(value: F) -> Self {
        FitnessSelector { fitness: value }
    }
}

fn abs64(v: f64) -> f64 {
    #[cfg(feature = "std")]
    {
        v.abs()
    }
    #[cfg(not(feature = "std"))]
    if v >= 0. {
        v
    } else {
        -v
    }
}

impl<T: Into<f64> + Clone, F: Fitness<Value = T>> Selector<T> for FitnessSelector<F> {
    fn select_chromosome<R: rand::RngCore>(
        &self,
        chromosomes: &[Chromosome<T>],
        rng: &mut R,
    ) -> Vec<Chromosome<T>> {
        let fitness = chromosomes.iter().map(|c| self.fitness.fitness(c));
        let nfv = cascade_sum(invert_normalize(fitness).map(abs64).collect());

        let mut result: Vec<Chromosome<T>> = Vec::with_capacity(chromosomes.len());
        while result.len() < chromosomes.len() {
            match nfv.random_index(rng) {
                Some(i) => result.push(chromosomes[i].clone()),
                _ => {}
            };
        }
        result
    }

    fn is_ideal_chromosome(&self, chromosome: &Chromosome<T>) -> bool {
        self.fitness
            .is_ideal_fitness(self.fitness.fitness(chromosome))
    }
}
