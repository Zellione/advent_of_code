use core::fmt;
use std::{
    cmp::Ordering,
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Copy, Clone, PartialEq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
    Unknown,
}

impl fmt::Display for HandType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            HandType::Unknown => write!(f, "Unknown"),
            HandType::FiveOfAKind => write!(f, "FiveOfAKind"),
            HandType::FourOfAKind => write!(f, "FourOfAKind"),
            HandType::FullHouse => write!(f, "FullHouse"),
            HandType::ThreeOfAKind => write!(f, "ThreeOfAKind"),
            HandType::TwoPair => write!(f, "TwoPair"),
            HandType::OnePair => write!(f, "OnePair"),
            HandType::HighCard => write!(f, "HighCard"),
        }
    }
}

impl HandType {
    pub fn get_by_count(count: u8) -> Self {
        match count {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => HandType::ThreeOfAKind,
            2 => HandType::OnePair,
            1 => HandType::HighCard,
            _ => panic!("This should not happen"),
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bid: u32,
    hand_type: HandType,
}

impl Hand {
    pub fn new(line: &str) -> Self {
        let elements: Vec<&str> = line.split(" ").collect();

        let bid = elements
            .last()
            .expect("to have the bid")
            .parse::<u32>()
            .expect("to be a number");

        let mut cards: Vec<char> = Vec::new();
        for element in elements
            .first()
            .expect("to have the cards")
            .to_string()
            .chars()
        {
            cards.push(element);
        }

        let mut hand = Hand {
            cards,
            bid,
            hand_type: HandType::Unknown,
        };
        hand.determine_type();

        hand
    }

    fn determine_type(&mut self) {
        let mut cards_by_count: BTreeMap<char, u8> = BTreeMap::new();
        for card in &self.cards {
            if cards_by_count.contains_key(&card) {
                cards_by_count.entry(*card).and_modify(|c| *c += 1);
            } else {
                cards_by_count.insert(*card, 1);
            }
        }
        let mut sortable: Vec<u8> = Vec::new();
        let mut count_jays: u8 = 0;
        for (card, count) in cards_by_count {
            if card != 'J' {
                sortable.push(count);
            }

            if card == 'J' {
                count_jays = count;
            }
        }
        sortable.sort_by(|a, b| match a < b {
            true => Ordering::Greater,
            false => Ordering::Less,
        });
        let count_fh = *sortable.first().unwrap_or(&0);
        let count_sh = sortable.get(1).unwrap_or(&0);

        match count_jays {
            5 => self.hand_type = HandType::FiveOfAKind,
            4 => self.hand_type = HandType::FiveOfAKind,
            3 => {
                if count_fh == 2 {
                    self.hand_type = HandType::FiveOfAKind;
                } else if count_fh == 1 {
                    self.hand_type = HandType::FourOfAKind;
                }
            }
            2 => {
                if count_fh == 3 {
                    self.hand_type = HandType::FiveOfAKind;
                } else if count_fh == 2 {
                    self.hand_type = HandType::FourOfAKind;
                } else if count_fh == 1 {
                    self.hand_type = HandType::ThreeOfAKind;
                }
            }
            1 => {
                if count_fh == 4 {
                    self.hand_type = HandType::FiveOfAKind;
                } else if count_fh == 3 {
                    self.hand_type = HandType::FourOfAKind;
                } else if count_fh == 2 && *count_sh == 2 {
                    self.hand_type = HandType::FullHouse;
                } else if count_fh == 2 {
                    self.hand_type = HandType::ThreeOfAKind;
                } else if count_fh == 1 {
                    self.hand_type = HandType::OnePair;
                }
            }
            _ => {}
        }

        if self.hand_type == HandType::Unknown {
            if count_fh == 0 {
                self.hand_type = HandType::FiveOfAKind;
            } else if count_fh + count_sh == 5 && *count_sh != 0 && *count_sh != 1 {
                self.hand_type = HandType::FullHouse;
                if count_jays > 0 {
                    self.hand_type = HandType::FourOfAKind;
                }
            } else if count_fh == 2 && *count_sh == 2 {
                self.hand_type = HandType::TwoPair;
                if count_jays > 0 {
                    self.hand_type = HandType::ThreeOfAKind;
                }
            } else {
                self.hand_type = HandType::get_by_count(count_fh);
            }
        }
    }

    fn compare(&self, other: &Hand, order: &Vec<char>) -> Ordering {
        let self_type = self.hand_type as u32;
        let other_type = other.hand_type as u32;
        if self_type != other_type {
            if self_type < other_type {
                return Ordering::Less;
            }
            return Ordering::Greater;
        }

        for i in 0..5 {
            let mut self_card: usize = usize::MAX;
            let mut other_card: usize = usize::MAX;
            let self_char: char = *self.cards.get(i).expect("to have a char");
            let other_char: char = *other.cards.get(i).expect("to have a char");
            for j in 0..order.len() {
                let current: char = *order.get(j).expect("to find a char");
                if self_char == current {
                    self_card = j;
                }
                if other_char == current {
                    other_card = j;
                }
            }

            if self_card == other_card {
                continue;
            }

            if self_card < other_card {
                return Ordering::Less;
            }

            return Ordering::Greater;
        }

        Ordering::Equal
    }
}

fn read_file_line_by_line(
    filepath: &str,
    hands: &mut Vec<Hand>,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("content");
        hands.push(Hand::new(&line));
    }

    Ok(())
}

fn main() {
    let mut hands: Vec<Hand> = Vec::new();
    let order: Vec<char> = vec![
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ];
    let _ = read_file_line_by_line("src/day7_input", &mut hands);

    hands.sort_by(|a, b| a.compare(b, &order));

    let mut total_winnings = 0;
    let mut multi: u32 = hands.len() as u32;
    for hand in hands {
        total_winnings += hand.bid * multi;

        multi -= 1;
    }

    println!("Total winnings: {}", total_winnings);
}
