use std::fs;

enum GameOutcome {
    WIN,
    LOSS,
    DRAW,
}

enum GameItem {
    ROCK,
    PAPER,
    SCISSORS,
}

impl GameItem {
    fn from_str(input: &str) -> GameItem {
        return match input {
            "A" | "X" => GameItem::ROCK,
            "B" | "Y" => GameItem::PAPER,
            "C" | "Z" => GameItem::SCISSORS,
            _ => panic!("Invalid input"),
        };
    }

    fn from_instruction(p1_val: GameItem, instruction: &str) -> GameItem {
        return match instruction {
            "X" => GameItem::get_response(GameOutcome::LOSS, p1_val),
            "Y" => GameItem::get_response(GameOutcome::DRAW, p1_val),
            "Z" => GameItem::get_response(GameOutcome::WIN, p1_val),
            _ => panic!("Invalid instruction"),
        };
    }

    fn points(&self) -> u32 {
        return match self {
            GameItem::ROCK => 1,
            GameItem::PAPER => 2,
            GameItem::SCISSORS => 3,
        };
    }

    fn get_response(result: GameOutcome, p1_val: GameItem) -> GameItem {
        return match result {
            GameOutcome::WIN => match p1_val {
                GameItem::ROCK => GameItem::PAPER,
                GameItem::PAPER => GameItem::SCISSORS,
                GameItem::SCISSORS => GameItem::ROCK,
            },
            GameOutcome::LOSS => match p1_val {
                GameItem::ROCK => GameItem::SCISSORS,
                GameItem::PAPER => GameItem::ROCK,
                GameItem::SCISSORS => GameItem::PAPER,
            },
            _ => p1_val,
        };
    }
}

struct Game {
    p_1_game_item: GameItem,
    p_2_game_item: GameItem,
    p_1_points: u32,
    p_2_points: u32,
}

impl Game {
    const WINNER_BONUS: u32 = 6;
    const EQUAL_BONUS: u32 = 3;
    const LOSER_BONUS: u32 = 0;

    fn new(p_1: &str, p_2: &str, is_instruction: bool) -> Game {
        return Game {
            p_1_game_item: GameItem::from_str(p_1),
            p_2_game_item: if is_instruction {
                GameItem::from_instruction(GameItem::from_str(p_1), p_2)
            } else {
                GameItem::from_str(p_2)
            },
            p_1_points: 0,
            p_2_points: 0,
        };
    }

    fn play(&mut self) {
        self.p_1_points = self.p_1_game_item.points();
        self.p_2_points = self.p_2_game_item.points();
        self.add_outcome_bonus();
    }

    fn add_outcome_bonus(&mut self) {
        let (p_1_bonus, p_2_bonus) = match (&self.p_1_game_item, &self.p_2_game_item) {
            (GameItem::ROCK, GameItem::PAPER) => (Game::LOSER_BONUS, Game::WINNER_BONUS),
            (GameItem::ROCK, GameItem::SCISSORS) => (Game::WINNER_BONUS, Game::LOSER_BONUS),
            (GameItem::PAPER, GameItem::ROCK) => (Game::WINNER_BONUS, Game::LOSER_BONUS),
            (GameItem::PAPER, GameItem::SCISSORS) => (Game::LOSER_BONUS, Game::WINNER_BONUS),
            (GameItem::SCISSORS, GameItem::ROCK) => (Game::LOSER_BONUS, Game::WINNER_BONUS),
            (GameItem::SCISSORS, GameItem::PAPER) => (Game::WINNER_BONUS, Game::LOSER_BONUS),
            _ => (Game::EQUAL_BONUS, Game::EQUAL_BONUS),
        };
        self.p_1_points += p_1_bonus;
        self.p_2_points += p_2_bonus;
    }
}

fn main() {
    let mut games_from_str: Vec<Game> = vec![];
    let mut games_from_instruction: Vec<Game> = vec![];
    let input: String = fs::read_to_string("input_p1").expect("Unable to read file");
    let input_lines: Vec<&str> = input.lines().collect::<Vec<&str>>();

    for line in input_lines {
        let items: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();
        let mut game_from_str: Game = Game::new(items[0], items[1], false);
        let mut game_from_instruction: Game = Game::new(items[0], items[1], true);
        game_from_str.play();
        games_from_str.push(game_from_str);
        game_from_instruction.play();
        games_from_instruction.push(game_from_instruction);
    }

    // Part 1
    println!(
        "Part 1: {:?}",
        games_from_str.iter_mut().map(|x| x.p_2_points).sum::<u32>()
    );

    // Part 2
    println!(
        "Part 2: {:?}",
        games_from_instruction
            .iter_mut()
            .map(|x| x.p_2_points)
            .sum::<u32>()
    );
}
