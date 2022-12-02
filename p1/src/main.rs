fn main() {
    let tmp = include_str!("input1.txt");
    let input_str: String = String::from(tmp);

    let mut current: usize = 0;
    let mut best: usize = 0;
    for line in input_str.split('\n') {
        if line.len() == 0 {
            best = std::cmp::max(current, best);
            current = 0;
            continue;
        }
        current += match line.parse() {
            Ok(val) => val,
            Err(_) => 0,
        };
    }

    println!("{}", best)
}
