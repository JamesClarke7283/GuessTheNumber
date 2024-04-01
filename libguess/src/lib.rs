use rand::prelude::*;
use std::cmp::Ordering;

/// Represents a number guessing game.
pub struct Game {
    pub min_num: u32,
    pub max_num: u32,
    pub lives: u32,
    pub rng: StdRng,
    pub secret_number: u32,
}

impl Game {
    pub const MIN_NUM: u32 = 1;
    pub const MAX_NUM: u32 = 20;
    pub const LIVES: u32 = 10;

    /// Creates a new instance of the `Game` struct.
    ///
    /// # Arguments
    ///
    /// * `min_num`: The minimum value for the secret number (default: 1).
    /// * `max_num`: The maximum value for the secret number (default: 20).
    /// * `lives`: The number of lives the player has (default: 10).
    /// * `rng`: The random number generator to use.
    ///
    /// # Returns
    ///
    /// A new instance of the `Game` struct.
    ///
    /// # Examples
    ///
    /// ```
    /// use libguess::{Game, GameTrait};
    /// use rand::SeedableRng;
    /// use rand::rngs::StdRng;
    ///
    /// let mut rng = StdRng::from_seed(Default::default());
    /// let game = Game::new(Some(1), Some(10), Some(5), &mut rng);
    /// assert_eq!(game.min_num(), 1);
    /// assert_eq!(game.max_num(), 10);
    /// assert_eq!(game.lives(), 5);
    ///
    /// let mut rng = StdRng::from_seed(Default::default());
    /// let default_game = Game::new(None, None, None, &mut rng);
    /// assert_eq!(default_game.min_num(), Game::MIN_NUM);
    /// assert_eq!(default_game.max_num(), Game::MAX_NUM);
    /// assert_eq!(default_game.lives(), Game::LIVES);
    /// ```
    pub fn new(min_num: Option<u32>, max_num: Option<u32>, lives: Option<u32>, rng: &mut StdRng) -> Self {
        let secret_number = rng.gen_range(min_num.unwrap_or(Self::MIN_NUM)..=max_num.unwrap_or(Self::MAX_NUM));
        Game {
            min_num: min_num.unwrap_or(Self::MIN_NUM),
            max_num: max_num.unwrap_or(Self::MAX_NUM),
            lives: lives.unwrap_or(Self::LIVES),
            rng: rng.clone(),
            secret_number,
        }
    }

    /// Returns a reference to the random number generator.
    pub fn rng(&self) -> &StdRng {
        &self.rng
    }
}

/// Represents the result of a single guess.
#[derive(Debug, PartialEq)]
pub enum GuessResult {
    Correct,
    TooHigh,
    TooLow,
    NoMoreLives,
}

/// Defines the behavior of the number guessing game.
pub trait GameTrait {
    /// Runs the number guessing game.
    ///
    /// # Arguments
    ///
    /// * `guess`: The player's guess.
    ///
    /// # Returns
    ///
    /// A `GuessResult` indicating the result of the guess.
    ///
    /// # Examples
    ///
    /// ```
    /// use libguess::{Game, GameTrait, GuessResult};
    /// use rand::SeedableRng;
    /// use rand::rngs::StdRng;
    ///
    /// let mut rng = StdRng::from_seed(Default::default());
    /// let mut game = Game {
    ///     min_num: 1,
    ///     max_num: 10,
    ///     lives: 3,
    ///     rng: rng.clone(),
    ///     secret_number: 7,
    /// };
    /// assert_eq!(game.play(5), GuessResult::TooLow);
    /// assert_eq!(game.play(7), GuessResult::Correct);
    /// ```
    fn play(&mut self, guess: u32) -> GuessResult;

    /// Returns the minimum value for the secret number.
    fn min_num(&self) -> u32;

    /// Returns the maximum value for the secret number.
    fn max_num(&self) -> u32;

    /// Returns the number of lives the player has.
    fn lives(&self) -> u32;
}

impl GameTrait for Game {
    fn play(&mut self, guess: u32) -> GuessResult {
        if self.lives() == 0 {
            return GuessResult::NoMoreLives;
        }

        let result = compare(guess, self.secret_number);
        if result != GuessResult::Correct {
            self.lives -= 1;
        }
        result
    }

    fn min_num(&self) -> u32 {
        self.min_num
    }

    fn max_num(&self) -> u32 {
        self.max_num
    }

    fn lives(&self) -> u32 {
        self.lives
    }
}

/// Performs the comparison between a guess and the secret number.
///
/// # Arguments
///
/// * `guess`: The player's guess.
/// * `secret`: The secret number to compare against.
///
/// # Returns
///
/// A `GuessResult` indicating whether the guess is correct, too high, or too low.
///
/// # Examples
///
/// ```
/// use libguess::compare;
/// use libguess::GuessResult;
///
/// assert_eq!(compare(5, 5), GuessResult::Correct);
/// assert_eq!(compare(4, 5), GuessResult::TooLow);
/// assert_eq!(compare(6, 5), GuessResult::TooHigh);
/// ```
pub fn compare(guess: u32, secret: u32) -> GuessResult {
    match guess.cmp(&secret) {
        Ordering::Equal => GuessResult::Correct,
        Ordering::Less => GuessResult::TooLow,
        Ordering::Greater => GuessResult::TooHigh,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let mut rng = StdRng::from_seed(Default::default());
        let game = Game::new(Some(1), Some(10), Some(5), &mut rng);
        assert_eq!(game.min_num(), 1);
        assert_eq!(game.max_num(), 10);
        assert_eq!(game.lives(), 5);

        let mut rng  = StdRng::from_seed(Default::default());
        let default_game = Game::new(None, None, None, &mut rng);
        assert_eq!(default_game.min_num(), Game::MIN_NUM);
        assert_eq!(default_game.max_num(), Game::MAX_NUM);
        assert_eq!(default_game.lives(), Game::LIVES);
    }

    #[test]
    fn test_play() {
        let secret_number = 3;
        let rng = StdRng::from_seed(Default::default());
        let mut game = Game {
            min_num: 1,
            max_num: 10,
            lives: 10,
            rng,
            secret_number,
        };

        for _ in 0..8 {
            assert_eq!(game.play(1), GuessResult::TooLow);
        }
        assert_eq!(game.play(20), GuessResult::TooHigh);
        assert_eq!(game.play(3), GuessResult::Correct);
        assert_eq!(game.play(1), GuessResult::TooLow);
    }

    #[test]
    fn test_compare() {
        let comparisons = [
            (5, 5, GuessResult::Correct),
            (4, 5, GuessResult::TooLow),
            (6, 5, GuessResult::TooHigh),
        ];
        for (guess, secret, result) in comparisons {
            assert_eq!(compare(guess, secret), result);
        }
    }
}