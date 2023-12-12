use aoc_runner_derive::{aoc, aoc_generator};

const MAX_RED_CUBES: u32 = 12;
const MAX_GREEN_CUBES: u32 = 13;
const MAX_BLUE_CUBES: u32 = 14;

fn calculate_color(v: Vec<char>, color: char) -> bool {
    let color_count = v
        .iter()
        .fold(0, |acc, ch| acc * 10 + ch.to_digit(10).unwrap());

    match color {
        'r' => MAX_RED_CUBES >= color_count,
        'g' => MAX_GREEN_CUBES >= color_count,
        'b' => MAX_BLUE_CUBES >= color_count,
        _ => false,
    }
}

#[aoc_generator(day2, part1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| {
            let parts = line.split(": ").collect::<Vec<&str>>();
            let (game, buckets) = (parts[0], parts[1]);

            let mut game_id: u32 = 0;
            let mut count = 0;
            for ch in game.chars().rev() {
                if ch.is_ascii_digit() {
                    game_id += ch.to_digit(10).unwrap() * 10u32.pow(count);
                    count += 1;
                } else {
                    break;
                }
            }

            let mut game_is_possible = true;
            let buckets = buckets.split("; ").collect::<Vec<&str>>();
            buckets
                .iter()
                .map(|bucket| bucket.split(", ").collect::<Vec<&str>>())
                .flat_map(|colors| {
                    println!("_______________________");
                    println!("{:?}", colors);
                    colors.into_iter()
                })
                .for_each(|color| {
                    if !game_is_possible {
                        return;
                    }

                    let mut color_c = vec![];
                    for ch in color.chars() {
                        match ch {
                            'r' | 'g' | 'b' => {
                                game_is_possible = calculate_color(color_c, ch);
                                break;
                            }
                            _ => {
                                if ch.is_ascii_digit() {
                                    color_c.push(ch);
                                }
                            }
                        }
                    }
                });

            if game_is_possible {
                game_id
            } else {
                0
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    input.iter().sum()
}
