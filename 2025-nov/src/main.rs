use num_bigint::BigUint;
use ponderthis_nov::{Range, Solver};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn parse_fibs() -> Vec<BigUint> {
    const FIB_DATA: &str = include_str!("../fib.txt");

    FIB_DATA
        .lines()
        .filter_map(|line| {
            line.trim().parse::<BigUint>().ok()
        })
        .collect()
}

fn parse_seqs() -> Vec<String> {
    const SEQ_DATA: &str = include_str!("../seqs.txt");

    SEQ_DATA
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn main() {
    // Initialize tracing with JSON formatting
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_target(false)
                .with_current_span(true)
                .with_span_list(true)
        )
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Hardcoded values: n = 10^100, start = 10^100, end = 10^100 + 1000
    let n = BigUint::from(10u32).pow(100);
    let start = BigUint::from(10u32).pow(100);
    let end = &start + BigUint::from(1000u32);

    let fibs = parse_fibs();
    let seqs = parse_seqs();
    let range = Range { start, end };

    // Create solver and solve
    let solver = Solver::new_with_biguint(range, fibs, seqs, &n);

    match solver.solve() {
        Some(result) => {
            println!("{}", result.substring);
        }
        None => {
            eprintln!("Unable to solve");
            std::process::exit(1);
        }
    }
}
