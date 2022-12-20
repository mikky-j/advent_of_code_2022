use std::collections::HashSet;

pub fn solution(input: &str) {
    let input_matrix: Vec<&[u8]> = input
        .split_terminator('\n')
        .map(|line| line.as_bytes())
        .collect();
    let total_length: usize = 9801;
    let mut final_set = HashSet::<usize>::new();
    let (mut left_max, mut right_max, mut top_max, mut bottom_max) = (0, 0, 0, 0);

    for index in 0..total_length {
        // Reset it after every line
        if index % 99 == 0 {
            (left_max, right_max, top_max, bottom_max) = (0, 0, 0, 0);
        }
        // Calculate all the respective direction index from this one index
        let left_index = (index / 99, index % 99);
        let right_index = (left_index.0, (98 - left_index.1));
        let top_index = (left_index.1, left_index.0);
        let bottom_index = (98 - top_index.0, top_index.1);

        // Get all the repsective values
        let left_value = input_matrix[left_index.0][left_index.1];
        let right_value = input_matrix[right_index.0][right_index.1];
        let bottom_value = input_matrix[bottom_index.0][bottom_index.1];
        let top_value = input_matrix[top_index.0][top_index.1];

        if left_value > left_max {
            final_set.insert((left_index.0 * 99) + left_index.1);
            left_max = left_value;
        }

        if right_value > right_max {
            final_set.insert((right_index.0 * 99) + right_index.1);
            right_max = right_value;
        }

        if top_value > top_max {
            final_set.insert((top_index.0 * 99) + top_index.1);
            top_max = top_value;
        }

        if bottom_value > bottom_max {
            final_set.insert((bottom_index.0 * 99) + bottom_index.1);
            bottom_max = bottom_value;
        }
    }

    println!(
        "The maximum number of visible trees are {}",
        final_set.len()
    );
    let mut scenic_scores = vec![];

    for items in final_set {
        let (row, col) = (items / 99, items % 99);
        let scenic_score = get_scenic_score_for_index(&input_matrix, (row, col));
        scenic_scores.push(scenic_score);
    }

    println!(
        "The highest scenic_scores is {}",
        scenic_scores.iter().max().unwrap()
    )
}

fn get_scenic_score_for_index(grid: &Vec<&[u8]>, (row, col): (usize, usize)) -> usize {
    let mut answer = vec![];
    let (mut left_count, mut right_count, mut top_count, mut bottom_count) = (1, 1, 1, 1);
    let (mut search_left, mut search_right, mut search_top, mut search_bottom) =
        (true, true, true, true);
    let value = grid[row][col];

    while answer.len() != 4 {
        if search_top {
            // Check if I am in bounds
            if let Some(new_row) = row.checked_sub(top_count) {
                // Check if the new value is greater than the current value
                if grid[new_row][col] >= value {
                    answer.push(top_count);
                    search_top = false;
                } else {
                    top_count += 1;
                }
            } else {
                answer.push(top_count - 1);
                search_top = false;
            }
        }

        if search_bottom {
            let new_row = bottom_count + row;
            // Check if I am still within the bounds
            if new_row < 99 {
                // Check if the new value is greater thatn the current value
                if grid[new_row][col] >= value {
                    answer.push(bottom_count);
                    search_bottom = false;
                } else {
                    bottom_count += 1;
                }
            }
            // I am at the end so stop searching and just push the count to the answer array
            else {
                answer.push(bottom_count - 1);
                search_bottom = false;
            }
        }

        if search_left {
            // Check if I am in bounds
            if let Some(new_col) = col.checked_sub(left_count) {
                // Check if the new value is greater than the current value
                if grid[row][new_col] >= value {
                    answer.push(left_count);
                    search_left = false;
                } else {
                    left_count += 1;
                }
            } else {
                answer.push(left_count - 1);
                search_left = false;
            }
        }

        if search_right {
            let new_col = right_count + col;
            // Check if I am still within the bounds
            if new_col < 99 {
                // Check if the new value is greater thatn the current value
                if grid[row][new_col] >= value {
                    answer.push(right_count);
                    search_right = false;
                } else {
                    right_count += 1;
                }
            }
            // I am at the end so stop searching and just push the count to the answer array
            else {
                answer.push(right_count - 1);
                search_right = false;
            }
        }
    }

    answer
        .into_iter()
        .fold(1, |acc, x: usize| if x > 0 { x * acc } else { acc })
}
