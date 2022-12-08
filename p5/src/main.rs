fn main() {
    let input_str = String::from(include_str!("input1.txt"));

    let str1 = solution1(&input_str);

    println!("solution 1: {}", str1);

    let str2 = solution2(&input_str);

    println!("solution 2: {}", str2);
}

fn fill(crates: &String, vec_of_stacks: &mut Vec<Vec<char>>) {
    let mut past_indeces_line = false;
    for line in crates.split('\n').rev() {
        let mut i: usize = 0;
        let len = line.len();
        while i + 2 < len {
            if !past_indeces_line {
                vec_of_stacks.push(vec![]);
            } else {
                if line.chars().nth(i).expect("") == '['
                    && line.chars().nth(i + 2).expect("") == ']'
                {
                    vec_of_stacks[i / 4].push(line.chars().nth(i + 1).expect(""));
                }
            }
            i += 4;
        }
        past_indeces_line = true;
    }
}

enum CmdPart {
    Null,
    MoveNum,
    FromCol,
    ToCol,
}

impl From<&str> for CmdPart {
    fn from(s: &str) -> CmdPart {
        match s {
            "move" => CmdPart::MoveNum,
            "from" => CmdPart::FromCol,
            "to" => CmdPart::ToCol,
            _ => CmdPart::Null,
        }
    }
}

fn run_command1(cmd: &String, vec_of_stacks: &mut Vec<Vec<char>>) {
    if cmd.len() == 0 {
        return;
    }
    let mut move_num: isize = -1;
    let mut from_col: isize = -1;
    let mut to_col: isize = -1;
    let mut cp: CmdPart = CmdPart::Null;
    for word in cmd.split(' ') {
        match cp {
            CmdPart::Null => cp = CmdPart::from(word),
            CmdPart::MoveNum => {
                move_num = word.parse().expect("");
                cp = CmdPart::Null;
            }
            CmdPart::FromCol => {
                from_col = word.parse().expect("");
                cp = CmdPart::Null;
            }
            CmdPart::ToCol => {
                to_col = word.parse().expect("");
                cp = CmdPart::Null;
            }
        }
    }
    if move_num == -1 || from_col == -1 || to_col == -1 {
        return;
    }
    for _ in 0..move_num {
        let c: char = vec_of_stacks[from_col as usize - 1].pop().expect("");
        vec_of_stacks[to_col as usize - 1].push(c);
    }
}

fn solution1(s: &String) -> String {
    let mut str_top_chars = String::new();
    let mut vec_of_stacks: Vec<Vec<char>> = vec![];
    let (crates, instructions) = s.split_at(s.find("\n\n").expect(""));
    fill(&String::from(crates), &mut vec_of_stacks);
    for line in String::from(instructions).split('\n') {
        run_command1(&String::from(line), &mut vec_of_stacks);
    }
    for v in vec_of_stacks.iter() {
        str_top_chars.push(*v.last().expect(""));
    }

    str_top_chars
}

fn run_command2(cmd: &String, vec_of_stacks: &mut Vec<Vec<char>>) {
    if cmd.len() == 0 {
        return;
    }
    let mut move_num: isize = -1;
    let mut from_col: isize = -1;
    let mut to_col: isize = -1;
    let mut cp: CmdPart = CmdPart::Null;
    for word in cmd.split(' ') {
        match cp {
            CmdPart::Null => cp = CmdPart::from(word),
            CmdPart::MoveNum => {
                move_num = word.parse().expect("");
                cp = CmdPart::Null;
            }
            CmdPart::FromCol => {
                from_col = word.parse().expect("");
                cp = CmdPart::Null;
            }
            CmdPart::ToCol => {
                to_col = word.parse().expect("");
                cp = CmdPart::Null;
            }
        }
    }
    if move_num == -1 || from_col == -1 || to_col == -1 {
        return;
    }
    let len = vec_of_stacks[from_col as usize - 1].len();
    let mut tmp_clump = vec_of_stacks[from_col as usize - 1].split_off(len - move_num as usize);
    vec_of_stacks[to_col as usize - 1].append(&mut tmp_clump);
}

fn solution2(s: &String) -> String {
    let mut str_top_chars = String::new();
    let mut vec_of_stacks: Vec<Vec<char>> = vec![];
    let (crates, instructions) = s.split_at(s.find("\n\n").expect(""));
    fill(&String::from(crates), &mut vec_of_stacks);
    for line in String::from(instructions).split('\n') {
        run_command2(&String::from(line), &mut vec_of_stacks);
    }
    for v in vec_of_stacks.iter() {
        str_top_chars.push(*v.last().expect(""));
    }

    str_top_chars
}
