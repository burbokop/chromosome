use core::{
    fmt::{self, Debug, Display, Formatter},
    ops::{Add, AddAssign, Range, Sub, SubAssign},
};

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use rand::{
    distributions::uniform::{SampleRange, SampleUniform},
    Rng, RngCore,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Chromosome contains genes and provide genetic operations on them
#[derive(Default, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Chromosome<T> {
    pub genes: Vec<T>,
}

impl<T> Chromosome<T> {
    #[inline]
    pub fn new(genes: Vec<T>) -> Self {
        Chromosome { genes }
    }

    #[inline]
    pub fn new_random<R: rand::RngCore, Range: SampleRange<T> + Clone>(
        size: usize,
        range: Range,
        rng: &mut R,
    ) -> Self
    where
        T: SampleUniform,
    {
        Chromosome {
            genes: (0..size)
                .into_iter()
                .map(|_| rng.gen_range(range.clone()))
                .collect(),
        }
    }

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
    #[inline]
    pub fn recombined_with(
        self: &Chromosome<T>,
        that: &Chromosome<T>,
        point: usize,
    ) -> (Chromosome<T>, Chromosome<T>)
    where
        T: Clone,
    {
        if point < self.genes.len() && point < that.genes.len() {
            (
                Chromosome::new(
                    self.genes[..point]
                        .iter()
                        .cloned()
                        .chain(that.genes[point..].iter().cloned())
                        .collect(),
                ),
                Chromosome::new(
                    that.genes[..point]
                        .iter()
                        .cloned()
                        .chain(self.genes[point..].iter().cloned())
                        .collect(),
                ),
            )
        } else {
            (self.clone(), that.clone())
        }
    }

    /// recombined_with make recombination at random point
    #[inline]
    pub fn recombined_random_with<R>(
        self: &Chromosome<T>,
        that: &Chromosome<T>,
        rng: &mut R,
    ) -> (Chromosome<T>, Chromosome<T>)
    where
        T: Clone,
        R: rand::RngCore,
    {
        let len = core::cmp::min(self.genes.len(), that.genes.len());
        self.recombined_with(
            that,
            if len > 1 {
                rng.gen_range(0..(len - 1))
            } else {
                0
            },
        )
    }

    /// get random mutated chromosome
    #[inline]
    pub fn mutated<D, F, R>(self: Self, delta: F, chance: f64, rng: &mut R) -> Chromosome<T>
    where
        T: Add<Output = T> + Sub<Output = T>,
        D: Superposition<T>,
        F: Fn(usize, &mut R) -> D,
        R: RngCore,
    {
        Chromosome::new(
            self.genes
                .into_iter()
                .enumerate()
                .map(|(i, gene)| {
                    if rng.gen_bool(chance) {
                        if rng.gen_bool(0.5) {
                            gene + delta(i, rng).collapse(rng)
                        } else {
                            gene - delta(i, rng).collapse(rng)
                        }
                    } else {
                        gene
                    }
                })
                .collect(),
        )
    }

    #[inline]
    pub fn mutate<D, F, R>(self: &mut Self, delta: F, chance: f64, rng: &mut R)
    where
        T: AddAssign + SubAssign,
        D: Superposition<T>,
        F: Fn(usize, &mut R) -> D,
        R: RngCore,
    {
        self.genes.iter_mut().enumerate().for_each(|(i, gene)| {
            if rng.gen_bool(chance) {
                if rng.gen_bool(0.5) {
                    *gene += delta(i, rng).collapse(rng);
                } else {
                    *gene -= delta(i, rng).collapse(rng);
                }
            }
        })
    }
}

pub trait Superposition<T> {
    fn collapse<R: rand::RngCore>(self, rng: &mut R) -> T;
}

impl<T> Superposition<T> for T {
    fn collapse<R: rand::RngCore>(self, _: &mut R) -> T {
        self
    }
}

impl<T: SampleUniform + PartialOrd> Superposition<T> for Range<T> {
    fn collapse<R: rand::RngCore>(self, rng: &mut R) -> T {
        rng.gen_range(self)
    }
}

impl<T: Display> Display for Chromosome<T> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
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

impl<T: Debug> Debug for Chromosome<T> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(formatter, "Chromosome {:?}", self.genes)
    }
}
