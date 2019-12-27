use intcode::{Program, Word};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::collections::HashMap;

use self::Tile::*;

fn main() {
    let input = include_str!("input.txt");
    let prog = Program::parse(input).unwrap();

    // Start the game. How many block tiles are on the screen when the game exits?
    let mut game = Game::new();
    game.run(&prog);

    let num_blocks = game.tiles.values().filter(|t| **t == Tile::Block).count();
    println!("Part 1: {}", num_blocks); // 341

    game.render();
}

struct Game {
    tiles: HashMap<(Word, Word), Tile>,
    score: Word,
    width: Word,
    height: Word,
}

impl Game {
    fn new() -> Self {
        Self {
            tiles: HashMap::new(),
            score: 0,
            width: 0,
            height: 0,
        }
    }

    fn run(&mut self, prog: &Program) {
        let mut rt = prog.new_runtime();

        while let Some(cmd) = rt.step_read(3).unwrap() {
            match cmd.as_slice() {
                [-1, 0, s] => self.score = *s,
                [x, y, t] => self.draw_cmd(*x, *y, *t),
                _ => unreachable!(),
            }
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

    fn render(&self) {
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
                print!("{}", self.get(x, y).render(&neighbors[..]));
            }
            println!();
        }
        println!();
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
            Ball => "◯",
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
