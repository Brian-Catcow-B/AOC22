fn main() {
    let input_str = String::from(include_str!("input1.txt"));

    let sum1 = solution1(&input_str);

    println!("first sum: {}", sum1);

    let sum2 = solution2(&input_str);

    println!("second sum: {}", sum2);
}

fn priority_from_char(c: char) -> usize {
    if c >= 'a' && c <= 'z' {
        return c as usize - 'a' as usize + 1;
    }
    if c >= 'A' && c <= 'Z' {
        return c as usize - 'A' as usize + 27;
    }
    0
}

fn solution1(s: &String) -> usize {
    let mut total_priority: usize = 0;
    'line_loop: for line in s.split('\n') {
        if line.len() == 0 {
            break;
        }
        for first_c_idx in 0..(line.len() / 2) {
            let c: char = line.chars().nth(first_c_idx).expect("");
            // second half
            for second_c_idx in (line.len() / 2)..line.len() {
                if c == line.chars().nth(second_c_idx).expect("") {
                    total_priority += priority_from_char(c);
                    continue 'line_loop;
                }
            }
        }
    }
    total_priority
}

fn char_exists_in_str(c: char, s: &str) -> bool {
    for c_idx in 0..s.len() {
        if s.chars().nth(c_idx).expect("") == c {
            return true;
        }
    }
    false
}

fn solution2(s: &String) -> usize {
    let mut total_priority: usize = 0;
    let mut i: isize = -1;
    let split = s.split('\n');
    for line in split {
        i += 1;
        if i % 3 != 0 {
            continue;
        }
        for c in line.chars() {
            if char_exists_in_str(c, s.split('\n').nth(i as usize + 1).expect(""))
                && char_exists_in_str(c, s.split('\n').nth(i as usize + 2).expect(""))
            {
                total_priority += priority_from_char(c);
                break;
            }
        }
    }
    total_priority
}
