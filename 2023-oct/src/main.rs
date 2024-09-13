use std::fs::File;
use std::io::{BufRead, BufReader};

use rayon::prelude::*;

fn main() {
    let num_to_check = 1_000_000_000;

    // let exists: Vec<u128> = possible_n(num_to_check)
    //     .filter(|i| !is_palindrome(i))
    //     .map(|n| u128::checked_pow(n, 2).expect("exponent overflow"))
    //     .filter(is_palindrome)
    //     .collect();
    // println!("{:?}", exists)

    let reader = BufReader::new(File::open("vals.txt").expect("cannot open file"));

    for line in reader.lines() {
        let l = line.unwrap();
        let val = l.split_whitespace().nth(1).unwrap();

        let x_res = val.parse::<u128>();
        let x = x_res.unwrap();

        if is_in_mod(&x) && is_palindrome(&u128::checked_pow(x, 2).unwrap()) {
            println!("{}", x)
        } else {
            println!("not {}({})", x, x % 1201)
        }
    }
}

fn possible_n(amt: u128) -> impl ParallelIterator<Item = u128> {
    (0..amt).into_par_iter().map(|i| 845 + i * 1201)
}

fn is_palindrome(n: &u128) -> bool {
    let s = n.to_string();
    let rev_s = s.chars().rev().collect::<String>();
    s == rev_s
}

fn is_in_mod(n: &u128) -> bool {
    n % 1201 == 845
}
