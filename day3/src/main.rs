use std::str::FromStr;

fn main() {
  /*
  Opening the front panel reveals a jumble of wires. Specifically, two wires are connected to a central port and extend outward on a grid. You trace the path each wire takes as it leaves the central port, one wire per line of text (your puzzle input).

  The wires twist and turn, but the two wires occasionally cross paths. To fix the circuit, you need to find the intersection point closest to the central port. Because the wires are on a grid, use the Manhattan distance for this measurement. While the wires do technically cross right at the central port where they both start, this point does not count, nor does a wire count as crossing with itself.

  What is the Manhattan distance from the central port to the closest intersection?
  */
  let input = include_str!("input.txt");
  let wires = input
    .lines()
    .map(|l| l.parse())
    .collect::<Result<Vec<_>, _>>()
    .unwrap();

  let intersections = find_intersections(&wires[0], &wires[1]);
  let closest = closest_to_origin(&intersections);
  println!("Part 1: {} (dist={})", closest, closest.dist_from_origin());

  let fastest = fastest_from_origin(&wires, &intersections);
  println!("Part 2: {} ({} steps)", fastest.0, fastest.1);
}

fn find_intersections(w1: &Wire, w2: &Wire) -> Vec<Node> {
  let mut out = Vec::new();
  for s1 in &w1.0 {
    for s2 in &w2.0 {
      if let Some(n) = s1.crosses(s2) {
        out.push(n);
      }
    }
  }
  out
}

fn closest_to_origin(nodes: &Vec<Node>) -> &Node {
  let mut iter = nodes.iter();
  let mut out = iter.next().unwrap();
  for n in iter {
    if n.dist_from_origin() < out.dist_from_origin() {
      out = n
    }
  }
  return out;
}

fn fastest_from_origin<'a, 'b>(
  wires: &'a Vec<Wire>,
  intersections: &'b Vec<Node>,
) -> (&'b Node, Dist) {
  let mut iter = intersections.iter();

  let mut out = iter.next().unwrap();
  let mut dist = wires.iter().filter_map(|w| w.locate(&out)).sum();
  for n in iter {
    let d = wires.iter().filter_map(|w| w.locate(&n)).sum();
    if d < dist {
      out = n;
      dist = d;
    }
  }
  return (out, dist);
}

type Coord = i32;
type Dist = i32;

#[derive(Clone, PartialEq, Copy)]
enum Orientation {
  Horiz,
  Vert,
}

enum Direction {
  Up,
  Right,
  Down,
  Left,
}

impl FromStr for Direction {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_ref() {
      "u" => Ok(Direction::Up),
      "r" => Ok(Direction::Right),
      "d" => Ok(Direction::Down),
      "l" => Ok(Direction::Left),
      x => Err(format!("Unknown direction {}", x)),
    }
  }
}

struct Instruction {
  pub dir: Direction,
  pub len: Dist,
}

impl Instruction {
  fn orientation(&self) -> Orientation {
    match self.dir {
      Direction::Up => Orientation::Vert,
      Direction::Down => Orientation::Vert,
      Direction::Left => Orientation::Horiz,
      Direction::Right => Orientation::Horiz,
    }
  }
  fn travel(&self, n: &Node) -> Node {
    match self.dir {
      Direction::Up => Node {
        x: n.x,
        y: n.y + self.len,
      },
      Direction::Right => Node {
        x: n.x + self.len,
        y: n.y,
      },
      Direction::Down => Node {
        x: n.x,
        y: n.y - self.len,
      },
      Direction::Left => Node {
        x: n.x - self.len,
        y: n.y,
      },
    }
  }
}

impl FromStr for Instruction {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let invalid = format!("Invalid instruction: {}", s);
    Ok(Self {
      dir: s
        .chars()
        .nth(0)
        .ok_or(invalid.clone())?
        .to_string()
        .parse()?,
      len: s[1..].parse().map_err(|_| invalid.clone())?,
    })
  }
}

struct Wire(Vec<Segment>);

impl Wire {
  fn locate(&self, n: &Node) -> Option<Dist> {
    let mut dist: Dist = 0;
    for seg in &self.0 {
      if let Some(d) = seg.locate(n) {
        return Some(d + dist);
      }
      dist += seg.length();
    }
    // if we make it the whole way through without finding the node
    return None;
  }
}

impl FromStr for Wire {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut curr = Node { x: 0, y: 0 };
    let mut out = Vec::new();

    let insts: Result<Vec<_>, _> = s.split(",").map(|s| s.parse()).collect();
    for inst in insts? {
      let seg = curr.travel(inst);
      curr = seg.end;
      out.push(seg);
    }

    Ok(Self(out))
  }
}

#[derive(Clone, Copy)]
struct Segment {
  pub start: Node,
  pub end: Node,
  pub orientation: Orientation,
}

impl Segment {
  fn length(&self) -> Dist {
    self.start.dist_from(&self.end)
  }
  fn left(&self) -> Coord {
    self.start.x.min(self.end.x)
  }
  fn right(&self) -> Coord {
    self.start.x.max(self.end.x)
  }
  fn top(&self) -> Coord {
    self.start.y.max(self.end.y)
  }
  fn bottom(&self) -> Coord {
    self.start.y.min(self.end.y)
  }

  fn left_of(&self, other: &Segment) -> bool {
    self.right() < other.left()
  }
  fn right_of(&self, other: &Segment) -> bool {
    self.left() > other.right()
  }
  fn above(&self, other: &Segment) -> bool {
    self.bottom() > other.top()
  }
  fn below(&self, other: &Segment) -> bool {
    self.top() < other.bottom()
  }

  fn crosses(&self, other: &Segment) -> Option<Node> {
    match self.orientation {
      // parallel lines can't cross
      x if x == other.orientation => None,
      // let's only deal with the vertical case
      Orientation::Horiz => other.crosses(self),
      Orientation::Vert => {
        if self.left_of(other) || self.right_of(other) || self.above(other) || self.below(other) {
          return None;
        }
        return Some(Node {
          x: self.left(),
          y: other.top(),
        });
      }
    }
  }

  fn contains(&self, n: &Node) -> bool {
    match self.orientation {
      Orientation::Horiz => self.top() == n.y && self.left() <= n.x && self.right() >= n.x,
      Orientation::Vert => self.left() == n.x && self.bottom() <= n.y && self.top() >= n.y,
    }
  }

  fn locate(&self, n: &Node) -> Option<Dist> {
    if self.contains(n) {
      Some(n.dist_from(&self.start))
    } else {
      None
    }
  }
}

#[derive(Clone, Copy)]
struct Node {
  pub x: Coord,
  pub y: Coord,
}
impl Node {
  fn dist_from(&self, n: &Node) -> Dist {
    (self.x - n.x).abs() + (self.y - n.y).abs()
  }
  fn dist_from_origin(&self) -> Dist {
    self.x.abs() + self.y.abs()
  }
  fn travel(self, inst: Instruction) -> Segment {
    Segment {
      start: self,
      end: inst.travel(&self),
      orientation: inst.orientation(),
    }
  }
}
impl std::fmt::Display for Node {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}
