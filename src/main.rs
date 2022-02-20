use std::iter::Sum;


use chromosome::{Chromosome, Selector, FitnessSelector, Fitness};



struct TestFitness {

}

impl TestFitness {
    fn new() -> Self { TestFitness {  } }
}

impl<T: Sum + Clone> Fitness<T> for TestFitness {
    fn fitness_value(self: &Self, chromosome: &Chromosome<T>) -> T {
        chromosome.genes.iter().cloned().sum()
    }
}

fn main() {
    let c0 = Chromosome::new(vec![2, 2, 5, 8]);

    let c1 = Chromosome::new(vec![1, 5, 4, 3]);

    let mut rng = rand::rngs::OsRng::default();

    let rc = FitnessSelector::from_fitness(&TestFitness::new()).select_chromosome(&vec![c0.clone(), c1.clone()], &mut rng);

    //TestFitness::new().selector().select_chromosome(&vec![c0, c1]);

    println!("Hello, world0! {}", c0);
    println!("Hello, world1! {}", c1);
    println!("rc           ! {}", rc);

}
