
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

mod chromosome;
mod selector;
mod simulator;
mod cascade_sum;

pub use crate::{
    chromosome::Chromosome,
    selector::Selector,
    selector::SelectorFactory,
    selector::Fitness,
    selector::FitnessSelector,
    simulator::Simulator,
    simulator::DefaultSimulator
};
