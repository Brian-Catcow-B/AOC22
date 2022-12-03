#[derive(PartialEq, Eq, Copy, Clone)]
enum RPSOption {
    Null,
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum RPSResult {
    Null,
    Win,
    Draw,
    Loss,
}

impl From<char> for RPSOption {
    fn from(c: char) -> RPSOption {
        match c {
            'a' | 'A' | 'x' | 'X' => RPSOption::Rock,
            'b' | 'B' | 'y' | 'Y' => RPSOption::Paper,
            'c' | 'C' | 'z' | 'Z' => RPSOption::Scissors,
            _ => RPSOption::Null,
        }
    }
}

impl From<char> for RPSResult {
    fn from(c: char) -> RPSResult {
        match c {
            'x' | 'X' => RPSResult::Loss,
            'y' | 'Y' => RPSResult::Draw,
            'z' | 'Z' => RPSResult::Win,
            _ => RPSResult::Null,
        }
    }
}

impl RPSOption {
    fn score(self) -> usize {
        match self {
            RPSOption::Rock => 1,
            RPSOption::Paper => 2,
            RPSOption::Scissors => 3,
            _ => 0,
        }
    }

    fn result_versus(self, other: Self) -> RPSResult {
        match self {
            RPSOption::Rock => match other {
                RPSOption::Rock => RPSResult::Draw,
                RPSOption::Paper => RPSResult::Loss,
                RPSOption::Scissors => RPSResult::Win,
                _ => RPSResult::Null,
            },
            RPSOption::Paper => match other {
                RPSOption::Rock => RPSResult::Win,
                RPSOption::Paper => RPSResult::Draw,
                RPSOption::Scissors => RPSResult::Loss,
                _ => RPSResult::Null,
            },
            RPSOption::Scissors => match other {
                RPSOption::Rock => RPSResult::Loss,
                RPSOption::Paper => RPSResult::Win,
                RPSOption::Scissors => RPSResult::Draw,
                _ => RPSResult::Null,
            },
            _ => RPSResult::Null,
        }
    }

    fn what_obtains_result_versus_this(self, result: RPSResult) -> RPSOption {
        match self {
            RPSOption::Rock => match result {
                RPSResult::Win => RPSOption::Paper,
                RPSResult::Draw => RPSOption::Rock,
                RPSResult::Loss => RPSOption::Scissors,
                _ => RPSOption::Null,
            },
            RPSOption::Paper => match result {
                RPSResult::Win => RPSOption::Scissors,
                RPSResult::Draw => RPSOption::Paper,
                RPSResult::Loss => RPSOption::Rock,
                _ => RPSOption::Null,
            },
            RPSOption::Scissors => match result {
                RPSResult::Win => RPSOption::Rock,
                RPSResult::Draw => RPSOption::Scissors,
                RPSResult::Loss => RPSOption::Paper,
                _ => RPSOption::Null,
            },
            _ => RPSOption::Null,
        }
    }
}

impl RPSResult {
    fn score(self) -> usize {
        match self {
            RPSResult::Win => 6,
            RPSResult::Draw => 3,
            RPSResult::Loss => 0,
            _ => 0,
        }
    }
}

fn main() {
    let input_str = String::from(include_str!("input1.txt"));

    let sum1: usize = solution1(&input_str);

    println!("solution1: {}", sum1);

    let sum2: usize = solution2(&input_str);

    println!("solution2: {}", sum2);
}

fn solution1(s: &String) -> usize {
    let mut score: usize = 0;
    let mut their_choice: RPSOption;
    let mut our_choice: RPSOption;
    let mut result: RPSResult;
    for line in s.split('\n') {
        if line.len() != 3 {
            break;
        }
        their_choice = RPSOption::from(line.chars().nth(0).expect(""));
        our_choice = RPSOption::from(line.chars().nth(2).expect(""));
        if their_choice == RPSOption::Null || our_choice == RPSOption::Null {
            break;
        }
        result = our_choice.result_versus(their_choice);
        if result == RPSResult::Null {
            break;
        }
        score += our_choice.score() + result.score();
    }
    score
}

fn solution2(s: &String) -> usize {
    let mut score: usize = 0;
    let mut their_choice: RPSOption;
    let mut our_choice: RPSOption;
    let mut result: RPSResult;
    for line in s.split('\n') {
        if line.len() != 3 {
            break;
        }
        their_choice = RPSOption::from(line.chars().nth(0).expect(""));
        result = RPSResult::from(line.chars().nth(2).expect(""));
        if their_choice == RPSOption::Null || result == RPSResult::Null {
            break;
        }
        our_choice = their_choice.what_obtains_result_versus_this(result);
        if our_choice == RPSOption::Null {
            break;
        }
        score += our_choice.score() + result.score();
    }
    score
}
