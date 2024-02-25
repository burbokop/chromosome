//! # Chromosome
//!
//! Utilities for genetic algorithms
//!
//! # Example
//!```
#![doc = include_str!("../tests/diophantus_equation.rs")]
//!```

#![no_std]

#[macro_use]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod cascade_sum;
mod chromosome;
mod selector;
mod simulator;

pub use crate::{
    chromosome::Chromosome, chromosome::Superposition, selector::Fitness,
    selector::FitnessSelector, selector::Selector, simulator::DefaultSimulator,
    simulator::Simulator,
};
