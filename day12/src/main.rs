use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");
    let mut system: System = input.parse().unwrap();

    /*
    What is the total energy in the system after simulating the moons given in your scan for 1000 steps?
    */
    system.simulate(1000);
    println!("Part 1: {}", system.total_energy()); // 8454
}

/*
The space near Jupiter is not a very safe place; you need to be careful of a big distracting red spot, extreme radiation, and a whole lot of moons swirling around. You decide to start by tracking the four largest moons: Io, Europa, Ganymede, and Callisto.
*/
#[derive(PartialEq)]
struct System {
    moons: Vec<Moon>,
}
impl System {
    fn new(moons: Vec<Moon>) -> Self {
        Self { moons }
    }
    fn simulate(&mut self, steps: usize) {
        for _ in 0..steps {
            self.step();
        }
    }
    fn step(&mut self) {
        // TODO remove this clone
        let other_moons = self.moons.clone();
        // Within each time step, first update the velocity of every moon by applying gravity
        for moon in &mut self.moons {
            // To apply gravity, consider every pair of moons.
            for other in &other_moons {
                if moon != other {
                    moon.apply_gravity(other);
                }
            }
        }
        // Then, once all moons' velocities have been updated, update the position of every moon by applying velocity
        for moon in &mut self.moons {
            moon.apply_velocity();
        }
    }
    fn total_energy(&self) -> i32 {
        self.moons.iter().map(Moon::total_energy).sum()
    }
}
impl FromStr for System {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        Ok(Self::new(
            s.trim()
                .lines()
                .map(|l| l.parse())
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}
impl std::fmt::Debug for System {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f)?;
        for m in &self.moons {
            writeln!(f, "{:?}", m)?;
        }
        Ok(())
    }
}

/*
Each moon has a 3-dimensional position (x, y, and z) and a 3-dimensional velocity. The position of each moon is given in your scan; the x, y, and z velocity of each moon starts at 0.
*/
#[derive(PartialEq, Clone, Copy)]
struct Moon {
    position: Vector,
    velocity: Vector,
}

impl Moon {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            position: Vector::new(x, y, z),
            velocity: Vector::zero(),
        }
    }
    fn apply_gravity(&mut self, other: &Moon) {
        /*
        On each axis (x, y, and z), the velocity of each moon changes by exactly +1 or -1 to pull the moons together. For example, if Ganymede has an x position of 3, and Callisto has a x position of 5, then Ganymede's x velocity changes by +1 (because 5 > 3) and Callisto's x velocity changes by -1 (because 3 < 5). However, if the positions on a given axis are the same, the velocity on that axis does not change for that pair of moons.
        */
        self.velocity += (other.position - self.position).signum()
    }
    fn apply_velocity(&mut self) {
        // simply add the velocity of each moon to its own position.
        self.position += self.velocity.clone()
    }
    fn total_energy(&self) -> i32 {
        // The total energy for a single moon is its potential energy multiplied by its kinetic energy.
        self.potential_energy() * self.kinetic_energy()
    }
    fn potential_energy(&self) -> i32 {
        // A moon's potential energy is the sum of the absolute values of its x, y, and z position coordinates
        self.position.abs_sum()
    }
    fn kinetic_energy(&self) -> i32 {
        // A moon's kinetic energy is the sum of the absolute values of its velocity coordinates
        self.velocity.abs_sum()
    }
}
impl std::fmt::Debug for Moon {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "pos={:?}, vel={:?}", self.position, self.velocity)
    }
}

impl FromStr for Moon {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        let parts: Vec<i32> = s
            .trim()
            .trim_matches(|c| c == '<' || c == '>')
            .split(",")
            .map(|s| s.trim()[2..].parse().expect("invalid position"))
            .collect();
        Ok(Moon::new(parts[0], parts[1], parts[2]))
    }
}

#[derive(PartialEq, Copy, Clone)]
struct Vector {
    x: i32,
    y: i32,
    z: i32,
}

