// Entropy-based hint engine for suggesting optimal guesses
// Information-gain calculation for Wordle solver

use crate::engine::{evaluate_guess, LetterFeedback};
use std::collections::HashMap;

/// Result of analyzing hints for a game state
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HintAnalysis {
    /// Top suggested next guesses ranked by information gain
    pub top_suggestions: Vec<GuessHint>,
    /// Entropy of current game state (bits of information)
    pub current_entropy: f64,
    /// Letter frequency analysis
    pub letter_analysis: HashMap<char, LetterFrequency>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GuessHint {
    /// The suggested guess
    pub word: String,
    /// Expected entropy reduction (information gain)
    pub entropy_reduction: f64,
    /// Average remaining possibilities after this guess
    pub expected_remaining: f64,
    /// Explanation of why this word is good
    pub explanation: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LetterFrequency {
    /// How many words still contain this letter
    pub occurrences_in_remaining: usize,
    /// Position frequencies
    pub position_frequencies: [usize; 5],
}

/// Compute Shannon entropy of a distribution
fn compute_entropy(distribution: &HashMap<String, usize>) -> f64 {
    if distribution.is_empty() {
        return 0.0;
    }

    let total: usize = distribution.values().sum();
    if total == 0 {
        return 0.0;
    }

    let total_f = total as f64;
    distribution
        .values()
        .map(|&count| {
            if count == 0 {
                0.0
            } else {
                let p = count as f64 / total_f;
                -p * p.log2()
            }
        })
        .sum()
}

pub struct HintEngine {
    candidate_set: Vec<String>, // Smaller set for fast evaluation
}

impl HintEngine {
    pub fn new(word_list: Vec<String>) -> Self {
        // Use top 1000 most common words as candidates for efficiency
        let candidate_set = word_list.iter().take(1000).cloned().collect();
        Self { candidate_set }
    }

    /// Analyze the current game state and provide hints
    pub fn analyze(
        &self,
        remaining_words: &[String],
        _previous_guesses: &[(String, Vec<LetterFeedback>)],
    ) -> HintAnalysis {
        if remaining_words.is_empty() {
            return HintAnalysis {
                top_suggestions: vec![],
                current_entropy: 0.0,
                letter_analysis: HashMap::new(),
            };
        }

        // Calculate entropy of current state
        let mut partitions = HashMap::new();
        for word in remaining_words {
            *partitions.entry(word.clone()).or_insert(0) += 1;
        }

        let current_entropy = compute_entropy(&partitions);

        // Evaluate candidate guesses
        let mut suggestions = Vec::new();

        for candidate in &self.candidate_set {
            let mut pattern_counts: HashMap<String, usize> = HashMap::new();

            for answer in remaining_words.iter() {
                let eval = evaluate_guess(candidate, answer);
                let pattern = format!("{:?}", eval.feedback);
                *pattern_counts.entry(pattern).or_insert(0) += 1;
            }

            let entropy_after = compute_entropy(&pattern_counts);
            let entropy_reduction = current_entropy - entropy_after;

            let expected_remaining = pattern_counts
                .values()
                .map(|&count| count as f64)
                .sum::<f64>()
                / pattern_counts.len() as f64;

            suggestions.push(GuessHint {
                word: candidate.clone(),
                entropy_reduction,
                expected_remaining,
                explanation: format!(
                    "This guess has {} partitions, reducing entropy by {:.2} bits",
                    pattern_counts.len(),
                    entropy_reduction
                ),
            });
        }

        // Sort by entropy reduction (descending)
        suggestions.sort_by(|a, b| {
            b.entropy_reduction
                .partial_cmp(&a.entropy_reduction)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Analyze letter frequencies
        let letter_analysis = self.analyze_letters(remaining_words);

        HintAnalysis {
            top_suggestions: suggestions.into_iter().take(5).collect(),
            current_entropy,
            letter_analysis,
        }
    }

    fn analyze_letters(&self, words: &[String]) -> HashMap<char, LetterFrequency> {
        let mut analysis: HashMap<char, LetterFrequency> = HashMap::new();

        for word in words {
            for (pos, ch) in word.to_uppercase().chars().enumerate() {
                let freq = analysis.entry(ch).or_insert_with(|| LetterFrequency {
                    occurrences_in_remaining: 0,
                    position_frequencies: [0; 5],
                });

                if freq.occurrences_in_remaining == 0 {
                    freq.occurrences_in_remaining = 1;
                } else if !word.to_uppercase().chars().take(pos).any(|c| c == ch) {
                    freq.occurrences_in_remaining += 1;
                }

                if pos < 5 {
                    freq.position_frequencies[pos] += 1;
                }
            }
        }

        analysis
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entropy_calculation() {
        let mut dist = HashMap::new();
        dist.insert("A".to_string(), 1);
        dist.insert("B".to_string(), 1);

        let entropy = compute_entropy(&dist);
        assert!((entropy - 1.0).abs() < 0.01); // log2(2) = 1
    }

    #[test]
    fn test_hint_engine_basic() {
        let words = vec![
            "STALE".to_string(),
            "STEAL".to_string(),
            "SLEET".to_string(),
        ];

        let engine = HintEngine::new(words.clone());
        let analysis = engine.analyze(&words, &[]);

        assert!(analysis.current_entropy > 0.0);
        assert!(!analysis.top_suggestions.is_empty());
    }
}
