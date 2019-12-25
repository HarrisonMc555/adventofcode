use regex::Regex;

const INPUT: &str = include_str!("../../static/day12.txt");
const DEFAULT_NUM_STEPS: usize = 1000;

type Result<T> = std::result::Result<T, String>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Body {
    pos: Vec3<isize>,
    vel: Vec3<isize>,
}

pub fn main() {
    let answer1 = solve1(INPUT, DEFAULT_NUM_STEPS);
    println!("{:?}", answer1);
    // let answer2 = solve2(INPUT);
    // println!("{:?}", answer2);
}

fn solve1(input: &str, num_steps: usize) -> Result<isize> {
    let mut bodies = parse_input(input)?;
    for _ in 0..num_steps {
        update_bodies(&mut bodies);
    }
    let energy = calc_energy(&bodies);
    Ok(energy)
}

fn update_bodies(bodies: &mut [Body]) {
    update_velocities(bodies);
    update_positions(bodies);
}

fn update_velocities(bodies: &mut [Body]) {
    for i in 0..bodies.len() {
        for j in (0..bodies.len()).filter(|&j| j != i) {
            let other = bodies[j];
            let body = &mut bodies[i];
            body.update_velocity(other);
        }
    }
}

fn update_positions(bodies: &mut [Body]) {
    for body in bodies.iter_mut() {
        body.update_position()
    }
}

fn calc_energy(bodies: &[Body]) -> isize {
    bodies.iter().map(Body::energy).sum()
}

fn parse_input(input: &str) -> Result<Vec<Body>> {
    input.trim().lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Result<Body> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();
    }
    let cap = RE
        .captures(line)
        .ok_or_else(|| format!("Invalid line \"{}\"", line))?;
    let x = parse_isize(&cap[1])?;
    let y = parse_isize(&cap[2])?;
    let z = parse_isize(&cap[3])?;
    let body = Body::new(x, y, z);
    Ok(body)
}

fn parse_isize(s: &str) -> Result<isize> {
    s.parse().map_err(|_| format!("Invalid value {}", s))
}

impl Body {
    pub fn new(pos_x: isize, pos_y: isize, pos_z: isize) -> Self {
        Body {
            pos: Vec3::new(pos_x, pos_y, pos_z),
            vel: Vec3::new(0, 0, 0),
        }
    }

    pub fn update_velocity(&mut self, other: Body) {
        self.vel.x += Body::change(self.pos.x, other.pos.x);
        self.vel.y += Body::change(self.pos.y, other.pos.y);
        self.vel.z += Body::change(self.pos.z, other.pos.z);
    }

    pub fn update_position(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
        self.pos.z += self.vel.z;
    }

    pub fn energy(&self) -> isize {
        self.potential_energy() * self.kinetic_energy()
    }

    fn potential_energy(&self) -> isize {
        self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs()
    }

    fn kinetic_energy(&self) -> isize {
        self.vel.x.abs() + self.vel.y.abs() + self.vel.z.abs()
    }

    fn change(source: isize, dest: isize) -> isize {
        use std::cmp::Ordering;
        match source.cmp(&dest) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        }
    }
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3 { x, y, z }
    }
}
