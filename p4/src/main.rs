fn main() {
    let input_str = String::from(include_str!("input1.txt"));

    let sum1 = solution1(&input_str);

    println!("solution 1: {}", sum1);

    let sum2 = solution2(&input_str);

    println!("solution 2: {}", sum2);
}

fn solution1(s: &String) -> usize {
    let mut num_contained_lists: usize = 0;
    for line in s.split('\n') {
        if line.len() == 0 {
            break;
        }
        let mut low: [usize; 2] = [0, 0];
        let mut high: [usize; 2] = [0, 0];
        let mut i = 0;
        for half in line.split(',') {
            let mut j = 0;
            for low_high in half.split('-') {
                if j == 0 {
                    low[i] = low_high.parse().expect("");
                } else {
                    high[i] = low_high.parse().expect("");
                }
                j += 1;
            }
            i += 1;
        }
        'outer: for a in 0..2 {
            for b in 0..2 {
                if a != b {
                    if low[a] <= low[b] && high[a] >= high[b] {
                        num_contained_lists += 1;
                        break 'outer;
                    }
                }
            }
        }
    }
    num_contained_lists
}

fn solution2(s: &String) -> usize {
    let mut num_overlapping: usize = 0;
    for line in s.split('\n') {
        if line.len() == 0 {
            break;
        }
        let mut low: [usize; 2] = [0, 0];
        let mut high: [usize; 2] = [0, 0];
        let mut i = 0;
        for half in line.split(',') {
            let mut j = 0;
            for low_high in half.split('-') {
                if j == 0 {
                    low[i] = low_high.parse().expect("");
                } else {
                    high[i] = low_high.parse().expect("");
                }
                j += 1;
            }
            i += 1;
        }
        'outer: for a in 0..2 {
            for b in 0..2 {
                if a != b {
                    if (low[a] <= low[b] && low[b] <= high[a])
                        || (low[a] <= high[b] && high[b] <= high[a])
                    {
                        num_overlapping += 1;
                        break 'outer;
                    }
                }
            }
        }
    }
    num_overlapping
}
