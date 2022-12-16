fn main() {
    let input_str = String::from(include_str!("input1.txt"));

    let sum1 = solution1(&input_str);

    println!("solution 1: {}", sum1);

    let best2 = solution2(&input_str);

    println!("solution 2: {}", best2);
}

// returns vec, numrows, numcols
fn parse_into_vec(input: &String) -> (Vec<Vec<u8>>, usize, usize) {
    // access by [y][x]
    let mut trees: Vec<Vec<u8>> = Vec::new();
    let mut capacity: usize = usize::MAX;
    for line in input.split('\n') {
        if line.len() == 0 {
            continue;
        }
        if capacity == usize::MAX {
            capacity = line.len()
        } else if capacity != line.len() {
            panic!("capacity: {} but line.len(): {}", capacity, line.len());
        }
        let mut row_of_trees: Vec<u8> = Vec::with_capacity(capacity);
        for c in line.chars() {
            row_of_trees.push(c as u8 - '0' as u8);
            if c as u8 - '0' as u8 > 9 {
                panic!("attempted to insert {} to matrix", c);
            }
        }
        trees.push(row_of_trees);
    }
    let len = trees.len();
    (trees, len, capacity)
}

fn solution1(s: &String) -> usize {
    let (forest_matrix, numrows, numcols) = parse_into_vec(s);
    let mut visible_count: usize = 0;

    for forest_y in 0..numrows {
        'tree_check: for forest_x in 0..numcols {
            let tree_height = forest_matrix[forest_y][forest_x];
            let mut found_tall_enough_tree = false;

            // left
            for check_x in 0..forest_x {
                if forest_matrix[forest_y][check_x] >= tree_height {
                    found_tall_enough_tree = true;
                    break;
                }
            }
            if !found_tall_enough_tree {
                visible_count += 1;
                continue 'tree_check;
            }
            found_tall_enough_tree = false;

            // right
            for check_x in (forest_x + 1)..numcols {
                if forest_matrix[forest_y][check_x] >= tree_height {
                    found_tall_enough_tree = true;
                    break;
                }
            }
            if !found_tall_enough_tree {
                visible_count += 1;
                continue 'tree_check;
            }
            found_tall_enough_tree = false;

            // up
            for check_y in 0..forest_y {
                if forest_matrix[check_y][forest_x] >= tree_height {
                    found_tall_enough_tree = true;
                    break;
                }
            }
            if !found_tall_enough_tree {
                visible_count += 1;
                continue 'tree_check;
            }
            found_tall_enough_tree = false;

            // down
            for check_y in (forest_y + 1)..numrows {
                if forest_matrix[check_y][forest_x] >= tree_height {
                    found_tall_enough_tree = true;
                    break;
                }
            }
            if !found_tall_enough_tree {
                visible_count += 1;
            }
        }
    }

    visible_count
}

fn solution2(s: &String) -> usize {
    let (forest_matrix, numrows, numcols) = parse_into_vec(s);
    let mut best_scenic_score: usize = 0;

    for forest_y in 0..numrows {
        for forest_x in 0..numcols {
            let tree_height = forest_matrix[forest_y][forest_x];
            let mut trees_in_raycast = 0;
            let mut score = 1;

            // left
            for check_x in (0..forest_x).rev() {
                trees_in_raycast += 1;
                if forest_matrix[forest_y][check_x] >= tree_height {
                    break;
                }
            }
            let trees_left = trees_in_raycast;
            score *= trees_in_raycast;
            trees_in_raycast = 0;

            // right
            for check_x in (forest_x + 1)..numcols {
                trees_in_raycast += 1;
                if forest_matrix[forest_y][check_x] >= tree_height {
                    break;
                }
            }
            let trees_right = trees_in_raycast;
            score *= trees_in_raycast;
            trees_in_raycast = 0;

            // up
            for check_y in (0..forest_y).rev() {
                trees_in_raycast += 1;
                if forest_matrix[check_y][forest_x] >= tree_height {
                    break;
                }
            }
            let trees_up = trees_in_raycast;
            score *= trees_in_raycast;
            trees_in_raycast = 0;

            // down
            for check_y in (forest_y + 1)..numrows {
                trees_in_raycast += 1;
                if forest_matrix[check_y][forest_x] >= tree_height {
                    break;
                }
            }
            let trees_down = trees_in_raycast;
            score *= trees_in_raycast;

            // update best score maybe
            if score > best_scenic_score {
                best_scenic_score = score;
            }
        }
    }

    best_scenic_score
}
