
//! # Genetic
//!
//! `genetic` - utilities for genetic algorithms


mod chromosome;
mod selector;


pub use crate::{
    chromosome::Chromosome,
    selector::Selector,
    selector::Fitness,
    selector::FitnessSelector
};