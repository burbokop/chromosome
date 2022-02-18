

use std::default::Default;
use rand::Rng;

#[derive(Default, Clone)]
pub struct Chromosome<T> {
    genes: Vec<T> 
}

impl<T> Chromosome<T> {
    pub fn new(genes: Vec<T>) -> Self {
        Chromosome { genes }
    }

}

impl<T: Clone> Chromosome<T> {
    pub fn recombine_with(self: &Chromosome<T>, that: &Chromosome<T>, point: usize) -> (Chromosome<T>, Chromosome<T>) {
        if point < self.genes.len() && point < that.genes.len() {
            (
                Chromosome::new(self.genes[..point].iter().cloned().chain(that.genes[point..].iter().cloned()).collect()),
                Chromosome::new(that.genes[..point].iter().cloned().chain(self.genes[point..].iter().cloned()).collect())
            )
        }
        else {
            (self.clone(), that.clone())
        }
    }

    pub fn recombine_random_with(self: &Chromosome<T>, that: &Chromosome<T>) -> (Chromosome<T>, Chromosome<T>) {
        let r = rand::thread_rng();
        r.

        self.recombine_With(that, RandGeg.int.generate(0, Math.min(genes.length, that.genes.length) - 1))
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

// rust-analyzer failed to load workspace: Failed to find sysroot for Cargo.toml file /home/boris/projects/rust/genetics/Cargo.toml. Is rust-src installed?: can't load standard library from sysroot
// /usr
// (discovered via `rustc --print sysroot`)
// try installing the Rust source the same way you installed rustc
