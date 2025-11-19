use num_bigint::BigUint;
use tracing::{debug, info, instrument};

/// Flip DNA bases: A↔G and C↔T
fn flip_bases(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'A' => 'G',
            'G' => 'A',
            'C' => 'T',
            'T' => 'C',
            _ => c,
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Parity {
    Even,
    Odd,
}

impl Parity {
    /// Flip the parity
    pub fn flip(self) -> Parity {
        match self {
            Parity::Even => Parity::Odd,
            Parity::Odd => Parity::Even,
        }
    }
}

impl From<usize> for Parity {
    fn from(n: usize) -> Self {
        if n % 2 == 0 {
            Parity::Even
        } else {
            Parity::Odd
        }
    }
}

impl From<&BigUint> for Parity {
    fn from(n: &BigUint) -> Self {
        let two = BigUint::from(2u32);
        if (n % &two) == BigUint::from(0u32) {
            Parity::Even
        } else {
            Parity::Odd
        }
    }
}

#[derive(Clone)]
pub struct Range {
    pub start: BigUint,
    pub end: BigUint,
    pub parity: Parity
}

pub struct SolveResult {
    pub substring: String,
}

pub struct Solver {
    range: Range,
    fibs: Vec<BigUint>,
    seqs: Vec<String>,
    n: BigUint,
}

impl Solver {
    pub fn new(range: Range, fibs: Vec<BigUint>, seqs: Vec<String>, n: usize) -> Self {
        Solver {
            range,
            fibs,
            seqs,
            n: BigUint::from(n),
        }
    }

    pub fn new_with_biguint(range: Range, fibs: Vec<BigUint>, seqs: Vec<String>, n: &BigUint) -> Self {
        Solver {
            range,
            fibs,
            seqs,
            n: n.clone(),
        }
    }

    #[instrument(skip(self, range), fields(range_start = %range.start, range_end = %range.end))]
    fn reduce_range(&self, range: &Range) -> (Range, Option<usize>) {
        self.reduce_range_with_count(range, 0).0
    }

    fn reduce_range_with_count(&self, range: &Range, count: usize) -> ((Range, Option<usize>), usize) {
        self.reduce_range_with_count_and_prev_index(range, count, None)
    }

    fn reduce_range_with_count_and_prev_index(&self, range: &Range, count: usize, prev_index: Option<usize>) -> ((Range, Option<usize>), usize) {
        // Find the largest Fibonacci number less than or equal to the start, with its index
        let result = self.fibs.iter()
            .enumerate()
            .rev()
            .find(|(_, f)| *f <= &range.start && **f > (range.end.clone() - range.start.clone()));

        match result {
            Some((index, fib)) => {
                debug!(
                    fib = %fib,
                    fib_index = index,
                    current_start = %range.start,
                    current_end = %range.end,
                    "Found Fibonacci number to subtract"
                );

                if let Some(next) = self.fibs.get(index + 1) {
                    if range.start <= *next && *next < range.end {
                        todo!("found bisecting fib ({}), between {}-{}", fib, range.start, range.end)
                    }
                }

                // Subtract from both start and end
                // Parity flips if the difference between previous and current index is odd
                let new_parity = if let Some(prev_idx) = prev_index {
                    let index_diff = prev_idx - index;
                    let diff_parity = Parity::from(index_diff);
                    if diff_parity == Parity::Odd {
                        range.parity.flip()
                    } else {
                        range.parity
                    }
                } else {
                    // First reduction, no flip
                    range.parity
                };

                let reduced = Range {
                    start: &range.start - fib,
                    end: &range.end - fib,
                    parity: new_parity,
                };

                debug!(
                    reduced_start = %reduced.start,
                    reduced_end = %reduced.end,
                    old_parity = ?range.parity,
                    prev_index = ?prev_index,
                    current_index = index,
                    new_parity = ?new_parity,
                    "After subtracting Fibonacci"
                );

                // Recursively reduce the new range
                let ((final_range, recursive_index), final_count) = self.reduce_range_with_count_and_prev_index(&reduced, count + 1, Some(index));
                // Use the index from the recursive call if it exists, otherwise use this one
                ((final_range, recursive_index.or(Some(index))), final_count)
            }
            None => {
                // No suitable Fibonacci number found - base case
                info!(reduction_count = count, final_start = %range.start, final_end = %range.end, "Range reduction complete - no Fibonacci number to subtract");
                ((range.clone(), None), count)
            }
        }
    }

    #[instrument(skip(self, range, fib_index), fields(range_start = %range.start, range_end = %range.end, fib_index))]
    fn find_sequence(&self, range: &Range, fib_index: usize) -> Option<(String, usize)> {
        info!(num_sequences = self.seqs.len(), fib_index, "Finding sequence with matching parity");

        let target_parity = Parity::from(fib_index);

        // Filter sequences by parity and length
        let matching_seq = self.seqs
            .iter()
            .enumerate()
            .filter(|(idx, _)| Parity::from(*idx) == target_parity)
            .filter(|(_, seq)| {
                let seq_len = BigUint::from(seq.len());
                seq_len >= range.end
            })
            .next();

        match matching_seq {
            Some((seq_index, seq)) => {
                info!(seq_index, seq_len = seq.len(), target_parity = ?target_parity, "Found matching sequence");
                Some((seq.clone(), seq_index))
            }
            None => {
                info!(target_parity = ?target_parity, required_end = %range.end, "No sequence with matching parity and sufficient length");
                None
            }
        }
    }

    #[instrument(skip(self), fields(initial_start = %self.range.start, initial_end = %self.range.end, n = %self.n))]
    pub fn solve(self) -> Option<SolveResult> {
        info!(num_fibs = self.fibs.len(), num_seqs = self.seqs.len(), n = %self.n, "Starting solve");

        // Reduce the range
        let (reduced_range, fib_index_opt) = self.reduce_range(&self.range);
        info!(reduced_start = %reduced_range.start, reduced_end = %reduced_range.end, fib_index = ?fib_index_opt, "Range reduced");

        // Determine which Fibonacci index to use for parity filtering
        // If reduction happened, use that index. Otherwise, use n's parity
        let fib_index = match fib_index_opt {
            Some(idx) => {
                info!(fib_index = idx, "Using Fibonacci index for parity filtering");
                idx
            }
            None => {
                // No reduction happened, use n directly
                // Convert n to index by finding which Fibonacci number it corresponds to
                info!(n = %self.n, "No reduction, computing parity from n");
                let n_parity = Parity::from(&self.n);
                // Return 0 for even, 1 for odd as a simple mapping
                match n_parity {
                    Parity::Even => 0,
                    Parity::Odd => 1,
                }
            }
        };

        // Find a sequence that can contain the range
        let (seq, seq_index) = self.find_sequence(&reduced_range, fib_index)?;

        // Extract the substring
        let start_idx: usize = reduced_range.start.to_string().parse().unwrap();
        let end_idx: usize = reduced_range.end.to_string().parse().unwrap();

        info!(start_idx, end_idx, seq_len = seq.len(), "Extracting substring");

        let mut substring = seq[start_idx..end_idx].to_string();

        // Check if we need to flip bases using the tracked parity from the reduced range
        // The range parity has been updated throughout all reductions
        let range_parity = reduced_range.parity;
        let seq_parity = Parity::from(seq_index);

        if range_parity == seq_parity {
            info!(range_parity = ?range_parity, seq_parity = ?seq_parity, "Range and sequence have matching parities, flipping bases: A↔G, C↔T");
            substring = flip_bases(&substring);
        } else {
            info!(range_parity = ?range_parity, seq_parity = ?seq_parity, "Range and sequence have different parities, no flip needed");
        }

        info!(substring_len = substring.len(), seq_index, "Solve complete");

        Some(SolveResult { substring })
    }
}
