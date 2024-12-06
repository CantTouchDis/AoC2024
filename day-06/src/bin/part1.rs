use std::{fs, collections::HashMap, collections::HashSet};

enum Direction {
    Up,
    Right,
    Down,
    Left
}

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let text = fs::read_to_string("day-06/assets/input.txt")?;
    // parse into grid
    let lines_it = text.lines();

    let grid_width = text.lines().next().unwrap().len();
    let grid_height = text.lines().count();

    let data = lines_it.enumerate().map(|(row,l)| {
        l.chars().enumerate().filter(|(_col, c)| *c != '.').map(move |(col, c)| (row, col, c))
    }).flatten().collect::<Vec<_>>();

    // search for the starting pos
    let mut guard_pos = data.iter().find(|(_, _, c)| *c != '#').map(|(a, b, _)| (*a, *b)).unwrap();

    // start fill
    let mut visited : HashSet<(usize, usize)> = HashSet::new();
    visited.insert(guard_pos);
    
    let mut dir = Direction::Up;
    loop {
        match dir {
            Direction::Up => {
                // the last one should be the largest one
                let blocker = data.iter().filter(|(a, b, c)| *c == '#' && guard_pos.1 == *b && *a < guard_pos.0).last().cloned().unwrap_or((usize::MAX, guard_pos.1, '#'));
                println!("B: {:?}", blocker);
                (blocker.0.wrapping_add(1)..guard_pos.0).map(|p| (p, guard_pos.1)).for_each(|v| { visited.insert(v); });
                guard_pos = (blocker.0.wrapping_add(1), guard_pos.1);
                if blocker.0 == usize::MAX {
                    break;
                }
                dir = Direction::Right;
            }
            Direction::Right => {
                // the last one should be the largest one
                let blocker = data.iter().filter(|(a, b, c)| *c == '#' && guard_pos.1 < *b && *a == guard_pos.0).next().cloned().unwrap_or((guard_pos.0, grid_width, '#'));
                println!("B: {:?}", blocker);
                println!("R: {:?}", (guard_pos.1..blocker.0.saturating_sub(1)));
                (guard_pos.1..blocker.1).map(|p| (guard_pos.0, p)).for_each(|v| { visited.insert(v); });
                guard_pos = (guard_pos.0, blocker.1.saturating_sub(1));
                if blocker.1 == grid_width {
                    break;
                }
                dir = Direction::Down;
            }
            Direction::Down => {
                // the last one should be the largest one
                let blocker = data.iter().filter(|(a, b, c)| *c == '#' && guard_pos.1 == *b && *a > guard_pos.0).next().cloned().unwrap_or((grid_height, guard_pos.1, '#'));
                println!("B: {:?}", blocker);
                (guard_pos.0..blocker.0).map(|p| (p, guard_pos.1)).for_each(|v| { visited.insert(v); });
                guard_pos = (blocker.0.saturating_sub(1), guard_pos.1);
                if blocker.0 == grid_height {
                    break;
                }
                dir = Direction::Left;
            }
            Direction::Left => {
                // the last one should be the largest one
                let blocker = data.iter().filter(|(a, b, c)| *c == '#' && guard_pos.1 > *b && *a == guard_pos.0).last().cloned().unwrap_or((guard_pos.0, usize::MAX, '#'));
                println!("B: {:?}", blocker);
                (blocker.1.wrapping_add(1)..guard_pos.1).map(|p| (guard_pos.0, p)).for_each(|v| { visited.insert(v); });
                guard_pos = (guard_pos.0, blocker.1.wrapping_add(1));
                if blocker.1 == grid_height {
                    break;
                }
                dir = Direction::Up;
            }
        }
    }
    for row in 0..10 {
        for col in 0..10 {
            if visited.contains(&(row, col)) {
                print!("X")
            } else {
                print!(".")
            }
        }
        println!();
    }

    println!("{:?}", visited);
    println!("Solution 1: {:?}", visited.len());

    Ok(())
}
