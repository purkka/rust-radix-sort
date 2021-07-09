#![feature(test)]
#![feature(is_sorted)]

extern crate test;

use std::mem;
use colored::*;
use rand::Rng;

// A least-significant-first radix sort implementation for 32-bit signed integers.
// The data is handled in 8-bit pieces and sorted using a counting sort algorithm.
// Sorts the vector in ascending order.
fn radix_sort(vec: &Vec<i32>) -> Vec<i32> {
    let n = vec.len();
    if n <= 1 { return vec.clone(); } // If the length of the vector is 1 or less, it is already sorted
    // The vectors a and b are temporary vectors where the partially sorted sets
    // will be moved back and forth.
    let mut a = vec.clone();
    let mut b = vec![0; n];
    // This is how many numbers there are in each 8-bit 'digit'.
    let k = 1 << 8;
    // Another temporary vector used to sort each 'digit' and to keep the counts of each number.
    let mut tmp = vec![0; k + 1];

    // The 32-bit integer is split into four 8-bit 'digits'. Go through each 'digit' and sort
    // the numbers using a counting sort algorithm.
    for s in 0..4 {
        // Flip the most significant bit (we're sorting signed integers) and extract the
        // desired 8 bits for each element. Count the instances of each digit.
        for i in 0..n {
            // To have the prefix sum start with a 0 instead of the first element, count the instances
            // in their respective positions + 1                 ↓
            tmp[(((a[i] ^ (1 << 31)) >> s * 8) & 0xff) as usize + 1] += 1;
        }
        // Calculate the prefix sum for tmp. This will be used to find the correct places for each number.
        tmp = tmp.iter().scan(0, |sum, i| {
            *sum += i;
            Some(*sum)
        }).collect::<Vec<_>>();
        // Go through the elements in a one by one. Move each of them to the correct position in b
        // using tmp.
        for i in 0..n {
            let element = a[i];
            let j = (((a[i] ^ (1 << 31)) >> s * 8) & 0xff) as usize;
            b[tmp[j]] = element;
            tmp[j] += 1; // Increase by one to get the next position.
        }
        // Switch a and b.
        mem::swap(&mut a, &mut b);
        // Fill tmp with zeroes.
        tmp = vec![0; k + 1];
    }
    // Return a, which currently holds the sorted vector.
    a
}

fn main() {
    println!("{}", "This program sorts vectors of 32-bit integers using a radix sort algorithm".purple().bold());
    println!("{}", "Erika Marttinen 2021".blue().blink().italic());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn static_test() {
        // Hard coded test vectors for simple cases.
        let htv = [
            vec![],
            vec![2],
            vec![2, 3, 1, 5, 4],
            vec![-10, 8, 0, -11],
        ];
        // Test radix sort for each vector one by one.
        for i in 0..4 {
            let mut current = htv[i].clone();
            let radix_res = radix_sort(&current);
            current.sort();
            assert_eq!(radix_res, current);
        }
    }

    #[test]
    fn dynamic_test() {
        // Randomly generated larger vectors.
        let mut rng = rand::thread_rng();
        // Test vectors consisting of 100, 10 000 and 1000 000 random integers.
        let v100: Vec<i32> = (0..100).map(|_| rng.gen::<i32>()).collect();
        let v10k: Vec<i32> = (0..10000).map(|_| rng.gen::<i32>()).collect();
        let v1m: Vec<i32> = (0..1000000).map(|_| rng.gen::<i32>()).collect();
        let rv = [v100, v10k, v1m];
        // Test radix sort for each vector one by one.
        for i in 0..3 {
            let mut current = rv[i].clone();
            let radix_res = radix_sort(&current);
            current.sort();
            assert_eq!(radix_res, current);
        }
    }

    #[bench]
    fn test_bench(b: &mut Bencher) {
        // This will be expanded later
        b.iter(|| radix_sort(&vec![0, 4, 1, 6]));
    }
}
