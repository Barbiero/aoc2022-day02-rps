use strum::Display;

#[derive(Debug, Eq, PartialEq, Display, Clone, Copy)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

pub trait Beats {
    fn beats(&self) -> Self;

    fn for_result(&self, result: GameResult) -> Self;
}

impl Beats for Hand {
    fn beats(&self) -> Self {
        match *self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }

    fn for_result(&self, result: GameResult) -> Self {
        match result {
            GameResult::Draw => *self,
            GameResult::Win => self.beats(),
            GameResult::Loss => self.beats().beats(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Display)]
pub enum GameResult {
    Win,
    Loss,
    Draw,
}

pub trait Reverse {
    fn reverse(&self) -> Self;
}
impl Reverse for GameResult {
    fn reverse(&self) -> Self {
        match *self {
            GameResult::Win => GameResult::Loss,
            GameResult::Loss => GameResult::Win,
            GameResult::Draw => GameResult::Draw,
        }
    }
}

pub trait Game {
    fn versus(&self, versus: Self) -> GameResult;
}

impl Game for Hand {
    fn versus(&self, versus: Self) -> GameResult {
        if *self == versus {
            return GameResult::Draw;
        }
        if self.beats() == versus {
            return GameResult::Win;
        }

        GameResult::Loss
    }
}

pub trait Score {
    fn score(&self) -> i32;
}
impl Score for Hand {
    fn score(&self) -> i32 {
        match *self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}

impl Score for GameResult {
    fn score(&self) -> i32 {
        match *self {
            GameResult::Win => 6,
            GameResult::Loss => 0,
            GameResult::Draw => 3,
        }
    }
}
