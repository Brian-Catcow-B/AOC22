fn main() {
    let input_str = String::from(include_str!("input1.txt"));

    let sum1 = solution1(&input_str);

    println!("solution 1: {}", sum1);
}

enum Cmd {
    Null,
    Ls,
    Cd,
}

impl From<&str> for Cmd {
    fn from(s: &str) -> Cmd {
        match s {
            "ls" => Cmd::Ls,
            "cd" => Cmd::Cd,
            _ => Cmd::Null,
        }
    }
}

fn solution1_rec(

fn solution1(s: &String) -> usize {
    let mut sum: usize = 0;
    let mut current_path = "";

    sum
}
