use rust_radix_sort::{radix_sort, generate_vector};

#[test]
fn static_test() {
    // Hard coded test vectors for simple cases.
    let htv = vec![
        vec![],
        vec![2],
        vec![2, 3, 1, 5, 4],
        vec![-10, 8, 0, -11],
    ];
    // Test radix sort for each vector one by one.
    for v in htv {
        let mut current = v;
        let radix_res = radix_sort(&current);
        current.sort();
        assert_eq!(radix_res, current);
    }
}

#[test]
fn dynamic_test() {
    // Randomly generated larger vectors.
    // Test vectors consisting of 100, 10 000 and 1000 000 random integers.
    let rv = vec![
        generate_vector(2),
        generate_vector(4),
        generate_vector(6),
    ];
    // Test radix sort for each vector one by one.
    for v in rv {
        let mut current = v;
        let radix_res = radix_sort(&current);
        current.sort();
        assert_eq!(radix_res, current);
    }
}
