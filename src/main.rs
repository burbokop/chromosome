use std::{iter::Sum, ops::{Mul, Sub}};


use chromosome::{Chromosome, Selector, Fitness, SelectorFactory};


struct DiophantusEquation<'a, 'b, T> {
    coefficients: &'a Vec<T>, 
    result: &'b T
}

impl<'a, 'b, T> DiophantusEquation<'a, 'b, T> {
    fn new(coefficients: &'a Vec<T>, result: &'b T) -> Self {
        DiophantusEquation { coefficients: coefficients, result: result }
    }
}

impl<'a, 'b, T: Mul<Output = T> + Sum + Sub<Output = T> + Clone> Fitness for DiophantusEquation<'a, 'b, T> {
    type Value = T;
    fn fitness(self: &Self, chromosome: &Chromosome<T>) -> T {
        (0..usize::min(self.coefficients.len(), chromosome.genes.len()))
            .map(|i| self.coefficients[i].clone() * chromosome.genes[i].clone())
            .sum::<T>() - self.result.clone()
  }
}


fn main() {
    let c0 = Chromosome::new(vec![2, 2, 5, 8]);

    let c1 = Chromosome::new(vec![1, 5, 4, 3]);

    let mut rng = rand::rngs::OsRng::default();

    let rc = DiophantusEquation::new(&vec![2, 23, 54], &2)
        .selector()
        .select_chromosome(&vec![c0.clone(), c1.clone()], &mut rng);


    println!("Hello, world0! {}", c0);
    println!("Hello, world1! {}", c1);
    println!("rc           ! {}", rc);

}
