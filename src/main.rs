use num_bigint::BigUint;
use num_traits::pow;
use std::cmp::max;

fn main() {
    let upper_search_bound: BigUint = pow(BigUint::from(10_u32), 20);
    let mut highest: BigUint = 0_u32.into();

    let mut n: BigUint = 1_u32.into();
    let mut m: BigUint = 1_u32.into();
    let mut num_square_triangular = 0;

    while highest <= upper_search_bound {
        println!("Searching n={n}, m={m}");
        let square = square_it(&n);
        let triangular = triangular_it(&m);

        match square.cmp(&triangular) {
            std::cmp::Ordering::Less => {
                highest = max(highest, triangular);
                n += 1_u32;
            }
            std::cmp::Ordering::Greater => {
                highest = max(highest, square);
                m += 1_u32;
            }
            std::cmp::Ordering::Equal => {
                num_square_triangular += 1;
                println!("Found square triangular (#{num_square_triangular})");
                highest = max(highest, square);
                n += 1_u32;
                m += 1_u32;
            }
        }
    }

    println!("Answer: {num_square_triangular}")
}

fn square_it(n: &BigUint) -> BigUint {
    pow(n.to_owned(), 2)
}

fn triangular_it(m: &BigUint) -> BigUint {
    (m * (m + 1_u32)) / 2_u32
}
