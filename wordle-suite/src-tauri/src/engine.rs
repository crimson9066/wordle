// Core game engine module
// Pure functions for Wordle logic evaluation

use std::collections::HashMap;

/// Represents the feedback for each letter in a guess
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum LetterFeedback {
    /// Letter is not in the answer
    Absent,
    /// Letter is in the answer but wrong position
    Present,
    /// Letter is in the answer and correct position
    Correct,
}

/// Result of evaluating a guess against an answer
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GuessEvaluation {
    pub guess: String,
    pub feedback: Vec<LetterFeedback>,
    pub is_correct: bool,
}

/// Evaluates a guess against the answer using a two-pass algorithm
/// 
/// # Algorithm
/// Pass 1: Mark all correct positions (greens)
/// Pass 2: Mark present letters (yellows), decrementing available counts
///
/// # Arguments
/// * `guess` - The guessed word (must be valid 5-letter word)
/// * `answer` - The actual answer (must be valid 5-letter word)
///
/// # Returns
/// GuessEvaluation with feedback for each position
///
/// # Example
/// ```
/// let evaluation = evaluate_guess("SLATE", "STALE");
/// assert_eq!(evaluation.feedback[0], LetterFeedback::Correct);
/// assert_eq!(evaluation.feedback[2], LetterFeedback::Present);
/// ```
pub fn evaluate_guess(guess: &str, answer: &str) -> GuessEvaluation {
    let guess_upper = guess.to_uppercase();
    let answer_upper = answer.to_uppercase();
    
    let guess_chars: Vec<char> = guess_upper.chars().collect();
    let answer_chars: Vec<char> = answer_upper.chars().collect();
    
    let mut feedback = vec![LetterFeedback::Absent; 5];
    let mut answer_letter_counts = HashMap::new();
    
    // Count all letters in answer
    for &ch in &answer_chars {
        *answer_letter_counts.entry(ch).or_insert(0) += 1;
    }
    
    // Pass 1: Mark correct positions and decrement counts
    for (i, &guess_char) in guess_chars.iter().enumerate() {
        if guess_char == answer_chars[i] {
            feedback[i] = LetterFeedback::Correct;
            if let Some(count) = answer_letter_counts.get_mut(&guess_char) {
                *count -= 1;
            }
        }
    }
    
    // Pass 2: Mark present letters (considering remaining counts)
    for (i, &guess_char) in guess_chars.iter().enumerate() {
        if feedback[i] == LetterFeedback::Absent {
            if let Some(&count) = answer_letter_counts.get(&guess_char) {
                if count > 0 {
                    feedback[i] = LetterFeedback::Present;
                    answer_letter_counts.insert(guess_char, count - 1);
                }
            }
        }
    }
    
    let is_correct = feedback.iter().all(|&f| f == LetterFeedback::Correct);
    
    GuessEvaluation {
        guess: guess_upper,
        feedback,
        is_correct,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_exact_match() {
        let eval = evaluate_guess("STALE", "STALE");
        assert!(eval.is_correct);
        assert!(eval.feedback.iter().all(|&f| f == LetterFeedback::Correct));
    }
    
    #[test]
    fn test_no_matches() {
        let eval = evaluate_guess("ABCDE", "FGHIJ");
        assert!(!eval.is_correct);
        assert!(eval.feedback.iter().all(|&f| f == LetterFeedback::Absent));
    }
    
    #[test]
    fn test_present_letters() {
        let eval = evaluate_guess("SLATE", "STALE");
        assert!(!eval.is_correct);
        assert_eq!(eval.feedback[0], LetterFeedback::Correct);  // S
        assert_eq!(eval.feedback[1], LetterFeedback::Correct);  // T
        assert_eq!(eval.feedback[2], LetterFeedback::Present);  // A (wrong position)
        assert_eq!(eval.feedback[3], LetterFeedback::Correct);  // L
        assert_eq!(eval.feedback[4], LetterFeedback::Correct);  // E
    }
    
    #[test]
    fn test_duplicate_letter_handling() {
        // Answer has 1 L, guess has 2 L's - only first should be correct, second absent
        let eval = evaluate_guess("LLAMA", "STALE");
        assert_eq!(eval.feedback[0], LetterFeedback::Absent);   // L (no L at position 0)
        assert_eq!(eval.feedback[1], LetterFeedback::Absent);   // L (only 1 L in answer)
        assert_eq!(eval.feedback[2], LetterFeedback::Present);  // A
        assert_eq!(eval.feedback[3], LetterFeedback::Correct);  // L
        assert_eq!(eval.feedback[4], LetterFeedback::Absent);   // A (already used)
    }
    
    #[test]
    fn test_case_insensitive() {
        let eval1 = evaluate_guess("stale", "STALE");
        let eval2 = evaluate_guess("STALE", "stale");
        assert_eq!(eval1.feedback, eval2.feedback);
    }
}
