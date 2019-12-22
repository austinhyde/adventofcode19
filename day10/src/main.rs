use std::f32::consts::PI;

fn main() {
    let input = include_str!("input.txt");
    let asteroids = parse_asteroids(input);

    /*
    Find the best location for a new monitoring station. How many other asteroids can be detected from that location?
    */
    let (station, n) = find_station(&asteroids);
    println!("Part 1: {:?} => {}", station, n); // Coords(19, 11) => 230

    /*
    The Elves are placing bets on which will be the 200th asteroid to be vaporized. Win the bet by determining which asteroid that will be; what do you get if you multiply its X coordinate by 100 and then add its Y coordinate? (For example, 8,2 becomes 802.)
    */
    let ast = get_vaporization_targets(&station, &asteroids)[199];
    println!("Part 2: {:?} => {}", ast, ast.0 * 100 + ast.1);
}

fn parse_asteroids(s: &str) -> Vec<Coords> {
    /*
    The map indicates whether each position is empty (.) or contains an asteroid (#). The asteroids are much smaller than they appear on the map, and every asteroid is exactly in the center of its marked position. The asteroids can be described with X,Y coordinates where X is the distance from the left edge and Y is the distance from the top edge (so the top-left corner is 0,0 and the position immediately to its right is 1,0).
    */
    s.trim()
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.trim().char_indices().filter_map(move |(x, c)| match c {
                '#' => Some(Coords(x as i32, y as i32)),
                _ => None,
            })
        })
        .collect()
}

fn find_station(asteroids: &Vec<Coords>) -> (Coords, usize) {
    /*
    Your job is to figure out which asteroid would be the best place to build a new monitoring station. A monitoring station can detect any asteroid to which it has direct line of sight - that is, there cannot be another asteroid exactly between them. This line of sight can be at any angle, not just lines aligned to the grid or diagonally. The best location is the asteroid that can detect the largest number of other asteroids.
    */
    asteroids
        .iter()
        .map(|station| {
            (
                station.clone(),
                get_visible_asteroids(station, asteroids).len(),
            )
        })
        .max_by_key(|(_, n)| *n)
        .unwrap()
}

fn get_vaporization_targets(station: &Coords, asteroids: &Vec<Coords>) -> Vec<Coords> {
    // 1. get all visible asteroids from the station
    let mut targets = get_visible_asteroids(station, asteroids);

    // 2. sort according to angle from the station.
    targets.sort_by(|a, b| {
        let dir_a = (*a - *station).direction();
        let dir_b = (*b - *station).direction();
        dir_a.partial_cmp(&dir_b).unwrap()
    });

    // 3. create a new map with those asteroids removed
    let updated: Vec<_> = asteroids
        .iter()
        .filter(|a| !targets.contains(a))
        .map(|a| a.clone())
        .collect();

    // 4. if there are any left besides the station, collect targets from the new map
    if updated.len() > 1 {
        targets.extend(get_vaporization_targets(station, &updated));
    }

    return targets;
}

fn get_visible_asteroids(station: &Coords, asteroids: &Vec<Coords>) -> Vec<Coords> {
    // I don't really feel like figuring out the optimal solution right now... can do something clever with quad/r-trees
    let mut out = Vec::new();
    'outer: for target in asteroids {
        if target == station {
            continue;
        }
        for check in asteroids {
            if check == target {
                continue;
            }
            if check == station {
                continue;
            }
            if check.is_between(station, target) {
                continue 'outer;
            }
        }
        out.push(*target);
    }
    out
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct Coords(i32, i32);

impl Coords {
    // north,east,south,west: (0, 90, 180, 270)
    pub fn direction(&self) -> f32 {
        let d = (-self.1 as f32).atan2(self.0 as f32) * 180f32 / PI - 90f32;
        if d > 0f32 {
            360f32 - d
        } else {
            -d
        }
    }
    pub fn len_sq(self) -> i32 {
        self.dot(self)
    }
    pub fn cross_len(self, rhs: Coords) -> i32 {
        (self.0 * rhs.1 - self.1 * rhs.0).abs()
    }
    pub fn dot(self, rhs: Coords) -> i32 {
        self.0 * rhs.0 + self.1 * rhs.1
    }
    // https://stackoverflow.com/questions/328107/how-can-you-determine-a-point-is-between-two-other-points-on-a-line-segment
    pub fn is_between(&self, a: &Coords, b: &Coords) -> bool {
        let seg = b.clone() - a.clone();
        let to_a = self.clone() - a.clone();
        let cross = seg.cross_len(to_a);
        if cross != 0 {
            return false;
        }

        let dot = seg.dot(to_a);
        if dot < 0 {
            return false;
        }
        return dot <= seg.len_sq();
    }
}
impl std::ops::Sub<Coords> for Coords {
    type Output = Coords;
    fn sub(self, rhs: Coords) -> Coords {
        Coords(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        let asteroids = parse_asteroids(
            "
            .#..#
            .....
            #####
            ....#
            ...##
        ",
        );
        assert_eq!((Coords(3, 4), 8), find_station(&asteroids));
    }

    #[test]
    pub fn direction() {
        assert_eq!(0.0, Coords(0, -1).direction());
        assert_eq!(90.0, Coords(1, 0).direction());
        assert_eq!(180.0, Coords(0, 1).direction());
        assert_eq!(270.0, Coords(-1, 0).direction());
    }

    #[test]
    pub fn rel_direction() {
        assert_eq!(0.0, (Coords(8, 1) - Coords(8, 3)).direction());
        assert_eq!(45.0, (Coords(9, 2) - Coords(8, 3)).direction());
        assert_eq!(270.0, (Coords(2, 3) - Coords(8, 3)).direction());
    }

    #[test]
    fn example2() {
        let asteroids = parse_asteroids(
            "
            .#....#####...#..
            ##...##.#####..##
            ##...#...#.#####.
            ..#.....#...###..
            ..#.#.....#....##
        ",
        );

        let station = Coords(8, 3);

        let expected = vec![
            // first 9
            Coords(8, 1),
            Coords(9, 0),
            Coords(9, 1),
            Coords(10, 0),
            Coords(9, 2),
            Coords(11, 1),
            Coords(12, 1),
            Coords(11, 2),
            Coords(15, 2),
            // second 9
            Coords(12, 2),
            Coords(13, 2),
            Coords(14, 2),
            Coords(15, 2),
            Coords(12, 3),
            Coords(16, 4),
            Coords(15, 4),
            Coords(10, 4),
            Coords(4, 4),
            // third 9
            Coords(2, 4),
            Coords(2, 3),
            Coords(0, 2),
            Coords(1, 2),
            Coords(0, 1),
            Coords(1, 1),
            Coords(5, 2),
            Coords(1, 0),
            Coords(5, 1),
            // last 3 of first rotation
            Coords(6, 1),
            Coords(6, 0),
            Coords(7, 0),
            // second rotation
            Coords(8, 0),
            Coords(10, 1),
            Coords(14, 0),
            Coords(16, 1),
            Coords(13, 3),
            // last rotation
            Coords(14, 3),
        ];
        let targets = get_vaporization_targets(&station, &asteroids);
        assert_eq!(expected, targets);
    }
}
