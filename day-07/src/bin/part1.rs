use std::{fs, collections::HashMap, collections::HashSet, ops::{Shl, BitAnd}};

fn test_plausible(result: i64, numbers: &[i64]) -> bool {
    let num_possibilities = 2usize.pow((numbers.len() - 1) as u32) as usize;
    for i in 0..num_possibilities {
        let mut res = numbers[0];
        for j in 0..numbers.len()-1 {
            if 1usize.shl(j).bitand(i) == 0 {
                // do +
                res += numbers[j + 1];
            } else {
                // do *
                res *= numbers[j + 1];
            }
        }
        println!("{}: {}", i, res);
        if res == result {
            return true;
        }
    }
    return false;
}

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let text = fs::read_to_string("day-07/assets/input.txt")?;
    // parse into grid
    let lines_it = text.lines();
    let mut total = 0;
    for line in lines_it {
        let (res_s, rest) = line.split_once(": ").unwrap();
        let result = res_s.parse::<i64>().unwrap();
        let numbers = rest.split_ascii_whitespace().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>();
        println!("{}: {:?}", result, numbers);
        
        if test_plausible(result, &numbers) {
            total += result;
        }
    }


    println!("Solution 1: {:?}", total);

    Ok(())
}
