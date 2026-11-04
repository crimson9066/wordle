// Core game engine module
// This has all the pure logic for evaluating guesses and getting feedback

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
/// This function handles duplicate letters correctly by first marking
/// all exact matches (green), then marking present letters (yellow)
/// while tracking how many of each letter are available
///
/// Pass 1: Mark all correct positions (greens)
/// Pass 2: Mark present letters (yellows), keeping track of letter counts
///
/// Arguments:
/// * `guess` - The guessed word (must be 5 letters)
/// * `answer` - The actual answer (must be 5 letters)
///
/// Returns:
/// GuessEvaluation with feedback for each position
///
/// Example:
/// ```
/// let evaluation = evaluate_guess("SLATE", "STALE");
/// assert_eq!(evaluation.feedback[0], LetterFeedback::Correct);  // S matches
/// assert_eq!(evaluation.feedback[2], LetterFeedback::Present);  // A is present but wrong spot
/// ```
pub fn evaluate_guess(guess: &str, answer: &str) -> GuessEvaluation {
    let guess_upper = guess.to_uppercase();
    let answer_upper = answer.to_uppercase();
    
    let guess_chars: Vec<char> = guess_upper.chars().collect();
    let answer_chars: Vec<char> = answer_upper.chars().collect();
    
    let mut feedback = vec![LetterFeedback::Absent; 5];
    let mut answer_letter_counts = HashMap::new();
    
    // Count how many times each letter appears in the answer
    // We need this to handle duplicates correctly
    for &ch in &answer_chars {
        *answer_letter_counts.entry(ch).or_insert(0) += 1;
    }
    
    // Pass 1: Mark correct positions (greens) and decrement counts
    // We do this first so greens take priority over yellows
    for (i, &guess_char) in guess_chars.iter().enumerate() {
        if guess_char == answer_chars[i] {
            feedback[i] = LetterFeedback::Correct;
            // Don't count this letter again for yellows
            if let Some(count) = answer_letter_counts.get_mut(&guess_char) {
                *count -= 1;
            }
        }
    }
    
    // Pass 2: Mark present letters (yellows) using remaining counts
    // This ensures we don't mark a letter yellow if all instances were already used as green
    for (i, &guess_char) in guess_chars.iter().enumerate() {
        if feedback[i] == LetterFeedback::Absent {
            // Only mark as present if we have available instances of this letter
            if let Some(&count) = answer_letter_counts.get(&guess_char) {
                if count > 0 {
                    feedback[i] = LetterFeedback::Present;
                    answer_letter_counts.insert(guess_char, count - 1);
                }
            }
        }
    }
    
    // Check if all letters are correct (win condition)
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
        assert_eq!(eval.feedback[1], LetterFeedback::Present);  // L
        assert_eq!(eval.feedback[2], LetterFeedback::Correct);  // A
        assert_eq!(eval.feedback[3], LetterFeedback::Present);  // T
        assert_eq!(eval.feedback[4], LetterFeedback::Correct);  // E
    }
    
    #[test]
    fn test_duplicate_letter_handling() {
        // Answer has 1 L, guess has 2 L's - only first should be correct, second absent
        let eval = evaluate_guess("LLAMA", "STALE");
        assert_eq!(eval.feedback[0], LetterFeedback::Present);  // L
        assert_eq!(eval.feedback[1], LetterFeedback::Absent);   // L (only 1 L in answer)
        assert_eq!(eval.feedback[2], LetterFeedback::Correct);  // A
        assert_eq!(eval.feedback[3], LetterFeedback::Absent);   // M
        assert_eq!(eval.feedback[4], LetterFeedback::Absent);   // A (already used)
    }
    
    #[test]
    fn test_case_insensitive() {
        let eval1 = evaluate_guess("stale", "STALE");
        let eval2 = evaluate_guess("STALE", "stale");
        assert_eq!(eval1.feedback, eval2.feedback);
    }
}
