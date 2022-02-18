

mod chromosome;


fn main() {
    let c0 = chromosome::Chromosome::new(vec![2, 2, 5, 8]);

    let c1 = chromosome::Chromosome::new(vec![1, 5, 4, 3]);

    let res = c0.recombine_with(&c1, 2);

    println!("Hello, world0! {}", c0);
    println!("Hello, world1! {}", c1);

    println!("res0! {}", res.0);
    println!("res1! {}", res.1);

}
