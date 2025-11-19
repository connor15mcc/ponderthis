use num_bigint::BigUint;
use serde::Deserialize;
use ponderthis_nov::{Parity, Range, Solver};
use tracing_subscriber::layer::SubscriberExt;
use std::sync::{Arc, Mutex};

#[derive(Debug, Deserialize)]
struct TestCase {
    range: [u32; 2],
    seqs: Vec<String>,
    expected: String,
    n: usize,
}

#[derive(Debug, Deserialize, Default)]
struct TestCases {
    #[serde(default)]
    test: Vec<TestCase>,
}

fn parse_fibs() -> Vec<BigUint> {
    const FIB_DATA: &str = include_str!("../fib.txt");

    FIB_DATA
        .lines()
        .filter_map(|line| {
            line.trim().parse::<BigUint>().ok()
        })
        .collect()
}

#[derive(Clone)]
struct LogCapture {
    buffer: Arc<Mutex<Vec<String>>>,
}

impl LogCapture {
    fn new() -> Self {
        Self {
            buffer: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn get_logs(&self) -> Vec<String> {
        self.buffer.lock().unwrap().clone()
    }
}

impl std::io::Write for LogCapture {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let s = String::from_utf8_lossy(buf).to_string();
        self.buffer.lock().unwrap().push(s);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[test]
fn run_solver_tests() {
    // Load test cases from TOML
    let test_data = include_str!("solver.toml");
    let test_cases: TestCases = toml::from_str(test_data)
        .expect("Failed to parse solver.toml");

    // Load fibonacci numbers (shared across all tests)
    let fibs = parse_fibs();

    // If no test cases, pass the test
    if test_cases.test.is_empty() {
        println!("No test cases found in solver.toml");
        return;
    }

    // Run each test case
    for (idx, case) in test_cases.test.iter().enumerate() {
        // Create a log capture for this test
        let log_capture = LogCapture::new();
        let log_capture_clone = log_capture.clone();

        // Initialize tracing for this test case with capturing
        let subscriber = tracing_subscriber::registry()
            .with(
                tracing_subscriber::fmt::layer()
                    .json()
                    .with_target(false)
                    .with_current_span(true)
                    .with_span_list(true)
                    .with_writer(move || log_capture_clone.clone())
            )
            .with(tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::DEBUG.into()));

        // Set subscriber for this test
        let _guard = tracing::subscriber::set_default(subscriber);

        // Create range
        let range = Range {
            parity: Parity::from(case.n),
            start: BigUint::from(case.range[0]),
            end: BigUint::from(case.range[1]),
        };

        // Create solver
        let solver = Solver::new(range, fibs.clone(), case.seqs.clone(), case.n);

        // Solve and verify result
        let result = solver.solve();

        match result {
            Some(res) => {
                if res.substring != case.expected {
                    // Test failed - print logs
                    eprintln!("Test {} FAILED:", idx + 1);
                    eprintln!("Expected: {}", case.expected);
                    eprintln!("Got: {}", res.substring);
                    eprintln!("\nLogs:");
                    for log in log_capture.get_logs() {
                        eprintln!("{}", log);
                    }
                    panic!(
                        "Test {} failed: expected '{}', got '{}'",
                        idx + 1, case.expected, res.substring
                    );
                }
            }
            None => {
                // Test failed - print logs
                eprintln!("Test {} FAILED: solver returned None", idx + 1);
                eprintln!("\nLogs:");
                for log in log_capture.get_logs() {
                    eprintln!("{}", log);
                }
                panic!("Test {} failed to solve", idx + 1);
            }
        }
    }
}
