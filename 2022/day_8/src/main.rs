use std::fs;

fn main() {
    let input = fs::read_to_string("input_p1").expect("Unable to read file");
    let input_lines = input.lines();
    let mut visibility_counter = 0;
    let mut tree_house_score = 0;
    let mut grid: Vec<Vec<u32>> = vec![];

    for line in input_lines {
        let mut grid_row = vec![];
        for c in line.chars() {
            grid_row.push(c.to_digit(10).unwrap());
        }
        grid.push(grid_row);
    }

    for i in 1..grid.len() - 1 {
        for j in 1..grid[i].len() - 1 {
            let curr_val = grid[i][j];
            let curr_tree_house_score;
            let mut not_visible_from_edge: u32 = 0;
            let mut curr_tree_house_score_tpl = (0, 0, 0, 0);
            let mut offset = 1;

            while (i as i32) - (offset as i32) >= 0
                || i + offset < grid[i].len()
                || (j as i32) - (offset as i32) >= 0
                || j + offset < grid.len()
            {
                // Part 1
                if (i as i32) - (offset as i32) >= 0
                    && grid[i - offset][j] >= curr_val
                    && curr_tree_house_score_tpl.3 == 0
                {
                    not_visible_from_edge |= 1 << 3;
                    curr_tree_house_score_tpl.3 = offset;
                }
                if i + offset < grid.len()
                    && grid[i + offset][j] >= curr_val
                    && curr_tree_house_score_tpl.2 == 0
                {
                    not_visible_from_edge |= 1 << 2;
                    curr_tree_house_score_tpl.2 = offset;
                }
                if (j as i32) - (offset as i32) >= 0
                    && grid[i][j - offset] >= curr_val
                    && curr_tree_house_score_tpl.1 == 0
                {
                    not_visible_from_edge |= 1 << 1;
                    curr_tree_house_score_tpl.1 = offset;
                }
                if j + offset < grid[i].len()
                    && grid[i][j + offset] >= curr_val
                    && curr_tree_house_score_tpl.0 == 0
                {
                    not_visible_from_edge |= 1 << 0;
                    curr_tree_house_score_tpl.0 = offset;
                }

                // Part 2
                if (i as i32) - (offset as i32) == 0 && curr_tree_house_score_tpl.3 == 0 {
                    curr_tree_house_score_tpl.3 = offset;
                }
                if i + offset == grid.len() - 1 && curr_tree_house_score_tpl.2 == 0 {
                    curr_tree_house_score_tpl.2 = offset;
                }
                if (j as i32) - (offset as i32) == 0 && curr_tree_house_score_tpl.1 == 0 {
                    curr_tree_house_score_tpl.1 = offset;
                }
                if j + offset == grid[i].len() - 1 && curr_tree_house_score_tpl.0 == 0 {
                    curr_tree_house_score_tpl.0 = offset;
                }

                offset += 1;
            }

            if not_visible_from_edge < 0b1111 {
                visibility_counter += 1;
            }

            curr_tree_house_score = curr_tree_house_score_tpl.0
                * curr_tree_house_score_tpl.1
                * curr_tree_house_score_tpl.2
                * curr_tree_house_score_tpl.3;

            if tree_house_score < curr_tree_house_score {
                tree_house_score = curr_tree_house_score;
            }
        }
    }

    // Part 1
    println!(
        "Part 1: {:?}",
        visibility_counter + (grid.len() + grid[0].len()) * 2 - 4
    );

    // Part 2
    println!("Part 2: {:?}", tree_house_score);
}
