use std:: fs::read_to_string;

fn parse_bot(val: &str) -> (i32, i32, i32, i32) {
    let (pos, velo) = val.split_once(" v=").unwrap();
    let pos = pos[2..].split_once(",").unwrap();
    let velo = velo.split_once(",").unwrap();
    (pos.0.parse::<i32>().unwrap(), pos.1.parse::<i32>().unwrap(),
    velo.0.parse::<i32>().unwrap(), velo.1.parse::<i32>().unwrap())
}

fn move_bot_on_board(bot: (i32, i32, i32, i32), width: i32, height: i32, seconds: i32) -> (i32, i32) {

    let x = (bot.0 + seconds * bot.2).rem_euclid(width);
    let y = (bot.1 + seconds * bot.3).rem_euclid(height);
    (x, y)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = read_to_string("day-14/assets/input.txt")?;

    // parse the bots

    let bots = text.lines().map(parse_bot);

    const SECONDS : i32 = 100;
    const BOARD_WIDTH : i32 = 101;
    const BOARD_HEIGHT : i32 = 103;

    const QUADRANT_X_SEP: i32 = BOARD_WIDTH / 2;
    const QUADRANT_Y_SEP: i32 = BOARD_HEIGHT / 2;

    let mut quadrants : [usize; 4] = [0; 4];

    for bot in bots {
        let pos = move_bot_on_board(bot, BOARD_WIDTH, BOARD_HEIGHT, SECONDS);
        //println!("Bot at {pos:?}");
        if pos.0 < QUADRANT_X_SEP {
            if pos.1 < QUADRANT_Y_SEP {
                quadrants[0] += 1;
            }
            else if pos.1 > QUADRANT_Y_SEP {
                quadrants[1] += 1;
            }
        } else if pos.0 > QUADRANT_X_SEP {
            if pos.1 < QUADRANT_Y_SEP {
                quadrants[2] += 1;
            }
            else if pos.1 > QUADRANT_Y_SEP {
                quadrants[3] += 1;
            }
        }
    }

    println!("Solution 1: {:?}", quadrants.iter().product::<usize>());

    Ok(())
}
