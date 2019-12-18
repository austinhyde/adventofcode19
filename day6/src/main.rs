use std::collections::HashMap;

fn main() {
  let input: &str = include_str!("input.txt");

  /*
  For example, suppose you have the following map:

      COM)B
      B)C
      C)D
      D)E
      E)F
      B)G
      G)H
      D)I
      E)J
      J)K
      K)L

  Visually, the above map of orbits looks like this:

              G - H       J - K - L
             /           /
      COM - B - C - D - E - F
                     \
                      I

  In this visual representation, when two objects are connected by a line, the one on the right directly orbits the one on the left.

  Here, we can count the total number of orbits as follows:

      D directly orbits C and indirectly orbits B and COM, a total of 3 orbits.
      L directly orbits K and indirectly orbits J, E, D, C, B, and COM, a total of 7 orbits.
      COM orbits nothing.

  The total number of direct and indirect orbits in this example is 42.

  What is the total number of direct and indirect orbits in your map data?
  */

  // let test1: System = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L"
  //   .parse()
  //   .unwrap();
  // println!("Test 1: {}", test1.count_orbits());
  // let test2: System = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN"
  //   .parse()
  //   .unwrap();
  // println!(
  //   "Test 2: {}",
  //   test2.count_transfers(&"YOU".to_string(), &"SAN".to_string())
  // );

  let system: System = input.parse().unwrap();
  println!("Part 1: {}", system.count_orbits()); // 110190

  /*
  What is the minimum number of orbital transfers required to move from the object YOU are orbiting to the object SAN is orbiting? (Between the objects they are orbiting - not between YOU and SAN.)
  */
  println!(
    "Part 2: {}",
    system.count_transfers(&"YOU".to_string(), &"SAN".to_string())
  );
}

type Body = String;
struct Orbit(Body, Body);
impl std::str::FromStr for Orbit {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let parts: Vec<&str> = s.split(")").collect();
    Ok(Self(parts[0].to_string(), parts[1].to_string()))
  }
}

struct System {
  // map of children to parents
  // contains e.g. {B: COM, G: B, C: B, ...}
  parents: HashMap<Body, Body>,
}

impl System {
  pub fn new() -> Self {
    Self {
      parents: HashMap::new(),
    }
  }
  pub fn register(&mut self, Orbit(parent, child): Orbit) {
    self.parents.insert(child, parent);
  }

  pub fn count_orbits(&self) -> i32 {
    let mut n = 0;
    // starting at each body, count the number of traversals to the root node
    for body in self.bodies() {
      n += self.walk_to_root(body).skip(1).count() as i32
    }
    n
  }

  pub fn count_transfers(&self, a: &Body, b: &Body) -> i32 {
    /* given
                                YOU
                               /
              G - H       J - K - L
             /           /
      COM - B - C - D - E - F
                     \
                      I - SAN
    */

    // YOU-K-J-E-D-C-B-COM => COM-B-C-D-E-J-K
    let mut a_to_root = self.walk_to_root(a).skip(1).collect::<Vec<_>>();
    a_to_root.reverse();

    // SAN-I-D-C-B-COM => COM-B-C-D-I
    let mut b_to_root = self.walk_to_root(b).skip(1).collect::<Vec<_>>();
    b_to_root.reverse();
    // println!("parents: {:?}", self.parents);
    // println!("a: {:?}", a_to_root);
    // println!("b: {:?}", b_to_root);
    // println!(
    //   "(a,b): {:?}",
    //   zip_open(a_to_root, b_to_root)
    //     .filter(|(a, b)| a != b)
    //     .collect::<Vec<_>>()
    // );

    // (COM,COM)-(B,B)-(C,C)-(D,D)-(E,I)-(J,_)-(K,_)
    // zip_open(a_to_root, b_to_root)
    //   // (E,I)-(J,_)-(K,_)
    //   .filter(|(a, b)| a != b)
    //   .count() as i32
    //   + 1
    let mut n = 0;
    for (a, b) in zip_open(a_to_root, b_to_root).filter(|(a, b)| a != b) {
      if a.is_some() {
        n += 1
      }
      if b.is_some() {
        n += 1
      }
    }
    n
  }

  fn bodies(&self) -> impl Iterator<Item = &Body> {
    self.parents.keys()
  }

  // ffs rust, I could do with 80% less line noise
  fn walk_to_root<'a>(&'a self, from: &'a Body) -> impl Iterator<Item = &'a Body> {
    std::iter::successors(Some(from), move |n| self.parents.get(n.to_owned()))
  }
}

impl std::str::FromStr for System {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut system = Self::new();
    for orbit in s.lines().filter_map(|l| l.parse().ok()) {
      system.register(orbit);
    }
    Ok(system)
  }
}

// like iter.zip, except stops on the _last_ None
fn zip_open<I, T>(a: I, b: I) -> impl Iterator<Item = (Option<T>, Option<T>)>
where
  I: IntoIterator<Item = T>,
{
  let mut ai = a.into_iter();
  let mut bi = b.into_iter();
  std::iter::from_fn(move || match (ai.next(), bi.next()) {
    (None, None) => None,
    x => Some(x),
  })
}
