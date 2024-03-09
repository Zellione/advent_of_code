use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

#[derive(Copy, Clone)]
struct Game {
    min_red: u32,
    min_green: u32,
    min_blue: u32,
    is_valid: bool
}

fn check_red(to_test: u32, game: &mut  Game) -> &Game {
    let num_of_red: &'static u32 = &12;

    game.is_valid = game.is_valid && !(to_test > *num_of_red);
    if to_test > game.min_red {
        game.min_red = to_test;
    }

    game
}

fn check_green(to_test: u32, game: &mut Game) -> &Game {
    let num_of_green: &'static u32 = &13;

    game.is_valid = game.is_valid && !(to_test > *num_of_green);
    if to_test > game.min_green {
        game.min_green = to_test;
    }

    game
}

fn check_blue(to_test: u32, game: &mut Game) -> &Game {
    let num_of_blue: &'static u32 = &14;

    game.is_valid = game.is_valid && !(to_test > *num_of_blue);
    if to_test > game.min_blue {
        game.min_blue = to_test;
    }

    game
}

fn is_game_valid(game: &str) -> Game {

    let offset = game.find(':').unwrap();
    let mut game = String::from(game);
    let _: Vec<_> = game.drain(..(offset + 2)).collect();
    let mut game_result = Game {
        min_red : 0,
        min_green : 0,
        min_blue : 0,
        is_valid : true
    };

    let subsets_of_cubes: Vec<&str> = game.split(';').collect();
    for subset in subsets_of_cubes {
        let cubes: Vec<&str> = subset.split(',').collect();
        for cube in cubes {
            let cube = cube.trim();
            let cube_num: Vec<&str> = cube.split(' ').collect();

            let num = cube_num[0].parse::<u32>().unwrap();
            match cube_num[1] {
                "red" => game_result = *check_red(num, &mut game_result),
                "green" => game_result = *check_green(num, &mut game_result),
                "blue" => game_result = *check_blue(num, &mut game_result),
                _ => println!("what happened?")
            }
        }
    }

    game_result
}

fn retrieve_game_number(line: &str) -> u32 {
    let mut line = String::from(line);
    let offset = line.find(':').unwrap_or(line.len());
    let drain = line.drain(..offset);

    drain.as_str().get(5..).unwrap().parse::<u32>().unwrap()
}

fn read_file_line_by_line(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut sum: u32 = 0;
    let mut power: u32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let game_number = retrieve_game_number(&line);

        let game_result = is_game_valid(&line);
        if game_result.is_valid {
            sum += game_number;
        }

        power += game_result.min_red * game_result.min_green * game_result.min_blue;
    }

    println!("Sum of games that are valid: {}", sum);
    println!("Power of minimum viable cubes: {}", power);

    Ok(())
}

fn main() {
    let _ = read_file_line_by_line("src/day2_input");
}
