use intcode::{Program, Word};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let prog = Program::parse(input).unwrap();

    /*
    Build a new emergency hull painting robot and run the Intcode program on it. How many panels does it paint at least once?
    */
    let mut robot = Robot::new();
    robot.run(&prog).unwrap();
    println!("Part 1: {}", robot.painted_panels()); // 1909

    /*
    Checking your external ship cameras again, you notice a white panel marked "emergency hull painting robot starting panel". The rest of the panels are still black, but it looks like the robot was expecting to start on a white panel, not a black one.
    */
    let mut robot = Robot::new();
    robot.paint(Color::White);
    robot.run(&prog).unwrap();
    println!("Part 2:\n{}", robot.panels_to_string());
    /*
    ..##...##..#....####.#..#.#..#.#....#..#...
    .#..#.#..#.#....#....#.#..#..#.#....#..#...
    ....#.#..#.#....#....#.#..#..#.###..#..#...
    ....#.#..#.###..###..##...####.#..#.####...
    ....#.#..#.#....#....#.#..#..#.#..#.#..#...
    ...##.#..#.####.####.#..#.#..#.###..#..#...
    */
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
struct Coord(i32, i32);

#[derive(FromPrimitive, ToPrimitive, Debug, PartialEq)]
enum Direction {
    Up = 0,
    Right,
    Down,
    Left,
}
#[derive(FromPrimitive)]
enum TurnDir {
    Left = 0,
    Right,
}

#[derive(Copy, Clone, FromPrimitive)]
enum Color {
    Black = 0,
    White,
}

struct Robot {
    panels: HashMap<Coord, Color>,
    loc: Coord,
    dir: Direction,
}
impl Robot {
    pub fn new() -> Self {
        Self {
            panels: HashMap::new(),
            loc: Coord(0, 0),
            dir: Direction::Up,
        }
    }

    pub fn painted_panels(&self) -> usize {
        self.panels.len()
    }

    pub fn panels_to_string(&self) -> String {
        let mut out = String::new();
        let (minx, miny, maxx, maxy) =
            self.panels
                .keys()
                .fold((0, 0, 0, 0), |(minx, miny, maxx, maxy), Coord(x, y)| {
                    (minx.min(*x), miny.min(*y), maxx.max(*x), maxy.max(*y))
                });

        for y in miny..=maxy {
            for x in minx..=maxx {
                out += match self.panels.get(&Coord(x, y)).unwrap_or(&Color::Black) {
                    Color::Black => ".",
                    Color::White => "#",
                }
            }
            out += "\n";
        }

        out
    }

    pub fn run(&mut self, prog: &Program) -> Result<(), String> {
        /*
        The Intcode program will serve as the brain of the robot. The program uses input instructions to access the robot's camera: provide 0 if the robot is over a black panel or 1 if the robot is over a white panel. Then, the program will output two values:

        First, it will output a value indicating the color to paint the panel the robot is over: 0 means to paint the panel black, and 1 means to paint the panel white.
        Second, it will output a value indicating the direction the robot should turn: 0 means it should turn left 90 degrees, and 1 means it should turn right 90 degrees.
        */

        let mut rt = prog.new_runtime();
        while rt.start()? {
            let out = rt.stepn(vec![self.read_camera() as Word], 2)?;
            self.prog_command(out[0], out[1]);
        }
        Ok(())
    }

    fn prog_command(&mut self, color: Word, dir: Word) {
        self.paint(Color::from_i64(color).unwrap());
        self.turn(TurnDir::from_i64(dir).unwrap());
        self.advance(1);
    }

    fn read_camera(&self) -> Color {
        *self.panels.get(&self.loc).unwrap_or(&Color::Black)
    }

    fn paint(&mut self, c: Color) {
        self.panels.insert(self.loc, c);
    }
    fn turn(&mut self, d: TurnDir) {
        self.dir = match d {
            TurnDir::Left => match self.dir {
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Up,
            },
            TurnDir::Right => match self.dir {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            },
        }
    }
    fn advance(&mut self, n: i32) {
        self.loc = match self.dir {
            Direction::Up => Coord(self.loc.0, self.loc.1 + n),
            Direction::Right => Coord(self.loc.0 + n, self.loc.1),
            Direction::Down => Coord(self.loc.0, self.loc.1 - n),
            Direction::Left => Coord(self.loc.0 - n, self.loc.1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut robot = Robot::new();
        robot.prog_command(1, 0);
        robot.prog_command(0, 0);
        robot.prog_command(1, 0);
        robot.prog_command(1, 0);
        robot.prog_command(0, 1);
        robot.prog_command(1, 0);
        robot.prog_command(1, 0);
        assert_eq!(robot.loc, Coord(0, 1));
        assert_eq!(robot.dir, Direction::Left);
        assert_eq!(robot.painted_panels(), 6);
    }
}
