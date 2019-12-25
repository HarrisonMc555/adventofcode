use crate::util::math;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashSet;

const INPUT: &str = include_str!("../../static/day12.txt");
const DEFAULT_NUM_STEPS: usize = 1000;

type Result<T> = std::result::Result<T, String>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Body {
    pub pos: Vec3<isize>,
    pub vel: Vec3<isize>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct PosVel {
    pub pos: isize,
    pub vel: isize,
}

pub fn main() {
    let answer1 = solve1(INPUT, DEFAULT_NUM_STEPS);
    println!("{:?}", answer1);
    let answer2 = solve2(INPUT);
    println!("{:?}", answer2);
}

fn solve1(input: &str, num_steps: usize) -> Result<isize> {
    let mut bodies = parse_input(input)?;
    for _ in 0..num_steps {
        update_bodies(&mut bodies);
    }
    let energy = calc_energy(&bodies);
    Ok(energy)
}

fn solve2(input: &str) -> Result<usize> {
    let bodies = parse_input(input)?;
    let (mut xs, mut ys, mut zs) = extract_parts(&bodies);
    let x_period = period(&mut xs);
    let y_period = period(&mut ys);
    let z_period = period(&mut zs);
    let xy_period = math::lcm(x_period, y_period);
    let xyz_period = math::lcm(xy_period, z_period);
    Ok(xyz_period)
}

fn period(posvels: &mut [PosVel]) -> usize {
    let mut seen = HashSet::<Vec<PosVel>>::new();
    while !seen.contains(posvels) {
        seen.insert(posvels.to_vec());
        update_posvels(posvels);
    }
    seen.len()
}

fn update_posvels(posvels: &mut [PosVel]) {
    update_posvels_velocities(posvels);
    update_posvels_positions(posvels);
}

fn update_posvels_velocities(posvels: &mut [PosVel]) {
    for i in 0..posvels.len() {
        for j in (0..posvels.len()).filter(|&j| j != i) {
            let other = posvels[j];
            let posvel = &mut posvels[i];
            posvel.update_velocity(other);
        }
    }
}

fn update_posvels_positions(posvels: &mut [PosVel]) {
    for posvel in posvels.iter_mut() {
        posvel.update_position();
    }
}

fn extract_parts(bodies: &[Body]) -> (Vec<PosVel>, Vec<PosVel>, Vec<PosVel>) {
    let xs = bodies
        .iter()
        .map(|body| PosVel::new(body.pos.x, 0))
        .collect();
    let ys = bodies
        .iter()
        .map(|body| PosVel::new(body.pos.y, 0))
        .collect();
    let zs = bodies
        .iter()
        .map(|body| PosVel::new(body.pos.z, 0))
        .collect();
    (xs, ys, zs)
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

impl PosVel {
    pub fn new(pos: isize, vel: isize) -> Self {
        PosVel { pos, vel }
    }

    pub fn update_velocity(&mut self, other: PosVel) {
        let change = match self.pos.cmp(&other.pos) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };
        self.vel += change;
    }

    pub fn update_position(&mut self) {
        self.pos += self.vel;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn answer1() {
        assert_eq!(solve1(INPUT, DEFAULT_NUM_STEPS), Ok(8625));
    }

    #[test]
    fn answer2() {
        assert_eq!(solve2(INPUT), Ok(332477126821644));
    }
}
