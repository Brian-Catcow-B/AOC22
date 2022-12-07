fn main() {
    let input_str = String::from(include_str!("input1.txt"));

    let str1 = solution1(&input_str);

    println!("solution 1: {}", str1);
}

fn fill(crates: &String, vec_of_stacks: &mut Vec<Vec<char>>) {
    'line_loop: for line in crates.split('\n').rev() {
        let mut i: usize = 0;
        let len = line.len();
        while i + 2 < len {
            if line.chars().nth(i).expect("") != '[' || line.chars().nth(i + 2).expect("") != ']' {
                vec_of_stacks.push(vec![]);
            } else {
                vec_of_stacks[i / 4].push(line.chars().nth(i + 1).expect(""));
            }
            i += 4;
        }
    }
}

fn solution1(s: &String) -> String {
    let mut str_top_chars = String::new();
    let mut vec_of_stacks: Vec<Vec<char>> = vec![];
    let (crates, instructions) = s.split_at(s.find("\n\n").expect(""));
    fill(&String::from(crates), &mut vec_of_stacks);

    str_top_chars
}
