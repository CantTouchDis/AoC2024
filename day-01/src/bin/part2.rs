use std::fs;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>>
{
    println!("{:?}", std::env::current_dir());
    let text = fs::read_to_string("day-01/assets/input.txt")?;
    let mut a = vec![];
    let mut b = vec![];
    for l in text.lines() {
        let mut it = l.split_whitespace().map(|n| n.parse::<i32>());
        a.push(it.next().expect("should have value!")?);
        b.push(it.next().expect("should have value!")?);
    }
    a.sort();
    b.sort();
    println!("Solution 2: {}", a.iter().map(|a| b.iter().skip_while(|b| *b < a).take_while(|b| *b == a).count() * (*a as usize)).sum::<usize>());

    Ok(())
}
