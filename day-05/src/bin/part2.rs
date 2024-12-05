use std::{fs, collections::HashMap};

fn push_back(hash_map: &mut HashMap<i32, Vec<i32>>, key: i32, value: i32) {
    hash_map.entry(key).and_modify(|v| v.push(value)).or_insert(vec![value]);
}

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let text = fs::read_to_string("day-05/assets/test.txt")?;
    // parse into grid
    let mut after: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut before: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut lines_it = text.lines();

    lines_it
        .by_ref()
        .take_while(|l| !l.is_empty())
        .for_each(|l| {
            let (a, b) = l.split_once('|').unwrap();
            let an = a.parse::<i32>().unwrap();
            let bn = b.parse::<i32>().unwrap();
            push_back(&mut before, an, bn);
            push_back(&mut after, bn, an);
        });

    let mut result = 0;
    //check if valid
    for l in lines_it {
        let order = l.split(',').map(|v| v.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let mut ok = true;
        for i in 0..order.len() {
            let after = after.get(&order[i]);
            //println!("{:?}", after);
            if let Some(b) = after {
                for j in (i+1)..order.len() {
                    if b.contains(&order[j]) {
                        ok = false;
                        break;
                    }
                    ok = ok && !b.contains(&order[j]);
                }
            }
            if !ok {
                break;
            }
        }
        if !ok {
            use std::cmp::Ordering;
            let mut order = order.clone();
            order.sort_by(|a, b| {
                let after = after.get(&a);
                let before = before.get(&a);
                match (after, before) {
                    (Some(after), Some(before)) => {
                        if before.contains(b) {
                            Ordering::Less
                        } else if after.contains(b) {
                            Ordering::Greater
                        } else {
                            panic!()
                        }
                    },
                    (Some(after), None) => {
                        if after.contains(b) {
                            Ordering::Greater
                        } else {
                            panic!()
                        }
                    },
                    (None, Some(before)) => {
                        if before.contains(b) {
                            Ordering::Less
                        } else {
                            panic!()
                        }
                    },
                    (None, None) => {
                        panic!()
                    }
                }
            });
            result += order[order.len() / 2];
        }
    };

    println!("Solution 1: {}", result);

    Ok(())
}