impl Vector {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
    fn zero() -> Self {
        Self::new(0, 0, 0)
    }
    fn abs_sum(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
    fn signum(&self) -> Vector {
        Vector::new(self.x.signum(), self.y.signum(), self.z.signum())
    }
}
impl std::fmt::Debug for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<x={:>3}, y={:>3}, z={:>3}>", self.x, self.y, self.z)
    }
}
impl std::ops::AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl std::ops::Sub for Vector {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gravity() {
        // For example, if Ganymede has an x position of 3, and Callisto has a x position of 5,
        let mut ganymede = Moon::new(3, 5, 2);
        let mut callisto = Moon::new(5, 3, 2);

        ganymede.apply_gravity(&callisto);
        callisto.apply_gravity(&ganymede);

        // then Ganymede's x velocity changes by +1 (because 5 > 3) and Callisto's x velocity changes by -1 (because 3 < 5).
        // However, if the positions on a given axis are the same, the velocity on that axis does not change for that pair of moons.
        assert_eq!(ganymede.position, Vector::new(3, 5, 2));
        assert_eq!(ganymede.velocity, Vector::new(1, -1, 0));
        assert_eq!(callisto.position, Vector::new(5, 3, 2));
        assert_eq!(callisto.velocity, Vector::new(-1, 1, 0));
    }

    #[test]
    fn velocity() {
        // For example, if Europa has a position of x=1, y=2, z=3 and a velocity of x=-2, y=0,z=3,
        let mut europa = Moon {
            position: Vector::new(1, 2, 3),
            velocity: Vector::new(-2, 0, 3),
        };

        europa.apply_velocity();

        // then its new position would be x=-1, y=2, z=6. This process does not modify the velocity of any moon.
        assert_eq!(europa.position, Vector::new(-1, 2, 6));
        assert_eq!(europa.velocity, Vector::new(-2, 0, 3));
    }

    #[test]
    fn energy() {
        let moon = Moon {
            position: Vector::new(2, 1, -3),
            velocity: Vector::new(-3, -2, 1),
        };

        assert_eq!(moon.potential_energy(), 6);
        assert_eq!(moon.kinetic_energy(), 6);
        assert_eq!(moon.total_energy(), 36);
    }

    #[test]
    fn example1() {
        let mut system: System = "
                <x=-1, y=0, z=2>
                <x=2, y=-10, z=-7>
                <x=4, y=-8, z=8>
                <x=3, y=5, z=-1>
            "
        .parse()
        .unwrap();

        // make sure we parse it correctly
        assert_eq!(
            system,
            System {
                moons: vec![
                    Moon::new(-1, 0, 2),
                    Moon::new(2, -10, -7),
                    Moon::new(4, -8, 8),
                    Moon::new(3, 5, -1),
                ],
            }
        );

        // make sure we step correctly
        system.step();
        assert_eq!(
            system,
            System {
                moons: vec![
                    Moon {
                        position: Vector::new(2, -1, 1),
                        velocity: Vector::new(3, -1, -1),
                    },
                    Moon {
                        position: Vector::new(3, -7, -4),
                        velocity: Vector::new(1, 3, 3),
                    },
                    Moon {
                        position: Vector::new(1, -7, 5),
                        velocity: Vector::new(-3, 1, -3),
                    },
                    Moon {
                        position: Vector::new(2, 2, 0),
                        velocity: Vector::new(-1, -3, 1),
                    },
                ]
            }
        );

        // run 9 more times and check state
        system.simulate(9);
        assert_eq!(
            system,
            System {
                moons: vec![
                    Moon {
                        position: Vector::new(2, 1, -3),
                        velocity: Vector::new(-3, -2, 1),
                    },
                    Moon {
                        position: Vector::new(1, -8, 0),
                        velocity: Vector::new(-1, 1, 3),
                    },
                    Moon {
                        position: Vector::new(3, -6, 1),
                        velocity: Vector::new(3, 2, -3),
                    },
                    Moon {
                        position: Vector::new(2, 0, 4),
                        velocity: Vector::new(1, -1, -1),
                    },
                ]
            }
        );

        // check energy after 10 steps
        assert_eq!(system.total_energy(), 179);
    }

    #[test]
    fn example2() {
        let mut system: System = "
                <x=-8, y=-10, z=0>
                <x=5, y=5, z=10>
                <x=2, y=-7, z=3>
                <x=9, y=-8, z=-3>
            "
        .parse()
        .unwrap();

        system.simulate(100);
        assert_eq!(system.total_energy(), 1940);
    }
}
