use std::cmp::Ordering;
use std::collections::HashMap;
use std::f64;
use std::io;

#[derive(Eq, PartialEq, Clone, Debug)]
struct Asteroid {
    x: u32,
    y: u32,
}

impl Asteroid {
    fn dist_to(&self, other: &Self) -> f64 {
        ((f64::from(self.x) - f64::from(other.x)).powi(2)
            + (f64::from(self.y) - f64::from(other.y)).powi(2))
        .sqrt()
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Angle {
    numerator: i64,
    denominator: i64,
    first_180: bool,
}

impl Angle {
    fn create_from_asteroids(source: &Asteroid, dest: &Asteroid) -> Self {
        let numerator = i64::from(source.x) - i64::from(dest.x);
        let denominator = i64::from(source.y) - i64::from(dest.y);
        let first_180 = source.x < dest.x || (source.x == dest.x && dest.y < source.y);

        let mut angle = Self {
            numerator,
            denominator,
            first_180,
        };
        angle.simplify();
        angle
    }

    fn to_rads(&self) -> f64 {
        if self.denominator == 0 {
            return if self.first_180 {
                f64::consts::FRAC_PI_2
            } else {
                f64::consts::FRAC_PI_2 * 3f64
            };
        }

        let rads = if self.numerator <= 0 {
            ((self.numerator.abs() as f64) / (self.denominator.abs() as f64)).atan()
        } else {
            ((self.denominator.abs() as f64) / (self.numerator.abs() as f64)).atan()
                + f64::consts::FRAC_PI_2
        };

        if self.first_180 {
            rads
        } else {
            rads + f64::consts::PI
        }
    }

    fn simplify(&mut self) {
        let common_factor = gcd(self.numerator, self.denominator);
        self.numerator /= common_factor;
        self.denominator /= common_factor;

        if self.denominator < 0 {
            self.numerator *= -1;
            self.denominator *= -1;
        }
    }
}

impl Ord for Angle {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_rads().partial_cmp(&other.to_rads()).unwrap()
    }
}

impl PartialOrd for Angle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type Asteroids = Vec<Asteroid>;
type RelAsteroidMap = HashMap<Angle, Asteroids>;

fn main() {
    println!("{}", solve(true));
}

fn solve(is_v2: bool) -> u32 {
    let asteroids = build_asteroids();

    // highest visibility asteroid's relative map
    let mut relative_asteroid_map = asteroids
        .iter()
        .map(|a| build_relative_asteroid_map(&asteroids, &a))
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap();

    if is_v2 {
        let mut destroyed_count = 0;
        let mut keys = relative_asteroid_map
            .keys()
            .map(|k| k.clone())
            .collect::<Vec<Angle>>();
        keys.sort();

        loop {
            for angle in &keys {
                let v = relative_asteroid_map.get_mut(&angle).unwrap();
                if v.len() != 0 {
                    let popped_asteroid = v.pop().unwrap();
                    destroyed_count += 1;
                    if destroyed_count == 200 {
                        return popped_asteroid.x * 100 + popped_asteroid.y;
                    }
                }
            }
        }
    } else {
        relative_asteroid_map.len() as u32
    }
}

fn build_asteroids() -> Asteroids {
    let mut y = 0;
    let mut asteroids: Asteroids = vec![];

    loop {
        let line = read_line().unwrap();
        if line == "" {
            break;
        }

        asteroids.append(
            &mut line
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(x, _)| x as u32)
                .map(|x| Asteroid { x, y })
                .collect::<Asteroids>(),
        );

        y += 1;
    }

    asteroids
}

// map of asteroids, grouped by angle from source asteroid
// grouped values are sorted in descending distance from source
fn build_relative_asteroid_map(asteroids: &Asteroids, source: &Asteroid) -> RelAsteroidMap {
    let mut asteroid_map: RelAsteroidMap =
        asteroids.iter().fold(HashMap::new(), |mut map, asteroid| {
            if asteroid == source {
                return map;
            }

            let angle = Angle::create_from_asteroids(source, asteroid);
            if !map.contains_key(&angle) {
                map.insert(angle.clone(), vec![]);
            }
            map.get_mut(&angle).unwrap().push(asteroid.clone());

            map
        });

    let keys = asteroid_map
        .keys()
        .map(|k| k.clone())
        .collect::<Vec<Angle>>();

    for angle in keys {
        asteroid_map
            .get_mut(&angle)
            .unwrap()
            .sort_by(|a, b| source.dist_to(b).partial_cmp(&source.dist_to(a)).unwrap())
    }

    asteroid_map
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

fn read_line() -> Result<String, io::Error> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input.pop();
            Ok(input)
        }
        Err(error) => Err(error),
    }
}
