use std::time::SystemTime;

use rand::prelude::StdRng;



mod chromosome;


fn main() {
    let c0 = chromosome::Chromosome::new(vec![2, 2, 5, 8]);

    let c1 = chromosome::Chromosome::new(vec![1, 5, 4, 3]);

    let d = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Duration since UNIX_EPOCH failed");

    let mut rng = rand::rngs::OsRng::default();

    let res = c0.recombined_random_with(&c1, &mut rng);

    let mutres = c0.mutated(1, 0.5, &mut rng);
    println!("mutres!        {}", mutres);

    println!("Hello, world0! {}", c0);
    println!("Hello, world1! {}", c1);

    println!("res0!          {}", res.0);
    println!("res1!          {}", res.1);

}
