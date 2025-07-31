
use std::fs::File;
use std::io::{self, BufRead};
//use std::path::Path;

fn read_grid_from_file(filename: &str) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let grid = reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.chars().collect())
        .collect();

    Ok(grid)
}

fn count_xmas(grid: &[Vec<char>]) -> usize {
    let target = ['X', 'M', 'A', 'S'];
    let directions = [
        (0, 1),   // right
        (0, -1),  // left
        (1, 0),   // down
        (-1, 0),  // up
        (1, 1),   // down-right
        (-1, -1), // up-left
        (-1, 1),  // up-right
        (1, -1),  // down-left
    ];

    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for row in 0..rows {
        for col in 0..cols {
            for &(dr, dc) in &directions {
                let mut found = true;
                for i in 0..target.len() {
                    let nr = row as isize + dr * i as isize;
                    let nc = col as isize + dc * i as isize;
                    if nr < 0 || nc < 0 || nr >= rows as isize || nc >= cols as isize {
                        found = false;
                        break;
                    }
                    if grid[nr as usize][nc as usize] != target[i] {
                        found = false;
                        break;
                    }
                }
                if found {
                    count += 1;
                }
            }
        }
    }

    count
}

fn main() -> io::Result<()> {
    let filename = "list.txt";
    let grid = read_grid_from_file(filename)?;

    let result = count_xmas(&grid);
    println!("Found {} XMAS matches!", result);

    Ok(())
}
