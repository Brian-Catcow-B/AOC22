use std::ops::Add;

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl TryFrom<char> for Direction {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'l' | 'L' => Ok(Direction::Left),
            'r' | 'R' => Ok(Direction::Right),
            'u' | 'U' => Ok(Direction::Up),
            'd' | 'D' => Ok(Direction::Down),
            _ => Err("Invalid Direction char"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Pos2d {
    pub y: isize,
    pub x: isize,
}

impl Pos2d {
    fn new(y: isize, x: isize) -> Self {
        Self { y, x }
    }
}

impl From<Direction> for Pos2d {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Left => Self { y: 0, x: -1 },
            Direction::Right => Self { y: 0, x: 1 },
            Direction::Up => Self { y: -1, x: 0 },
            Direction::Down => Self { y: 1, x: 0 },
        }
    }
}

impl Add for Pos2d {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            y: self.y + other.y,
            x: self.x + other.x,
        }
    }
}

fn main() {
    let input_str = String::from(include_str!("input1.txt"));

    let sum1 = solution1(&input_str);

    println!("solution 1: {}", sum1);
}

fn solution1(s: &String) -> usize {
    let mut exp_matrix: Vec<Vec<usize>> = vec![vec![1]];
    let mut total_positions_visited: usize = 0;
    let mut head_pos = Pos2d::new(0, 0);
    let mut tail_pos = Pos2d::new(0, 0);

    for line in s.split('\n') {
        if line.len() == 0 {
            continue;
        }
        let str_line = String::from(line);
        let mut line_itr = str_line.split(' ');
        let movement: Pos2d = Pos2d::from(
            Direction::try_from(line_itr.next().expect("").chars().nth(0).expect("")).expect(""),
        );
        let num_movements: usize = line_itr
            .next()
            .expect("no second part to line_itr")
            .parse()
            .expect("bad number");
        for _ in 0..num_movements {
            head_pos = head_pos + movement;
            // handle out of bounds case (only head can go out of bounds logically)
            if head_pos.y == -1 {
                let x_len = exp_matrix[0].len();
                exp_matrix.insert(0, vec![]);
                head_pos.y = 0;
                tail_pos.y += 1;
                for _ in 0..x_len {
                    exp_matrix[0].push(0);
                }
            } else if head_pos.y == exp_matrix.len() as isize {
                let x_len = exp_matrix[0].len();
                let y_len_before = exp_matrix.len();
                exp_matrix.push(vec![]);
                for _ in 0..x_len {
                    exp_matrix[y_len_before].push(0);
                }
            } else if head_pos.x == -1 {
                for v in exp_matrix.iter_mut() {
                    v.insert(0, 0);
                }
                head_pos.x = 0;
                tail_pos.x += 1;
            } else if head_pos.x == exp_matrix[0].len() as isize {
                for v in exp_matrix.iter_mut() {
                    v.push(0);
                }
            }
            let y_eq: bool = head_pos.y == tail_pos.y;
            let x_eq: bool = head_pos.x == tail_pos.x;
            if (head_pos.y - tail_pos.y).abs() < 2 && (head_pos.x - tail_pos.x).abs() < 2 {
                continue;
            }
            if y_eq && x_eq {
                continue;
            } else if y_eq {
                if tail_pos.x < head_pos.x {
                    tail_pos.x += 1;
                } else {
                    tail_pos.x -= 1;
                }
            } else if x_eq {
                if tail_pos.y < head_pos.y {
                    tail_pos.y += 1;
                } else {
                    tail_pos.y -= 1;
                }
            } else {
                if tail_pos.x < head_pos.x {
                    tail_pos.x += 1;
                } else {
                    tail_pos.x -= 1;
                }
                if tail_pos.y < head_pos.y {
                    tail_pos.y += 1;
                } else {
                    tail_pos.y -= 1;
                }
            }
            // set tail position in matrix to 1
            exp_matrix[tail_pos.y as usize][tail_pos.x as usize] = 1;
        }
    }

    for y in 0..exp_matrix.len() {
        for x in 0..exp_matrix[0].len() {
            total_positions_visited += exp_matrix[y][x];
        }
    }
    total_positions_visited
}
