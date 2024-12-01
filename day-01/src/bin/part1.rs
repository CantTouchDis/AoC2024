use std::fs;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>>
{
    println!("{:?}", std::env::current_dir());
    let text = fs::read_to_string("day-01/assets/test.txt")?;
    let mut a = vec![];
    let mut b = vec![];
    for l in text.lines() {
        let mut it = l.split_whitespace().map(|n| n.parse::<i32>());
        a.push(it.next().expect("should have value!")?);
        b.push(it.next().expect("should have value!")?);
    }
    a.sort();
    b.sort();
    println!("Solution 1: {}", a.iter().zip(b.iter()).map(|(a, b)| (a - b).abs()).sum::<i32>());

    Ok(())
}
