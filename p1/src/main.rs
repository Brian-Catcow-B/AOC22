fn main() {
    let tmp = include_str!("input1.txt");
    let input_str: String = String::from(tmp);

    let best = find_best(&input_str);

    println!("part 1 solution: {}", best);

    let best3_sum = find_best3_sum(&input_str);

    println!("part 2 solution: {}", best3_sum);
}

fn find_best(s: &String) -> usize {
    let mut current: usize = 0;
    let mut best: usize = 0;
    for line in s.split('\n') {
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
    best
}

fn find_best3_sum(s: &String) -> usize {
    let mut current: usize = 0;
    let mut best3: [usize; 3] = [0, 0, 0];
    for line in s.split('\n') {
        if line.len() == 0 {
            for idx in 0..3 {
                if current > best3[idx] {
                    for i in ((idx + 1)..3).rev() {
                        best3[i] = best3[i - 1];
                    }
                    best3[idx] = current;
                    break;
                }
            }
            current = 0;
            continue;
        }
        current += match line.parse() {
            Ok(val) => val,
            Err(_) => 0,
        }
    }
    best3[0] + best3[1] + best3[2]
}

