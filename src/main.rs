use std::cmp::max;

fn main() {
    let upper_search_bound: i64 = 10_i64.checked_pow(10).unwrap();

    let mut highest: i64 = 0;
    let mut n: i64 = 1;
    let mut m: i64 = 1;
    let mut num_square_triangular = 0;

    while highest <= upper_search_bound {
        // println!("Searching n={n}, m={m}");
        let square = square_it(n);
        let triangular = triangular_it(m);

        match square.cmp(&triangular) {
            std::cmp::Ordering::Less => {
                highest = max(highest, triangular);
                n += 1;
            }
            std::cmp::Ordering::Greater => {
                highest = max(highest, square);
                m += 1;
            }
            std::cmp::Ordering::Equal => {
                num_square_triangular += 1;
                println!("Found square triangular (#{num_square_triangular})");
                highest = max(highest, square);
                n += 1;
                m += 1;
            }
        }
    }

    println!("Answer: {num_square_triangular}")
}

fn square_it(n: i64) -> i64 {
    n.checked_pow(2).unwrap()
}

fn triangular_it(m: i64) -> i64 {
    (m * (m + 1)) / 2
}
