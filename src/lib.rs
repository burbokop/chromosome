
//! # Chromosome
//!
//! `chromosome` - utilities for genetic algorithms

#![no_std]

#[macro_use]
extern crate alloc;

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
