use rayon::prelude::*;

fn main() {
    let num_to_check = 100_000_000_000;

    let exists: Vec<u128> = possible_n(num_to_check)
        .map(|n| u128::checked_pow(n, 2).expect("exponent overflow"))
        .filter(is_palindrome)
        .collect();

    println!("{:?}", exists)
}

fn possible_n(amt: u128) -> impl ParallelIterator<Item = u128> {
    (0..amt).into_par_iter().map(|i| 845 + i * 1201)
}

fn is_palindrome(n: &u128) -> bool {
    let s = n.to_string();
    let rev_s = s.chars().rev().collect::<String>();
    s == rev_s
}
