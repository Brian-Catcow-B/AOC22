fn main() {
    let input_str = String::from(include_str!("input1.txt"));

    let num1 = solution1(&input_str);

    println!("solution 1: {}", num1);

    let num2 = solution2(&input_str);

    println!("solution 2: {}", num2);
}

fn solution1(s: &String) -> usize {
    let mut i: usize = 4;
    let mut last4: [char; 4] = [
        s.chars().nth(0).expect(""),
        s.chars().nth(1).expect(""),
        s.chars().nth(2).expect(""),
        s.chars().nth(3).expect(""),
    ];
    let mut itr = s.chars();
    for _ in 0..4 {
        itr.next();
    }
    loop {
        let mut equal_chars_in_array = false;
        'j_loop: for j in 0..4 {
            for k in 0..j {
                if last4[j] == last4[k] {
                    equal_chars_in_array = true;
                    break 'j_loop;
                }
            }
        }
        if !equal_chars_in_array {
            break;
        }
        let c = itr.next().expect("");
        last4[i % 4] = c;
        i += 1;
    }

    i
}

fn solution2(s: &String) -> usize {
    let mut i: usize = 14;
    let mut last4: [char; 14] = [
        s.chars().nth(0).expect(""),
        s.chars().nth(1).expect(""),
        s.chars().nth(2).expect(""),
        s.chars().nth(3).expect(""),
        s.chars().nth(4).expect(""),
        s.chars().nth(5).expect(""),
        s.chars().nth(6).expect(""),
        s.chars().nth(7).expect(""),
        s.chars().nth(8).expect(""),
        s.chars().nth(9).expect(""),
        s.chars().nth(10).expect(""),
        s.chars().nth(11).expect(""),
        s.chars().nth(12).expect(""),
        s.chars().nth(13).expect(""),
    ];
    let mut itr = s.chars();
    for _ in 0..14 {
        itr.next();
    }
    loop {
        let mut equal_chars_in_array = false;
        'j_loop: for j in 0..14 {
            for k in 0..j {
                if last4[j] == last4[k] {
                    equal_chars_in_array = true;
                    break 'j_loop;
                }
            }
        }
        if !equal_chars_in_array {
            break;
        }
        let c = itr.next().expect("");
        last4[i % 14] = c;
        i += 1;
    }

    i
}
