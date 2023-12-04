use std::{cmp, fmt};
use std::fmt::Formatter;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use regex::Regex;

struct Game {
    number: i32,
    entry_lists: Vec<EntryList>
}

struct EntryList {
    entries: Vec<Entry>
}

struct Entry {
    number: i32,
    color: String
}

enum Token {
    Game,
    Number(i32),
    Color(String),
    Semicolon,
    Colon,
    Comma,
    End
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Token::Game => write!(f, "Game"),
            Token::Number(n) => write!(f, "Number({})", n),
            Token::Color(c) => write!(f, "Color({})", c),
            Token::Semicolon => write!(f, ";"),
            Token::Colon => write!(f, ":"),
            Token::Comma => write!(f, ","),
            Token::End => write!(f, "End")
        }
    }
}

struct Parser {
    tokens: Vec<Token>,
    current: usize
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn parse(&mut self) -> Result<Vec<Game>, String> {
        self.parse_game_list()
    }

    // GameList → Game | Game GameList
    fn parse_game_list(&mut self) -> Result<Vec<Game>, String> {
        let mut games = Vec::new();

        loop {
            match self.current_token() {
                Some(Token::Game) => {
                    games.push(self.parse_game()?);
                },
                _ => break
            }
        }

        Ok(games)
    }

    // Game → 'Game' Number ':' EntryList
    fn parse_game(&mut self) -> Result<Game, String> {
        match self.current_token() {
            Some(Token::Game) => self.advance(),
            _ => return Err("Expected 'Game' token".to_string())
        }

        let number = self.parse_number()?;
        self.advance();

        match self.current_token() {
            Some(Token::Colon) => self.advance(),
            other => {
                return match other {
                    Some(value) => Err(format!("Expected token ':'. Got '{}'.", value)),
                    None => Err("Unexpected end.".to_string())
                }
            }
        }

        let mut entry_lists = Vec::new();

        while let Ok(entry_list) = self.parse_entry_list() {
            entry_lists.push(entry_list);

            if let Some(Token::Semicolon) = self.current_token() {
                self.advance();
            }
            else {
                break;
            }
        }

        Ok(Game {
            number,
            entry_lists
        })
    }

    // EntryList → Entry | Entry ';' EntryList
    fn parse_entry_list(&mut self) -> Result<EntryList, String> {
        let mut list = Vec::new();

        loop {
            list.push(self.parse_entry()?);
            match self.current_token() {
                Some(Token::Comma) => {
                    self.advance();
                    continue
                },
                _ => break
            }
        }

        Ok(EntryList {
            entries: list
        })
    }

    // Entry → Number Color
    fn parse_entry(&mut self) -> Result<Entry, String> {
        let number = self.parse_number()?;
        self.advance();

        let color = self.parse_color()?;
        self.advance();

        Ok(Entry {
            number,
            color
        })
    }

    // Number → [Numeric value handling]
    fn parse_number(&mut self) -> Result<i32, String> {
        match self.current_token() {
            Some(Token::Number(x)) => {
                Ok(*x)
            },
            other => {
                return match other {
                    Some(value) => Err(format!("Expected number. Got '{}'.", value)),
                    None => Err("Unexpected end.".to_string())
                }
            }
        }
    }

    // Color → red | green | blue
    fn parse_color(&mut self) -> Result<String, String> {
        match self.current_token() {
            Some(Token::Color(c)) => Ok(c.clone()),
            other => {
                return match other {
                    Some(value) => Err(format!("Expected color. Got '{}'.", value)),
                    None => Err("Unexpected end.".to_string())
                }
            }
        }
    }
}

fn tokenize(line: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let re = Regex::new(r"(Game)|(:)|(,)|(;)|(\d+)|([a-zA-Z]+)").unwrap();

    for cap in re.captures_iter(line) {
        match cap.iter().flatten().skip(1).next() {
            Some(m) => match m.as_str() {
                "Game" => tokens.push(Token::Game),
                ":" => tokens.push(Token::Colon),
                "," => tokens.push(Token::Comma),
                ";" => tokens.push(Token::Semicolon),
                word if word.parse::<i32>().is_ok() => {
                    tokens.push(Token::Number(word.parse().unwrap()))
                }
                color => tokens.push(Token::Color(color.to_string())),
            },
            None => return Err(format!("Unexpected token")),
        }
    }

    Ok(tokens)
}


fn main() -> Result<(), String> {
    let file_path =
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("input.txt");

    println!("Loading file: {}", file_path.display());

    let file: File = File::open(file_path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);

    let mut tokens: Vec<Token> = Vec::new();
    for line in reader.lines() {
        let line = line.map_err(|e| e.to_string())?;
        if let Ok(i) = tokenize(line.as_str()) {
            tokens.extend(i);
        }
    }

    let mut game_parser = Parser::new(tokens);
    let games = game_parser.parse()?;

    let red_cubes = 12;
    let green_cubes = 13;
    let blue_cubes = 14;

    let mut sum_1 = 0;

    for game in &games {
        let mut impossible = false;
        for entry_list in &game.entry_lists {
            for entry in &entry_list.entries {
                if entry.color == "red" && entry.number > red_cubes {
                    impossible = true;
                    break;
                }

                if entry.color == "green" && entry.number > green_cubes {
                    impossible = true;
                    break;
                }

                if entry.color == "blue" && entry.number > blue_cubes {
                    impossible = true;
                    break;
                }
            }
        }

        if !impossible {
            sum_1 += game.number
        }
    }

    println!("Result #1: {}", sum_1);

    let mut sum_2 = 0;

    for game in &games {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for entry_list in &game.entry_lists {
            for entry in &entry_list.entries {
                if entry.color == "red" {
                    min_red = cmp::max(min_red, entry.number)
                }

                if entry.color == "green" {
                    min_green = cmp::max(min_green, entry.number)
                }

                if entry.color == "blue" {
                    min_blue = cmp::max(min_blue, entry.number)
                }
            }
        }

        let mut power = 1;
        if min_blue > 0 {
            power *= min_blue;
        }

        if min_red > 0 {
            power *= min_red;
        }

        if min_green > 0 {
            power *= min_green;
        }

        sum_2 += power;
    }

    println!("Result #2: {}", sum_2);

    Ok(())
}
