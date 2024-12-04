use std::fs;

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let text = fs::read_to_string("day-04/assets/test.txt")?;
    // parse into grid
    let mut grid = text.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut xmas_count = 0;
    let count_xmas = |grid : &[Vec<char>]| {
        let mut xmas_count = 0;
        for (row, l) in grid.iter().enumerate() {
            let mut idx = 1;
            while idx < l.len() {
                if idx > 0 && idx < l.len() - 1 && row > 0 && row < grid.len() - 1 && l[idx] == 'A' {
                    // check for X
                    let first_diag = grid[row-1][idx-1] == 'M' && grid[row+1][idx+1] == 'S' || grid[row-1][idx-1] == 'S' && grid[row+1][idx+1] == 'M';
                    let second_diag = grid[row-1][idx+1] == 'M' && grid[row+1][idx-1] == 'S' || grid[row-1][idx+1] == 'S' && grid[row+1][idx-1] == 'M';
                    if first_diag && second_diag {
                        xmas_count += 1;
                    }
                }
                idx += 1;
            }
        }
        xmas_count
    };

    xmas_count += count_xmas(&grid);

    println!("Solution 2: {}", xmas_count);

    Ok(())
}
