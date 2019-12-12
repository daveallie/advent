use std::io;

#[derive(PartialEq, Clone, Copy)]
struct Dim {
    pos: i32,
    vel: i32,
}

impl Dim {
    fn new(pos: i32) -> Self {
        Self { pos, vel: 0 }
    }

    fn adjust_velocities(dim1: &mut Self, dim2: &mut Self) {
        let diff = dim1.pos - dim2.pos;

        if diff > 0 {
            dim1.vel -= 1;
            dim2.vel += 1;
        } else if diff < 0 {
            dim1.vel += 1;
            dim2.vel -= 1;
        }
    }

    fn step(&mut self) {
        self.pos += self.vel
    }
}

struct Moon {
    x: Dim,
    y: Dim,
    z: Dim,
}

impl Moon {
    fn adjust_velocities(moon1: &mut Moon, moon2: &mut Moon) {
        Dim::adjust_velocities(&mut moon1.x, &mut moon2.x);
        Dim::adjust_velocities(&mut moon1.y, &mut moon2.y);
        Dim::adjust_velocities(&mut moon1.z, &mut moon2.z);
    }

    fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            x: Dim::new(x),
            y: Dim::new(y),
            z: Dim::new(z),
        }
    }

    fn get_dim(&self, index: usize) -> &Dim {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("unknown dim index"),
        }
    }

    fn step(&mut self) {
        self.x.step();
        self.y.step();
        self.z.step();
    }

    fn energy(&self) -> i32 {
        let pos_sum = self.x.pos.abs() + self.y.pos.abs() + self.z.pos.abs();
        let vel_sum = self.x.vel.abs() + self.y.vel.abs() + self.z.vel.abs();
        pos_sum * vel_sum
    }
}

struct Moons {
    moons: Vec<Moon>,
    initial: [Vec<Dim>; 3],
    repeat_step: [Option<u32>; 3],
    step_count: u32,
}

impl Moons {
    fn new(moons: Vec<Moon>) -> Self {
        let initial = [
            moons.iter().map(|m| m.x).collect::<Vec<Dim>>(),
            moons.iter().map(|m| m.y).collect::<Vec<Dim>>(),
            moons.iter().map(|m| m.z).collect::<Vec<Dim>>(),
        ];
        let repeat_step = [None, None, None];
        Self {
            moons,
            step_count: 0,
            initial,
            repeat_step,
        }
    }

    fn energy(&self) -> i32 {
        self.moons.iter().map(Moon::energy).sum()
    }

    fn all_repeats_found(&self) -> bool {
        self.repeat_step.iter().all(|r| r.is_some())
    }

    fn step(&mut self) {
        self.calculate_velocities();
        self.step_positions();
        self.step_count += 1;

        (0..3).for_each(|dim_index| self.check_set_repeat_step(dim_index));
    }

    fn check_set_repeat_step(&mut self, index: usize) {
        if self.repeat_step[index].is_none() {
            let curr_dims = self.moons.iter().map(|m| m.get_dim(index));
            let initial_dims = &self.initial[index];

            if curr_dims.zip(initial_dims).all(|(curr, init)| curr == init) {
                self.repeat_step[index] = Some(self.step_count);
            }
        }
    }

    fn calculate_velocities(&mut self) {
        let len = self.moons.len();

        for i in 1..len {
            let (head, tail) = self.moons.split_at_mut(i);
            let mut moon1 = head.last_mut().unwrap();
            for mut moon2 in tail {
                Moon::adjust_velocities(&mut moon1, &mut moon2);
            }
        }
    }

    fn step_positions(&mut self) {
        self.moons.iter_mut().for_each(Moon::step);
    }
}

fn main() {
    println!("{}", solve(true));
}

fn solve(is_v2: bool) -> u64 {
    let mut moons: Vec<Moon> = vec![];

    loop {
        let line = read_line().unwrap();
        if line == "" {
            break;
        }

        let moon_pos = line[1..line.len() - 1].split(", ").collect::<Vec<&str>>();
        let x = extract_pos(&moon_pos, "x=");
        let y = extract_pos(&moon_pos, "y=");
        let z = extract_pos(&moon_pos, "z=");
        let moon = Moon::new(x, y, z);
        moons.push(moon);
    }

    let mut moons = Moons::new(moons);

    if is_v2 {
        // x, y, z cycle is just LCM(x cycle, y cycle, z cycle)
        while !moons.all_repeats_found() {
            moons.step();
        }

        let x_repeat = u64::from(moons.repeat_step[0].unwrap());
        let y_repeat = u64::from(moons.repeat_step[1].unwrap());
        let z_repeat = u64::from(moons.repeat_step[2].unwrap());

        lcm(x_repeat, lcm(y_repeat, z_repeat))
    } else {
        for _ in 0..1000 {
            moons.step();
        }
        moons.energy() as u64
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
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

fn extract_pos(parts: &Vec<&str>, target: &str) -> i32 {
    parts.iter().find(|pos| pos.starts_with(target)).unwrap()[2..]
        .parse::<i32>()
        .unwrap()
}
