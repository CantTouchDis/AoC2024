use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let text = fs::read_to_string("day-03/assets/input.txt")?;
    let mut res : i64 = 0;
    let mut current_index = 0;
    while current_index < text.len() {
        if let Some(idx) = text[current_index..].find("mul(") {
            let total = text[(current_index+idx+4)..].chars().take_while(|c| *c != ')').collect::<String>();
            current_index += idx;
            if let Some((a, b)) = total.split_once(|a| a == ',') {
                if let (Ok(an), Ok(bn)) = (a.parse::<i64>(), b.parse::<i64>()) {
                    res += an * bn;
                    current_index += total.len();
                }
                current_index += 4;
            } else {
                current_index += 4;
            }
        } else {
            break;
        }
    }
    println!("Solution 1: {}", res);

    Ok(())
}
