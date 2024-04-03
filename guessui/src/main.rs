use iced::widget::{Button, Column, Text, TextInput};
use iced::alignment::Alignment;
use iced::{Element, Sandbox, Settings};
use libguess::{Game, GameTrait, GuessResult};
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn main() -> iced::Result {
    GuessUI::run(Settings::default())
}

struct GuessUI {
    game: Game,
    guess_input: String,
    message: String,
}

#[derive(Debug, Clone)]
enum Message {
    GuessInputChanged(String),
    GuessButtonClicked,
    PlayAgainButtonClicked,
}

impl Sandbox for GuessUI {
    type Message = Message;

fn new() -> Self {
    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let mut rng = StdRng::seed_from_u64(seed);
    let game = Game::new(None, None, None, &mut rng);
    Self {
        game,
        guess_input: String::new(),
        message: String::new(),
    }
}

    fn title(&self) -> String {
        String::from("Guess the Number")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::GuessInputChanged(value) => {
                self.guess_input = value;
            }
            Message::GuessButtonClicked => {
                if let Ok(guess) = self.guess_input.trim().parse() {
                    let result = self.game.play(guess);
                    match result {
                        GuessResult::Correct => {
                            self.message = "Congratulations! You guessed the number!".to_string();
                        }
                        GuessResult::TooHigh => {
                            self.message = "Too high! Try again.".to_string();
                        }
                        GuessResult::TooLow => {
                            self.message = "Too low! Try again.".to_string();
                        }
                        GuessResult::NoMoreLives => {
                            self.message =
                                "No more lives left. The secret number was ".to_string()
                                    + &self.game.secret_number.to_string();
                        }
                    }
                } else {
                    self.message = "Please enter a valid number.".to_string();
                }
                self.guess_input.clear();
            }
            Message::PlayAgainButtonClicked => {
                let mut rng = StdRng::from_seed(Default::default());
                self.game = Game::new(None, None, None, &mut rng);
                self.message.clear();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let mut content = Column::new()
            .padding(20)
            .align_items(Alignment::Center)
            .spacing(10);

        content = content.push(
            Text::new(format!(
                "Guess a number between {} and {}:",
                self.game.min_num(),
                self.game.max_num()
            ))
            .size(18),
        );

        content = content.push(
            TextInput::new("Guess", &self.guess_input)
                .on_input(Message::GuessInputChanged)
                .padding(10)
                .size(30),
        );

        content = content.push(
            Button::new(Text::new("Guess"))
                .on_press(Message::GuessButtonClicked)
                .padding(10),
        );

        if !self.message.is_empty() {
            content = content.push(Text::new(&self.message).size(18));
        }

        if self.game.lives() == 0 {
            content = content.push(
                Button::new(Text::new("Play Again"))
                    .on_press(Message::PlayAgainButtonClicked)
                    .padding(10),
            );
        }

        content.into()
    }
}