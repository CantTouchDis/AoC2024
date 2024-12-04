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
    let text = fs::read_to_string("day-04/assets/input.txt")?;
    // parse into grid
    let mut grid = text.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut xmas_count = 0;
    let count_xmas = |grid : &[Vec<char>]| {
        let mut xmas_count = 0;
        for (row, l) in grid.iter().enumerate() {
            let mut out = vec![];
            let mut idx = 0;
            while idx < l.len() {
                out.push(l[idx]);
                if l[idx] == 'X' && idx < l.len() - 3 && l[idx + 1] == 'M' && l[idx + 2] == 'A' && l[idx + 3] == 'S' {
                    //println!("{}: {}/{}", 1, idx, row);
                    xmas_count += 1;
                }
                if l[idx] == 'X' && row < grid.len() - 3 && idx < l.len() - 3 && grid[row+1][idx + 1] == 'M' && grid[row+2][idx + 2] == 'A' && grid[row+3][idx + 3] == 'S' {
                    //println!("{}: {}/{}", 2, idx, row);
                    xmas_count += 1;
                }
                if l[idx] == 'X' && row < grid.len() - 3 && idx >= 3 && grid[row+1][idx - 1] == 'M' && grid[row+2][idx - 2] == 'A' && grid[row+3][idx - 3] == 'S' {
                    //println!("{}: {}/{}", 3, idx, row);
                    xmas_count += 1;
                }
                if l[idx] == 'X' && row < grid.len() - 3 && grid[row+1][idx] == 'M' && grid[row+2][idx] == 'A' && grid[row+3][idx] == 'S' {
                    //println!("{}: {}/{}", 4, idx, row);
                    xmas_count += 1;
                }
                if l[idx] == 'S' &&  idx < l.len() - 3 && l[idx + 1] == 'A' && l[idx + 2] == 'M' && l[idx + 3] == 'X' {
                    //println!("{}: {}/{}", 5, idx, row);
                    xmas_count += 1;
                }
                if l[idx] == 'S' && row < grid.len() - 3 && idx < l.len() - 3 && grid[row+1][idx + 1] == 'A' && grid[row+2][idx + 2] == 'M' && grid[row+3][idx + 3] == 'X' {
                    //println!("{}: {}/{}", 6, idx, row);
                    xmas_count += 1;
                }
                if l[idx] == 'S' && row < grid.len() - 3 && idx >= 3 && grid[row+1][idx - 1] == 'A' && grid[row+2][idx - 2] == 'M' && grid[row+3][idx - 3] == 'X' {
                    //println!("{}: {}/{}", 7, idx, row);
                    xmas_count += 1;
                }
                if l[idx] == 'S' && row < grid.len() - 3 && grid[row+1][idx] == 'A' && grid[row+2][idx] == 'M' && grid[row+3][idx] == 'X' {
                    //println!("{}: {}/{}", 8, idx, row);
                    xmas_count += 1;
                } else {
                    *out.last_mut().unwrap() = '.';
                }
                idx += 1;
            }
            println!("{}", out.iter().collect::<String>());
        }
        xmas_count
    };

    xmas_count += count_xmas(&grid);

    println!("Solution 1: {}", xmas_count);

    Ok(())
}
