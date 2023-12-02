use std::cmp::max;

use super::Day;

pub struct Day02;

impl Day for Day02 {
    fn file_name(&self) -> String {
        String::from("day_02.txt")
    }

    fn solution_1(&self, lines: &Vec<String>) -> usize {
        /*
        for each line/game- for each set, count the occurrences of blue, red, green
        and check to see if it exceeds 12, 13, 14
        */
        // (red, green, blue)
        let max_cubes = (12, 13, 14);
        let mut total_possible_games: usize = 0;
        for line in lines {
            let line_split = line.split(":").collect::<Vec<_>>();
            let game_id: usize = line_split[0].split(" ").collect::<Vec<_>>()[1]
                .parse::<usize>()
                .unwrap();
            let game_sets = line_split[1]
                .trim()
                .split(";")
                .map(|cubes| cubes.trim())
                .collect::<Vec<_>>();

            let mut flag = true;
            for game_set in game_sets {
                let mut game_cube_count = (0, 0, 0);
                flag = true;
                let cubes = game_set
                    .split(",")
                    .map(|cube_color| {
                        let color_counts = cube_color.trim().split(" ").collect::<Vec<_>>();
                        (color_counts[0].parse::<i32>().unwrap(), color_counts[1])
                    })
                    .collect::<Vec<_>>();

                for cube in cubes {
                    match cube {
                        (red_count, "red") => game_cube_count.0 += red_count,
                        (green_count, "green") => game_cube_count.1 += green_count,
                        (blue_count, "blue") => game_cube_count.2 += blue_count,
                        _ => {}
                    }
                }
                if game_cube_count.0 > max_cubes.0
                    || game_cube_count.1 > max_cubes.1
                    || game_cube_count.2 > max_cubes.2
                {
                    flag = false;
                    break;
                }
            }
            if flag {
                total_possible_games += game_id;
            }
        }
        total_possible_games
    }

    fn solution_2(&self, lines: &Vec<String>) -> usize {
        // (red, green, blue)
        let mut total = 0;
        for line in lines {
            let line_split = line.split(":").collect::<Vec<_>>();
            let game_sets = line_split[1]
                .trim()
                .split(";")
                .map(|cubes| cubes.trim())
                .collect::<Vec<_>>();

            let mut game_cube_max = (0, 0, 0);
            for game_set in game_sets {
                let cubes = game_set
                    .split(",")
                    .map(|cube_color| {
                        let color_counts = cube_color.trim().split(" ").collect::<Vec<_>>();
                        (color_counts[0].parse::<i32>().unwrap(), color_counts[1])
                    })
                    .collect::<Vec<_>>();

                for cube in cubes {
                    match cube {
                        (red_count, "red") => game_cube_max.0 = max(red_count, game_cube_max.0),
                        (green_count, "green") => {
                            game_cube_max.1 = max(green_count, game_cube_max.1)
                        }
                        (blue_count, "blue") => game_cube_max.2 = max(blue_count, game_cube_max.2),
                        _ => {}
                    }
                }
            }
            total += game_cube_max.0 * game_cube_max.1 * game_cube_max.2;
        }
        total.try_into().unwrap()
    }
}
