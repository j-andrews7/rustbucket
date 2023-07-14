use extendr_api::prelude::*;
use rayon::prelude::*;

/// Compute the Hamming distance between a sequence and a vector of sequences with a limit. Complexity: O(n).
/// @export
#[extendr]
fn hamming(alpha: &str, beta: Vec<String>, limit: u64) -> Vec<u64> {
    let alpha_bytes = alpha.as_bytes();

    beta.par_iter().map(|seq| {
        assert_eq!(
            alpha.len(),
            seq.len(),
            "hamming distance cannot be calculated for texts of different length ({}!={})",
            alpha.len(),
            seq.len()
        );

        let seq_bytes = seq.as_bytes();

        let mut dist = 0;
        for (a, b) in alpha_bytes.iter().zip(seq_bytes) {
            if a != b {
                dist += 1;
                if dist > limit {
                    break;
                }
            }
        }
        if dist > limit { return limit + 1 }
        dist
    }).collect()
}

// Macro to generate exports
extendr_module! {
    mod rustbucket;
    fn hamming;
}
