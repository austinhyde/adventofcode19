use crossterm::event::{KeyCode, KeyEvent};
use intcode::{Program, RuntimeState, Word};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use self::Tile::*;

mod screen;
use screen::Screen;

fn main() {
    let input = include_str!("input.txt");
    let prog = Program::parse(input).unwrap();
    let mut screen = Screen::new();

    // Start the game. How many block tiles are on the screen when the game exits?
    let mut game = Game::new();
    game.run(&prog, &mut screen);

    let num_blocks = game.tiles.values().filter(|t| **t == Tile::Block).count();
    println!("Part 1: {}", num_blocks); // 341

    /*
    The game didn't run because you didn't put in any quarters. Unfortunately, you did not bring any quarters. Memory address 0 represents the number of quarters that have been inserted; set it to 2 to play for free.
    */
    game.insert_quarters(2);
    game.run(&prog, &mut screen);
}

struct Game {
    tiles: HashMap<(Word, Word), Tile>,
    last_score: Word,
    score: Word,
    width: Word,
    height: Word,
    quarters: Option<Word>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            tiles: HashMap::new(),
            last_score: 0,
            score: 0,
            width: 0,
            height: 0,
            quarters: None,
        }
    }

    pub fn insert_quarters(&mut self, n: Word) {
        self.quarters = Some(n);
    }

    pub fn run(&mut self, prog: &Program, screen: &mut Screen) {
        let mut rt = prog.new_runtime();

        if let Some(n) = self.quarters {
            rt.set(0, n).unwrap();
        }
        screen.clear();
        // screen.hide();

        let mut input = 0;

        let mut state = rt.resume(None).unwrap();
        while let RuntimeState::Resumable(_) = state {
            // 1. execute draw commands
            while let RuntimeState::Resumable(Some(x)) = state {
                let yt = rt.step_read(2).unwrap().unwrap();
                if x == -1 && yt[0] == 0 {
                    self.last_score = self.score;
                    self.score = yt[1];
                } else {
                    self.draw_cmd(x, yt[0], yt[1]);
                }
                state = rt.resume(None).unwrap();
            }

            // 2. render screen once drawn
            self.render(screen);

            // 3. if we're done bail early
            if let RuntimeState::Complete = state {
                break;
            }

            // 4. collect input
            let budget = Duration::from_millis(200);
            let start = Instant::now();

            input = match screen.key_pressed(budget) {
                Some(KeyEvent {
                    code: KeyCode::Left,
                    ..
                }) => -1,
                Some(KeyEvent {
                    code: KeyCode::Right,
                    ..
                }) => 1,
                _ => 0,
            };
            let end = Instant::now();

            // 5. slow things down
            if end - start < budget {
                std::thread::sleep((start + budget) - end);
            }

            // 6. continue
            state = rt.resume(Some(input)).unwrap();
        }
    }

    fn draw_cmd(&mut self, x: Word, y: Word, t: Word) {
        self.tiles.insert((x, y), Tile::from_i64(t).unwrap());
        self.width = self.width.max(x);
        self.height = self.height.max(y);
    }

    fn get(&self, x: Word, y: Word) -> &Tile {
        self.tiles.get(&(x, y)).unwrap_or(&Tile::Nil)
    }

    fn render(&self, screen: &mut Screen) {
        for y in 0..=self.height {
            for x in 0..=self.width {
                let neighbors = [
                    self.get(x - 1, y - 1),
                    self.get(x, y - 1),
                    self.get(x + 1, y - 1),
                    self.get(x - 1, y),
                    self.get(x + 1, y),
                    self.get(x - 1, y + 1),
                    self.get(x, y + 1),
                    self.get(x + 1, y + 1),
                ];
                screen.put(x, y, &self.get(x, y).render(&neighbors[..]));
            }
        }
        screen.put(
            self.width + 2,
            2,
            &" ".repeat(self.last_score.to_string().len()),
        );
        screen.put(self.width + 2, 2, &self.score.to_string());
        screen.moveto(0, self.height + 2);
    }
}

#[derive(FromPrimitive, Debug, PartialEq)]
enum Tile {
    Nil = -1,
    Empty = 0,
    Wall,
    Block,
    Paddle,
    Ball,
}
impl Tile {
    // neighbors is 0 1 2
    //              3   4
    //              5 6 7
    fn render(&self, neighbors: &[&Tile]) -> String {
        match self {
            Nil | Empty => " ",
            Wall => match neighbors {
                // top-left
                [Nil, Nil, Nil, Nil, Wall, Nil, Wall, _] => "╔",
                // top/bottom
                [_, _, _, Wall, Wall, _, _, _] => "═",
                // top-right
                [Nil, Nil, Nil, Wall, Nil, _, Wall, Nil] => "╗",
                // bottom-left
                [Nil, Wall, _, Nil, Wall, Nil, Nil, Nil] => "╚",
                // bottom-right
                [_, Wall, Nil, Wall, Nil, Nil, Nil, Nil] => "╝",
                // left/right
                [_, Wall, _, _, _, _, _, _] | [_, _, _, _, _, _, Wall, _] => "║",
                // unknown?
                _ => "╬",
            },
            Block => "◼",
            Paddle => "─",
            Ball => "○",
        }
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        let mut game = Game::new();
        game.draw_cmd(1, 2, 3);
        game.draw_cmd(6, 5, 4);

        assert_eq!(game.tiles.get(&(1, 2)), Some(&Tile::Paddle));
        assert_eq!(game.tiles.get(&(6, 5)), Some(&Tile::Ball));
    }
}
