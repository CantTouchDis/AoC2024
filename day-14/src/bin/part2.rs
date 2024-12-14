use std::{ fs::read_to_string, collections::HashSet, io::Write};
use std::fmt;

fn parse_bot(val: &str) -> (i64, i64, i64, i64) {
    let (pos, velo) = val.split_once(" v=").unwrap();
    let pos = pos[2..].split_once(",").unwrap();
    let velo = velo.split_once(",").unwrap();
    (pos.0.parse::<i64>().unwrap(), pos.1.parse::<i64>().unwrap(),
    velo.0.parse::<i64>().unwrap(), velo.1.parse::<i64>().unwrap())
}

fn move_bot_on_board(bot: (i64, i64, i64, i64), width: i64, height: i64, seconds: i64) -> (i64, i64) {

    let x = (bot.0 + seconds * bot.2).rem_euclid(width);
    let y = (bot.1 + seconds * bot.3).rem_euclid(height);
    (x, y)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = read_to_string("day-14/assets/input.txt")?;

    // parse the bots

    let bots = text.lines().map(parse_bot).collect::<Vec<_>>();

    const BOARD_WIDTH : i64 = 101;
    const BOARD_HEIGHT : i64 = 103;

    const QUADRANT_X_SEP: i64 = BOARD_WIDTH / 2;
    const QUADRANT_Y_SEP: i64 = BOARD_HEIGHT / 2;


    let seconds = 8159;
    //loop
    {
        let mut quadrants : [usize; 2] = [0; 2];
        let unique_moved_bots = bots.iter().map(|b| move_bot_on_board(*b, BOARD_WIDTH, BOARD_HEIGHT, seconds)).collect::<HashSet<_>>();

        unique_moved_bots.iter().for_each(|pos| {
            if pos.0 < QUADRANT_X_SEP {
                quadrants[0] += 1;
            } else if pos.0 > QUADRANT_X_SEP {
                quadrants[1] += 1;
            }
        });

        {
            let mut out : String = String::new();
            fmt::write(&mut out, format_args!("\n")).unwrap();
            fmt::write(&mut out, format_args!("{} Seconds:\n", seconds)).unwrap();

            for y in 0..BOARD_HEIGHT {
                for x in 0..BOARD_WIDTH {
                    fmt::write(&mut out, format_args!("{}", if unique_moved_bots.contains(&(x, y)) { "x" } else { " " })).unwrap();
                }
                fmt::write(&mut out, format_args!("\n")).unwrap();
            }
            std::io::stdout().write(out.as_bytes()).unwrap();
        }
        //lock.flush().unwrap();
    }

    Ok(())
}
