use std::{
    collections::HashMap,
    collections::HashSet,
    fs,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = fs::read_to_string("day-08/assets/input.txt")?;
    // parse into grid
    let lines_it = text.lines();
    let mut antennas = HashMap::new();

    let width = text.lines().next().unwrap().len() as i32;
    let height = text.lines().count() as i32;

    lines_it
        .enumerate()
        .map(|(row, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(move |(col, c)| (c, (row as i32, col as i32)))
        })
        .flatten()
        .for_each(|(c, e)| {
            antennas.entry(c).and_modify(|data: &mut Vec<_>| data.push(e))
                .or_insert(vec![e]);
        });

    let mut focus_locations = HashSet::new();

    for (_, antennas) in &antennas {
        for i in 0..antennas.len() {
            focus_locations.insert(antennas[i]);
            for j in (i+1)..antennas.len() {
                // compute the axis
                let a_i = antennas[i];
                let a_j = antennas[j];
                let col_delta = antennas[i].1 - antennas[j].1;
                let row_delta = antennas[i].0 - antennas[j].0;
                let mut pos0 = (a_i.0 + row_delta, a_i.1 + col_delta);
                let mut pos1 = (a_j.0 - row_delta, a_j.1 - col_delta);
                while pos0.0 >= 0 && pos0.0 < height && pos0.1 >= 0 && pos0.1 < width {
                    focus_locations.insert(pos0);
                    pos0 = (pos0.0 + row_delta, pos0.1 + col_delta);
                }
                while pos1.0 >= 0 && pos1.0 < height && pos1.1 >= 0 && pos1.1 < width {
                    focus_locations.insert(pos1);
                    pos1 = (pos1.0 - row_delta, pos1.1 - col_delta);
                }
            }
        }
    }
    for i in 0..height {
        for j in 0..width {
            if focus_locations.contains(&(i, j)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    println!("Solution 2: {:?}", focus_locations.len());

    Ok(())
}
