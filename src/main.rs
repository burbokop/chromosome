

mod chromosome;



fn main() {


    let vec = vec![1, 2, 3];


    for elem in &vec {
        println!("{}", elem);
    }

    let c = chromosome::Chromosome::new(vec);

    println!("Hello, world! {}", c);
}
