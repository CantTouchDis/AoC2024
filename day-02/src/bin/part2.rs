use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let text = fs::read_to_string("day-02/assets/input.txt")?;
    let mut safe = 0;
    for l in text.lines() {
        let numbers = l.split_whitespace().map(|n| n.parse::<i32>()).collect::<Result<Vec<_>, _>>()?;
        for i in 0..(numbers.len()) {
            let numbers = numbers.iter().enumerate().filter(|&(a, _b)| a != i).map(|(_a, b)| b).cloned().collect::<Vec<_>>();
            let differences = numbers.iter().zip(numbers.iter().skip(1)).map(|(a, b)| a - b).collect::<Vec<_>>();
            if differences.iter().all(|&v| v <= -1 && v >= -3) || differences.iter().all(|&v| v >= 1 && v <= 3) {
                safe += 1;
                break;
            }
        }
    }
    println!("Solution 2: {}", safe);

    Ok(())
}
