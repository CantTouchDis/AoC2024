use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let text = fs::read_to_string("day-03/assets/input.txt")?;
    let mut res : i64 = 0;
    let mut current_index = 0;
    while current_index < text.len() {
        // next instructions
        if let Some(idx) = text[current_index..].find("mul(") {
            let total = text[(current_index+idx+4)..].chars().take_while(|c| *c != ')').collect::<String>();
            current_index += idx;
            if let Some((a, b)) = total.split_once(|a| a == ',') {
                if let (Ok(an), Ok(bn)) = (a.parse::<i64>(), b.parse::<i64>()) {
                    // check the last do or dont
                    match (text[0..current_index].rfind("do()"), text[0..current_index].rfind("don't()")) {
                        (Some(a), Some(b)) =>
                        {
                            if a > b {
                                res += an * bn;
                            }
                        },
                        (Some(_), _) => {
                            res += an * bn;
                        },
                        (None, None) => {
                            res += an * bn;
                        },
                        _ => {},
                    }
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
    println!("Solution 2: {}", res);

    Ok(())
}
