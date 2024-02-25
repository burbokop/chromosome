use alloc::vec::Vec;
use core::cmp::Ordering::{Greater, Less};
use rand::Rng as _;

pub fn map_random_pairs<T: Clone, R: rand::RngCore, F: Fn(T, T, &mut R) -> (T, T)>(
    vec: Vec<T>,
    f: F,
    rng: &mut R,
) -> Vec<T> {
    let mut result = vec;
    result.sort_by(|_, _| if rng.gen_bool(0.5) { Less } else { Greater });
    for i in (0..result.len() - 1).step_by(2) {
        let (a, b) = f(result[i].clone(), result[i + 1].clone(), rng);
        result[i] = a;
        result[i + 1] = b;
    }
    result
}
