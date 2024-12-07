use std::{fs, collections::HashMap, collections::HashSet, ops::{Shl, BitAnd}};

fn test_plausible(result: i64, numbers: &[i64]) -> bool {
    let num_possibilities = 3usize.pow((numbers.len() - 1) as u32) as usize;
    let mut op = vec![0; numbers.len()];
    for i in 0..num_possibilities {
        let mut res = numbers[0];
        for j in 0..numbers.len()-1 {
            match op[j] {
                0 => {
                    // do +
                    res += numbers[j + 1];
                }
                1 => {
                    // do *
                    res *= numbers[j + 1];
                }
                2 => {
                    // concat
                    res = res * 10i64.pow(numbers[j + 1].ilog10() + 1) + numbers[j + 1];
                }
                _ => panic!()
            }
            // early continue
            if res > result
            {
                break;
            }
        }
        if res == result {
            return true;
        }
        // update ops;
        for j in 0..numbers.len() {
            op[j] = op[j] + 1;
            if op[j] != 3 {
                break;
            }
            op[j] = 0;
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
        if test_plausible(result, &numbers) {
            total += result;
        }
    }


    println!("Solution 1: {:?}", total);

    Ok(())
}
