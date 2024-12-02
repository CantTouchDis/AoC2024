use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    println!("{:?}", std::env::current_dir());
    let text = fs::read_to_string("day-02/assets/input.txt")?;
    let mut safe = 0;
    for l in text.lines() {
        let numbers = l.split_whitespace().map(|n| n.parse::<i32>()).collect::<Result<Vec<_>, _>>()?;
        let differences = numbers.iter().zip(numbers.iter().skip(1)).map(|(a, b)| a - b).collect::<Vec<_>>();
        if differences.iter().all(|&v| v <= -1 && v >= -3) || differences.iter().all(|&v| v >= 1 && v <= 3) {
            safe += 1;
        }
    }
    println!("Solution 1: {}", safe);

    Ok(())
}
