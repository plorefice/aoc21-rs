use std::ops::RangeInclusive;

pub type Target = (RangeInclusive<i32>, RangeInclusive<i32>);

pub fn parse_input(input: &str) -> Target {
    let (x, y) = input.split_once(", ").unwrap();

    let (xs, xe) = x[15..].split_once("..").unwrap();
    let (ys, ye) = y[2..].split_once("..").unwrap();

    (
        xs.parse().unwrap()..=xe.parse().unwrap(),
        ys.parse().unwrap()..=ye.parse().unwrap(),
    )
}

pub fn part_1(tgt: Target) -> i32 {
    solve(tgt).0
}

pub fn part_2(tgt: Target) -> u32 {
    solve(tgt).1
}

pub fn solve(tgt: Target) -> (i32, u32) {
    let mut valid_solutions = 0;
    let mut y_max = 0;

    // The lower x velocity bound is the first triangular number such that we will hit the target
    // after a certain number of steps. Initial x velocities lower that this number will always
    // fall short of the target.
    let xmin = (f32::sqrt(2. * (*tgt.0.start()) as f32 + 0.25) - 0.5).ceil() as i32;

    // The upper bound is trivial: a velocity too high will miss the target entirely, so we limit
    // it to the furthest target coordinate.
    for vx in xmin..=*tgt.0.end() {
        // The lower bound for y is trivial: if we shoot straight forward with an x velocity equal
        // in magnitude to a point inside the target, all negative y velocities with the same
        // condition will cause us to land inside the target after a single step.
        //
        // The upper bound is trickier: since the rise and fall of the projectile for a positive y
        // velocity are symmetrical, we will always hit y=0 on the descent path after a given
        // number of steps. This implies that a y velocity higher in absolute magnitude than the
        // lower edge of the target will always end up in a miss.
        for vy in *tgt.1.start()..=-tgt.1.start() {
            let mut pos = (0, 0);
            let mut vel = (vx, vy);

            while pos.1 > *tgt.1.start() {
                step(&mut pos, &mut vel);

                if in_target(pos, &tgt) {
                    valid_solutions += 1;
                    y_max = y_max.max(vy * (vy + 1) / 2);
                    break;
                }
            }
        }
    }

    (y_max, valid_solutions)
}

fn step(pos: &mut (i32, i32), vel: &mut (i32, i32)) {
    pos.0 += vel.0;
    pos.1 += vel.1;

    vel.1 -= 1;
    match vel.0 {
        1..=i32::MAX => vel.0 -= 1,
        i32::MIN..=-1 => vel.0 += 1,
        0 => (),
    }
}

fn in_target(pos: (i32, i32), target: &Target) -> bool {
    target.0.contains(&pos.0) && target.1.contains(&pos.1)
}

crate::solutions! {
    p1 => {
        part_1(parse_input(include_str!("../inputs/day17.txt"))),
        4186
    },
    p2 => {
        part_2(parse_input(include_str!("../inputs/day17.txt"))),
        2709
    }
}
