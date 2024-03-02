use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Default, Debug)]
struct Game {
    game: u32,
    win_nums: Vec<u32>,
    my_nums: Vec<u32>,
}

impl Game {
    pub fn new(line: &String) -> Self {
        let split: Vec<&str> = line.split(":").collect();
        let game_number: Vec<&str> = split
            .first()
            .expect("Game should have an number a card number")
            .split(" ")
            .collect();
        let game_number: u32 = game_number
            .last()
            .expect("Should be able to get a line number")
            .parse()
            .expect("Should be a parseable number");
        let numbers = split.last().expect("there should be the actual numbers");
        let numbers: Vec<&str> = numbers.split("|").collect();
        let win_nums: Vec<u32> = numbers
            .first()
            .expect("should have part with winning numbers")
            .split(" ")
            .filter(|x| x.trim() != "")
            .map(|x| x.parse::<u32>().expect("Cannot parse to integer"))
            .collect();

        let my_nums: Vec<u32> = numbers
            .last()
            .expect("should have part with winning numbers")
            .split(" ")
            .filter(|x| x.trim() != "")
            .map(|x| x.parse::<u32>().expect("Cannot parse to integer"))
            .collect();

        Self {
            game: game_number,
            win_nums,
            my_nums,
        }
    }

    pub fn calc_score(&self) -> u32 {
        let mut score = 0;
        let mut my_nums = self.my_nums.clone();
        my_nums.retain(|x| self.win_nums.contains(x));
        my_nums.iter().for_each(|_| {
            if score == 0 {
                score = 1;
            } else {
                score *= 2;
            }
        });

        score
    }
}

fn read_file_line_by_line(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut score: u32 = 0;

    for line in reader.lines() {
        let line = line.expect("Should be a line");
        let game = Game::new(&line);
        let part_score = game.calc_score();
        score += part_score;
        println!("Game: {} Score: {}", game.game, part_score);
    }

    println!("Final score: {}", score);

    Ok(())
}

fn main() {
    println!("Welcome day 4");
    let _ = read_file_line_by_line("src/day4/day4_input");
}
