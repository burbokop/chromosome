//! # Chromosome
//!
//! Utilities for genetic algorithms
//!
//! # Example
//!```
#![doc = include_str!("../tests/diophantus_equation.rs")]
//!```

#![deny(warnings)]
#![no_std]

#[macro_use]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod cascade_sum;
mod chromosome;
mod selector;
mod simulation_iter;
mod simulator;
mod utils;

pub use crate::{
    chromosome::Chromosome, chromosome::Superposition, selector::Fitness,
    selector::FitnessSelector, selector::Selector, simulation_iter::SimulationIter,
    simulator::DefaultSimulator, simulator::Simulator, utils::map_random_pairs,
};
