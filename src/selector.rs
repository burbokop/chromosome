
use rand::Rng;

use crate::chromosome::Chromosome;

pub fn invert_normalize<T, I: Iterator<Item=T>>(values: I) -> impl Iterator<Item=f64>
    where 
    T: Into<f64> + Clone,
    I: Clone {
    let coefficients = values
        .map(|v| -> f64 { 1_f64 / v.clone().into() });
    let sum: f64 = coefficients.clone().sum();
    coefficients.map(move |v| v / sum)
}

pub fn cascade_sum(vec: Vec<f64>) -> Vec<f64> {
    let mut sum: f64 = 0.;
    let mut result = Vec::with_capacity(vec.len());
    for i in vec {
        result.push(i + sum);
        sum += i;
    }
    return result;
}

pub trait Selector<T> {
    fn select_chromosome<R : rand::RngCore>(self: &Self, chromosomes: &Vec<Chromosome<T>>, rng: &mut R) -> Chromosome<T>;
}

pub trait Fitness<T> {
    fn fitness_value(self: &Self, chromosome: &Chromosome<T>) -> T;
    fn selector(self: &Self) -> FitnessSelector<Self> where Self: Sized { FitnessSelector { fitness: self } }
}

pub struct FitnessSelector<'a, F> {
    fitness: &'a F
}

impl <'a, F> FitnessSelector<'a, F> {
    pub fn from_fitness(f: &'a F) -> Self { FitnessSelector { fitness: f } }
}

impl<'a, T: Into<f64> + Clone, F: Fitness<T>> Selector<T> for FitnessSelector<'a, F> {
    fn select_chromosome<R : rand::RngCore>(self: &Self, chromosomes: &Vec<Chromosome<T>>, rng: &mut R) -> Chromosome<T> {
        let fitness = chromosomes.iter().map(|c| self.fitness.fitness_value(c));
        let nfv: Vec<_> =  cascade_sum(invert_normalize(fitness)
            .map(|x| x.abs()).collect());
        
        println!("nfv {:?}", nfv);
        for i in 0..nfv.len() - 1 {
            let p: f64 = rng.gen();
            println!("i: {}, f: {}, p: {}", i, nfv[i], p);
            if p < nfv[i] && if i > 0 { p > nfv[i - 1] } else { true } {
                println!("yes");
                return chromosomes[i].clone();
            }
        }
        chromosomes.last().unwrap().clone()
    }

}            
