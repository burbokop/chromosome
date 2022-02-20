

use std::{default::Default, ops::Add, ops::Sub};
use rand::Rng;

/// Chromosome contains genes and provide genetic operations on them
/// 
#[derive(Default, Clone)]
pub struct Chromosome<T> {
    pub genes: Vec<T> 
}

impl<T> Chromosome<T> {
    pub fn new(genes: Vec<T>) -> Self {
        Chromosome { genes }
    }
}

impl<T: Clone> Chromosome<T> {
    /// recombined_with make recombination at specific point
    /// 
    /// 
    // --snip--
    /// # Examples
    /// 
    /// ```
    /// use chromosome::Chromosome;
    /// let result = Chromosome::new(vec![2, 2, 5, 8])
    ///     .recombined_with(&Chromosome::new(vec![1, 5, 4, 3]), 2);
    /// 
    /// assert_eq!(result.0.genes, vec![2, 2, 4, 3]);
    /// assert_eq!(result.1.genes, vec![1, 5, 5, 8]);
    /// ```
    /// 
    /// ```
    /// use chromosome::Chromosome;
    /// let result = Chromosome::new(vec![1, 1, 1, 1])
    ///     .recombined_with(&Chromosome::new(vec![2, 2, 2, 2]), 1);
    /// 
    /// assert_eq!(result.0.genes, vec![1, 2, 2, 2]);
    /// assert_eq!(result.1.genes, vec![2, 1, 1, 1]);
    /// ```
    /// 
    pub fn recombined_with(self: &Chromosome<T>, that: &Chromosome<T>, point: usize) -> (Chromosome<T>, Chromosome<T>) {
        if point < self.genes.len() && point < that.genes.len() {
            (
                Chromosome::new(self.genes[..point].iter().cloned().chain(that.genes[point..].iter().cloned()).collect()),
                Chromosome::new(that.genes[..point].iter().cloned().chain(self.genes[point..].iter().cloned()).collect())
            )
        } else {
            (self.clone(), that.clone())
        }
    }

    /// recombined_with make recombination at random point
    pub fn recombined_random_with<R : rand::RngCore>(self: &Chromosome<T>, that: &Chromosome<T>, rng: &mut R) -> (Chromosome<T>, Chromosome<T>) {
        self.recombined_with(that, rng.gen_range(0..(std::cmp::min(self.genes.len(), that.genes.len()) - 1)))
    }

}

impl <T: Add<Output=T> + Sub<Output=T> + Clone> Chromosome<T> {
    /// get random mutated chromosome
    pub fn mutated<R : rand::RngCore>(self: &Chromosome<T>, delta: T, chance: f64, rng: &mut R) -> Chromosome<T> {
        Chromosome::new(self.genes.iter().cloned().map(
            |gene| 
                if rng.gen_bool(chance) {
                    if rng.gen_bool(0.5) { gene + delta.clone() } else { gene - delta.clone() }
                } else { gene }
        ).collect())
    }
}


impl<T: std::fmt::Display> std::fmt::Display for Chromosome<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        let mut comma_separated = String::from("Chromosome { ");

        for gene in &self.genes[0..self.genes.len() - 1] {
            comma_separated.push_str(&format!("{}", gene));
            comma_separated.push_str(", ");
        }

        comma_separated.push_str(&self.genes[self.genes.len() - 1].to_string());
        comma_separated.push_str(" }");
        write!(formatter, "{}", comma_separated)
    }
}

