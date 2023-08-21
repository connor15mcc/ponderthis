use num_bigint::BigUint;
use num_traits::pow;
use std::cmp::max;

fn main() {
    let mut dp: Vec<BigUint> = vec![BigUint::from(1_u32), BigUint::from(36_u32)];
    let upper_search_bound: BigUint = pow(BigUint::from(10_u32), 100);

    while dp.last().unwrap() <= &upper_search_bound {
        let n = dp.len();
        dp.push((dp.get(n - 1).unwrap() - BigUint::from(1_u32)).pow(2) / dp.get(n - 2).unwrap());
        println!("{}", dp.last().unwrap());
    }

    dp.retain(|term| term.lt(&upper_search_bound));
    println!(
        "{} square triangular numbers less than {:#?}",
        dp.len(),
        upper_search_bound
    );

    // ping_pong_search(dp);
}

fn square_it(n: &BigUint) -> BigUint {
    pow(n.to_owned(), 2)
}

fn triangular_it(m: &BigUint) -> BigUint {
    (m * (m + 1_u32)) / 2_u32
}

fn ping_pong_search(dp: Vec<BigUint>) {
    let upper_search_bound: BigUint = pow(BigUint::from(10_u32), 20);
    let mut highest: BigUint = 0_u32.into();

    let mut n: BigUint = 1_u32.into();
    let mut m: BigUint = 1_u32.into();
    let mut num_square_triangular = 0;

    while highest <= upper_search_bound {
        // println!("Searching n={n}, m={m}");
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
                println!(
                    "Found square triangular (#{num_square_triangular}): {square}, (n={n}, m={m})"
                );
                // assert!(&square == dp.get(num_square_triangular - 1).unwrap());
                highest = max(highest, square);
                n += 1_u32;
                m += 1_u32;
            }
        }
    }

    println!("Answer: {num_square_triangular}")
}
