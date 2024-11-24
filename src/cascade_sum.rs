use alloc::vec::Vec;
use rand::Rng;

pub(crate) struct CascadeSum {
    vec: Vec<f64>,
}

pub(crate) fn cascade_sum(vec: Vec<f64>) -> CascadeSum {
    let mut sum: f64 = 0.;
    let mut result = Vec::with_capacity(vec.len());
    for i in vec {
        result.push(i + sum);
        sum += i;
    }
    return CascadeSum { vec: result };
}

impl CascadeSum {
    pub(crate) fn random_index<R: rand::RngCore>(self: &Self, rng: &mut R) -> Option<usize> {
        if self.vec.len() > 0 {
            let p: f64 = rng.gen();
            for i in 0..self.vec.len() - 1 {
                if p < self.vec[i] && if i > 0 { p > self.vec[i - 1] } else { true } {
                    return Some(i);
                }
            }
            Some(self.vec.len() - 1)
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub(crate) fn generate_average<R: rand::RngCore>(
        self: &Self,
        size: usize,
        rng: &mut R,
    ) -> Vec<f64> {
        let mut meetings = vec![0; self.vec.len()];
        let mut cnt = 0;
        for _ in 0..size {
            let index = self.random_index(rng);
            meetings[index.unwrap()] += 1;
            cnt += 1;
        }
        meetings
            .into_iter()
            .map(|x| f64::from(x) / f64::from(cnt))
            .collect::<Vec<f64>>()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn cascade_sum_test() {
        use crate::cascade_sum::cascade_sum;
        assert_eq!(cascade_sum(vec![0.5, 0.1, 0.4]).vec, vec![0.5, 0.6, 1.0])
    }
}
