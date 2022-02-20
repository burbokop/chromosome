

use crate::{chromosome::Chromosome, cascade_sum::cascade_sum};

fn invert_normalize<T, I: Iterator<Item=T>>(values: I) -> impl Iterator<Item=f64>
    where 
    T: Into<f64> + Clone,
    I: Clone {
    let coefficients = values
        .map(|v| -> f64 { 1_f64 / v.clone().into() });
    let sum: f64 = coefficients.clone().sum();
    coefficients.map(move |v| v / sum)
}


#[cfg(test)]
mod tests {
    #[test]
    fn invert_normalize_test() {
        use crate::selector::invert_normalize;
        assert_eq!(invert_normalize(vec![2, 4, 8, 8].into_iter()).collect::<Vec<_>>(), vec![0.5, 0.25, 0.125, 0.125]);
    }
}

pub trait Selector<T> {
    /// select_chromosome - selects not worst chromosomes with some random from vec
    /// returns tuple (selected chromosome, is the ideal)
    fn select_chromosome<R : rand::RngCore>(self: &Self, chromosomes: &Vec<Chromosome<T>>, rng: &mut R) -> Vec<Chromosome<T>>;
    fn is_ideal_chromosome(self: &Self, chromosome: &Chromosome<T>) -> bool;
}

/// Fitness trait provides interface for finging fitness value for chromosome
/// 
/// fitness value - the smaller it is, the closer the chromosome is to the ideal (more adapted). if 0 is an ideal chromosome (solution of the problem)
/// 
pub trait Fitness {
    type Value;
    fn fitness(self: &Self, chromosome: &Chromosome<Self::Value>) -> Self::Value;
}

pub trait SelectorFactory<'a, S> {
    fn selector(self: &'a Self) -> S;
}

impl<'a, F: Fitness> SelectorFactory<'a, FitnessSelector<'a, F>> for F {
    fn selector(self: &'a Self) -> FitnessSelector<'a, F> { FitnessSelector::<'a, F> { fitness: self } }
}

pub struct FitnessSelector<'a, F> {
    fitness: &'a F
}

impl <'a, F> FitnessSelector<'a, F> {
    pub fn from_fitness(f: &'a F) -> Self { FitnessSelector { fitness: f } }
}

impl<'a, T: Into<f64> + Clone, F: Fitness<Value = T>> Selector<T> for FitnessSelector<'a, F> {
    fn select_chromosome<R : rand::RngCore>(self: &Self, chromosomes: &Vec<Chromosome<T>>, rng: &mut R) -> Vec<Chromosome<T>> {
        let fitness = chromosomes.iter().map(|c| self.fitness.fitness(c));
        let nfv =  cascade_sum(invert_normalize(fitness)
            .map(|x| x.abs()).collect());
        
        let mut result: Vec<Chromosome<T>> = Vec::with_capacity(chromosomes.len());
        while result.len() < chromosomes.len() {
            match nfv.random_index(rng) {
                Some(i) => result.push(chromosomes[i].clone()),
                _ => {}
            };
        }
        result
    }
    fn is_ideal_chromosome(self: &Self, chromosome: &Chromosome<T>) -> bool { self.fitness.fitness(chromosome).into() == 0_f64 }
}            
