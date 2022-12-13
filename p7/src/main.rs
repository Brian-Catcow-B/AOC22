fn main() {
    let input_str = String::from(include_str!("input1.txt"));

    let sum1 = solution1(&input_str);

    println!("solution 1: {}", sum1);

    let optimal2 = solution2(&input_str);

    println!("solution 2: {}", optimal2);
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
    split_input_str_itr: &mut std::str::Split<'_, char>,
    current_dir: String,
    total_sum_of_le100k: &mut usize,
) -> usize {
    let mut current_dir_total_size: usize = 0;
    let mut current_cmd: Cmd = Cmd::Null;

    loop {
        match split_input_str_itr.next() {
            Some(l) => {
                let line = String::from(l);
                let mut line_itr = line.split(' ').into_iter();
                match line_itr.next() {
                    Some(maybe_dollar_tok) => {
                        if let None = maybe_dollar_tok.chars().nth(0) {
                            break;
                        }
                        if maybe_dollar_tok.chars().nth(0).expect("") == '$' {
                            current_cmd = Cmd::from(line_itr.next().expect(""));
                            match current_cmd {
                                Cmd::Ls => {}
                                Cmd::Cd => {
                                    let cd_arg = line_itr.next().expect("");
                                    let next_dir;
                                    match cd_arg {
                                        "/" => {
                                            next_dir = "/".to_string();
                                        }
                                        ".." => {
                                            break;
                                        }
                                        _ => {
                                            next_dir =
                                                current_dir.clone() + &cd_arg + &"/".to_string();
                                        }
                                    }
                                    current_dir_total_size += solution1_rec(
                                        split_input_str_itr,
                                        next_dir,
                                        total_sum_of_le100k,
                                    );
                                }
                                Cmd::Null => {
                                    println!("weird command? {}", line)
                                }
                            }
                        } else {
                            match current_cmd {
                                Cmd::Ls => {
                                    let mut line_itr_ls = line.split(' ').into_iter();
                                    let tok = line_itr_ls.next().expect("");
                                    match tok.parse::<usize>() {
                                        Ok(file_size) => current_dir_total_size += file_size,
                                        _ => {}
                                    }
                                }
                                _ => {
                                    println!("weird command state? {}", line)
                                }
                            }
                        }
                    }
                    None => {}
                }
            }
            None => {
                break;
            }
        }
    }

    if current_dir_total_size <= 100_000 {
        *total_sum_of_le100k += current_dir_total_size;
    }

    current_dir_total_size
}

fn solution1(s: &String) -> usize {
    let mut sum_of_le100k: usize = 0;
    let mut split_input_str_itr = s.split('\n').into_iter();

    solution1_rec(
        &mut split_input_str_itr,
        String::from("/"),
        &mut sum_of_le100k,
    );

    sum_of_le100k
}

fn solution2_rec(
    split_input_str_itr: &mut std::str::Split<'_, char>,
    current_dir: String,
    most_optimal_dir_size: &mut usize,
    target_size: usize,
) -> usize {
    let mut current_dir_total_size: usize = 0;
    let mut current_cmd: Cmd = Cmd::Null;

    loop {
        match split_input_str_itr.next() {
            Some(l) => {
                let line = String::from(l);
                let mut line_itr = line.split(' ').into_iter();
                match line_itr.next() {
                    Some(maybe_dollar_tok) => {
                        if let None = maybe_dollar_tok.chars().nth(0) {
                            break;
                        }
                        if maybe_dollar_tok.chars().nth(0).expect("") == '$' {
                            current_cmd = Cmd::from(line_itr.next().expect(""));
                            match current_cmd {
                                Cmd::Ls => {}
                                Cmd::Cd => {
                                    let cd_arg = line_itr.next().expect("");
                                    let next_dir;
                                    match cd_arg {
                                        "/" => {
                                            next_dir = "/".to_string();
                                        }
                                        ".." => {
                                            break;
                                        }
                                        _ => {
                                            next_dir =
                                                current_dir.clone() + &cd_arg + &"/".to_string();
                                        }
                                    }
                                    current_dir_total_size += solution2_rec(
                                        split_input_str_itr,
                                        next_dir,
                                        most_optimal_dir_size,
                                        target_size,
                                    );
                                }
                                Cmd::Null => {
                                    println!("weird command? {}", line)
                                }
                            }
                        } else {
                            match current_cmd {
                                Cmd::Ls => {
                                    let mut line_itr_ls = line.split(' ').into_iter();
                                    let tok = line_itr_ls.next().expect("");
                                    match tok.parse::<usize>() {
                                        Ok(file_size) => current_dir_total_size += file_size,
                                        _ => {}
                                    }
                                }
                                _ => {
                                    println!("weird command state? {}", line)
                                }
                            }
                        }
                    }
                    None => {}
                }
            }
            None => {
                break;
            }
        }
    }

    if target_size != 0
        && current_dir_total_size >= target_size
        && current_dir_total_size < *most_optimal_dir_size
    {
        *most_optimal_dir_size = current_dir_total_size;
    }

    current_dir_total_size
}

fn solution2(s: &String) -> usize {
    let mut most_optimal_dir_size: usize = 0;
    let mut split_input_str_itr = s.split('\n').into_iter();

    let target_delete_size: usize = solution2_rec(
        &mut split_input_str_itr,
        "".to_string(),
        &mut most_optimal_dir_size,
        0,
    ) + 30_000_000
        - 70_000_000;

    split_input_str_itr = s.split('\n').into_iter();
    most_optimal_dir_size = usize::MAX;
    solution2_rec(
        &mut split_input_str_itr,
        "".to_string(),
        &mut most_optimal_dir_size,
        target_delete_size,
    );

    most_optimal_dir_size
}
