

use std::{ops::{Mul, Sub}, iter::Sum};

use chromosome::{Chromosome, DefaultSimulator, Simulator, SelectorFactory};
use chromosome::Fitness;

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


#[test]
fn diophantus_equation() {
    let mut rng = rand::rngs::OsRng::default();
    let coefs = vec![2_i32, 23, 54, 1];
    let equation = DiophantusEquation::new(&coefs, &2);

    let sim_result = DefaultSimulator::new(1, 0.09, 10000)
        .simulate(
            vec![
                Chromosome::new_random(equation.coefficients.len(), 0_i32..10, &mut rng), 
                Chromosome::new_random(equation.coefficients.len(), 0_i32..10, &mut rng)                
                ],             
            equation.selector(),
            & mut rng
        );

    assert!(sim_result.is_some())
} 