mod hand;
use std::{
    fs::File,
    io::{BufRead, BufReader, Lines, Result},
    path::Path,
};

use hand::*;

fn parse_opponent_hand(value: &str) -> Hand {
    match value {
        "A" => Hand::Rock,
        "B" => Hand::Paper,
        "C" => Hand::Scissors,
        _ => panic!("invalid opponent hand value"),
    }
}

fn parse_my_hand(value: &str) -> Hand {
    match value {
        "X" => Hand::Rock,
        "Y" => Hand::Paper,
        "Z" => Hand::Scissors,
        _ => panic!("invalid hand value"),
    }
}

fn parse_hands_from_line(line: String) -> (Hand, Hand) {
    let mut opp_my = line.split(char::is_whitespace);
    let opp_hand = opp_my.next().unwrap();
    let my_hand = opp_my.next().unwrap();
    assert_eq!(None, opp_my.next());

    (parse_opponent_hand(opp_hand), parse_my_hand(my_hand))
}

const INPUT_FILENAME: &str = "./inputs/input.txt";

/** from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html */
fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;

    Ok(BufReader::new(file).lines())
}

fn part1() -> Result<()> {
    let lines_buffer = read_lines(INPUT_FILENAME)?;
    
    let total_score = lines_buffer.flatten().map(parse_hands_from_line).fold(
        0,
        |curr_score, (opp_hand, my_hand)| {
            curr_score + my_hand.versus(opp_hand).score() + my_hand.score()
        },
    );

    println!("Total score (part1): {}", total_score);
    Ok(())
}

fn parse_expected_result(value: &str) -> GameResult {
    match value {
        "X" => GameResult::Loss,
        "Y" => GameResult::Draw,
        "Z" => GameResult::Win,
        _ => panic!("invalid game result value"),
    }
}

fn parse_hand_and_expected_result_from_line(line: String) -> (Hand, GameResult) {
    let mut opp_my = line.split(char::is_whitespace);
    let opp_hand = opp_my.next().unwrap();
    let my_hand = opp_my.next().unwrap();
    assert_eq!(None, opp_my.next());

    (
        parse_opponent_hand(opp_hand),
        parse_expected_result(my_hand),
    )
}

fn part2() -> Result<()> {
    let lines_buffer = read_lines(INPUT_FILENAME)?;

    let total_score = lines_buffer
        .flatten()
        .map(parse_hand_and_expected_result_from_line)
        .fold(0, |curr_score, (opp_hand, expected_result)| {
            let result_for_opponent = expected_result.reverse();
            let my_hand = opp_hand.for_result(result_for_opponent);

            curr_score + my_hand.versus(opp_hand).score() + my_hand.score()
        });

    println!("Total score (part1): {}", total_score);
    Ok(())
}

fn main() -> Result<()> {
    part1()?;
    part2()?;
    Ok(())
}
