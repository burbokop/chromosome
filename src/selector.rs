use std::{ops::Deref, collections::binary_heap::Iter};

use crate::chromosome::Chromosome;


trait Selector {
    fn select_chromosome<T>(self: &Self, chromosomes: Vec<Chromosome<T>>) -> Chromosome<T>;
}

trait Fitness {
    fn fitness_value<T>(self: &Self, chromosome: Chromosome<T>) -> T;
    fn selector(self: &Self) -> FitnessSelector<Self> where Self: Sized { FitnessSelector { fitness: self } }
}

struct FitnessSelector<'a, F: Fitness> {
    fitness: &'a F
}

impl<'a, F: Fitness> FitnessSelector<'a, F> {
    fn normalized_values<'b, T>(values: & 'b Vec<T>) -> impl Iterator<Item=f64> + 'b where T: Deref<Target=f64> + Clone {
        let coefficients = values.iter().map(|v: &'b T| 1. / v.deref());
        let sum: f64 = coefficients.cloned().sum();
        coefficients.map(|v| v / sum)
    }
}

impl<'a, F: Fitness> Selector for FitnessSelector<'a, F> {
    fn select_chromosome<T>(self: &Self, chromosomes: Vec<Chromosome<T>>) -> Chromosome<T> {
        let fitness = chromosomes.iter().map(|c| self.fitness.fitness_value(c));
        let nfv = Self::normalized_values(fitness)
            .map(|x| x.abs()).collect();

/*
            .sorted
            .mapWithLast((c, l) => l.map(c + _).getOrElse(c))
            (
            (0 until chromosomes.length).map(_ => chromosomes(nfv.indexWhere(_ > RandGeg.double.generate(0, 1)))),
            fitness
            )
            */
}

}