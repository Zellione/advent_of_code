use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Default, Debug, Clone)]
struct Game {
    game: usize,
    win_nums: Vec<u32>,
    my_nums: Vec<u32>,
    score: u32,
    wins: usize,
    copies: usize,
}

impl Game {
    pub fn new(line: &String) -> Self {
        let split: Vec<&str> = line.split(":").collect();
        let game_number: Vec<&str> = split
            .first()
            .expect("Game should have an number a card number")
            .split(" ")
            .collect();
        let game_number: usize = game_number
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

        let mut game = Self {
            game: game_number,
            win_nums,
            my_nums,
            score: 0,
            wins: 0,
            copies: 0,
        };
        game.calc_score();

        game
    }

    fn calc_score(&mut self) {
        let mut score = 0;
        let mut num_wins = 0;
        let mut my_nums = self.my_nums.clone();
        my_nums.retain(|x| self.win_nums.contains(x));
        my_nums.iter().for_each(|_| {
            num_wins += 1;
            if score == 0 {
                score = 1;
            } else {
                score *= 2;
            }
        });

        self.score = score;
        self.wins = num_wins;
    }

    pub fn spawn_clones(games: &mut BTreeMap<usize, Game>) {
        let max_cards = games.len() + 1;
        for i in 1..max_cards {
            let game = games.get_mut(&i).expect("index to be present");
            if game.wins == 0 {
                continue;
            }
            let copies = game.copies;
            for card_to_clone in i + 1..i + 1 + game.wins {
                if card_to_clone > max_cards {
                    break;
                }
                let game_change = games.get_mut(&card_to_clone).expect("index to be present");
                game_change.copies += 1 * (copies + 1);
            }
        }
    }
}

fn read_file_line_by_line(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut score: u32 = 0;
    let mut num_cards: u32 = 0;
    let mut games: BTreeMap<usize, Game> = BTreeMap::new();

    for line in reader.lines() {
        let line = line.expect("Should be a line");
        let game = Game::new(&line);
        let card_num = game.game;
        games.insert(card_num, game);
    }
    Game::spawn_clones(&mut games);

    for (_, game) in &mut games {
        score += game.score;
        num_cards += 1 + game.copies as u32;
    }

    // println!("{:#?}", games);
    println!("Final score: {}", score);
    println!("Final num of cards: {}", num_cards);

    Ok(())
}

fn main() {
    let _ = read_file_line_by_line("src/day4_input");
}
