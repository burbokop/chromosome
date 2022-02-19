

use genetic::Chromosome;

#[test]
fn recombined_with_test() {
    let result = Chromosome::new(vec![2, 2, 5, 8])
        .recombined_with(&Chromosome::new(vec![1, 5, 4, 3]), 2);

    assert_eq!(result.0.genes, vec![2, 2, 4, 3]);
    assert_eq!(result.1.genes, vec![1, 5, 5, 8]);
    println!("gogadodao test")
} 