

use std::default::Default;

#[derive(Default)]
pub struct Chromosome<T> {
    genes: Vec<T> 
}

impl<T> Chromosome<T> {
    pub fn new(genes: Vec<T>) -> Self {
        Chromosome { genes }
    }
}

impl<T> std::fmt::Display for Chromosome<T> {    
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        formatter.

        todo!() 
    }
}
