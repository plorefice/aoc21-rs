use std::{
    cmp::{self, Reverse},
    collections::{BinaryHeap, HashSet},
    hash::{Hash, Hasher},
    mem::swap,
};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Spot {
    Hallway(usize),
    Room(usize, usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    who: u8,
    from: Spot,
    to: Spot,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Map {
    rooms: [Vec<Option<u8>>; 4],
    hallway: [Option<u8>; 11],
}

impl Map {
    pub fn organized(&self) -> bool {
        self.rooms
            .iter()
            .enumerate()
            .all(|(amph, room)| room.iter().all(|&spot| spot == Some(amph as u8)))
    }

    pub fn available_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();

        // Moves from hallway to room
        for (hall, amph) in self
            .hallway
            .iter()
            .enumerate()
            .filter_map(|(i, x)| x.map(|x| (i, x)))
        {
            let room = &self.rooms[amph as usize];
            let room_x = (amph as usize + 1) * 2;

            let range = if hall < room_x {
                hall + 1..=room_x
            } else {
                room_x..=hall - 1
            };

            // Can't move to a room if the hallway is occupied
            if self.hallway[range].iter().any(|x| x.is_some()) {
                continue;
            }

            // Find a free spot
            if let Some((spot, _)) = room.iter().rev().find_position(|&&r| r == None) {
                let spot = room.len() - spot - 1;

                // Do not move into a room unless the rest of the room is already organized
                if room[spot + 1..].iter().all(|&r| r == Some(amph)) {
                    moves.push(Move {
                        who: amph,
                        from: Spot::Hallway(hall),
                        to: Spot::Room(amph as usize, spot),
                    })
                }
            }
        }

        // Moves from room to hallway
        for (amph, room) in self.rooms.iter().enumerate() {
            if room.iter().all(|&r| r == Some(amph as u8)) {
                continue;
            }

            // Find the first occupied spot
            if let Some((place, Some(other))) = room.iter().find_position(|r| r.is_some()) {
                // Do not mess up a room if the rest of the room is organized
                if !room[place..].iter().all(|&r| r == Some(amph as u8)) {
                    for spot in [0, 1, 3, 5, 7, 9, 10] {
                        if self.hallway[spot].is_none()
                            && !self.hallway[spot.min((amph + 1) * 2)..=spot.max((amph + 1) * 2)]
                                .iter()
                                .any(|x| x.is_some())
                        {
                            moves.push(Move {
                                who: *other,
                                from: Spot::Room(amph, place),
                                to: Spot::Hallway(spot),
                            })
                        }
                    }
                }
            }
        }

        moves
    }
}

#[derive(Debug, Clone)]
pub struct Configuration {
    map: Map,
    score: usize,
}

impl PartialEq for Configuration {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for Configuration {}

impl PartialOrd for Configuration {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl Ord for Configuration {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl Hash for Configuration {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.map.hash(state);
        self.score.hash(state);
    }
}

pub fn parse_input(input: &str) -> Map {
    let mut lines = input.lines().skip(2);

    let front = lines.next().unwrap().as_bytes();
    let back = lines.next().unwrap().as_bytes();

    Map {
        rooms: [
            vec![Some(front[3] - b'A'), Some(back[3] - b'A')],
            vec![Some(front[5] - b'A'), Some(back[5] - b'A')],
            vec![Some(front[7] - b'A'), Some(back[7] - b'A')],
            vec![Some(front[9] - b'A'), Some(back[9] - b'A')],
        ],
        hallway: [None; 11],
    }
}

pub fn part_1(map: Map) -> usize {
    solve(map)
}

pub fn part_2(mut map: Map) -> usize {
    map.rooms[0].insert(1, Some(3));
    map.rooms[0].insert(2, Some(3));
    map.rooms[1].insert(1, Some(2));
    map.rooms[1].insert(2, Some(1));
    map.rooms[2].insert(1, Some(1));
    map.rooms[2].insert(2, Some(0));
    map.rooms[3].insert(1, Some(0));
    map.rooms[3].insert(2, Some(2));

    solve(map)
}

fn solve(map: Map) -> usize {
    let mut configurations = BinaryHeap::new();
    let mut seen = HashSet::new();

    configurations.push(Reverse(Configuration { map, score: 0 }));

    while let Some(Reverse(cfg)) = configurations.pop() {
        if cfg.map.organized() {
            return cfg.score;
        }

        for Move { who, from, to } in cfg.map.available_moves() {
            let mut cfg = cfg.clone();

            match (from, to) {
                (Spot::Hallway(h), Spot::Room(r, i)) | (Spot::Room(r, i), Spot::Hallway(h)) => {
                    swap(&mut cfg.map.hallway[h], &mut cfg.map.rooms[r][i]);

                    let room_x = (r + 1) * 2;
                    cfg.score += (room_x.max(h) - room_x.min(h) + i + 1) * 10_usize.pow(who as u32);
                }
                _ => unreachable!(),
            };

            if seen.insert(cfg.clone()) {
                configurations.push(Reverse(cfg));
            }
        }
    }

    unreachable!()
}

crate::solutions! {
    p1 => {
        part_1(parse_input(include_str!("../inputs/day23.txt"))),
        18051
    },
    p2 => {
        part_2(parse_input(include_str!("../inputs/day23.txt"))),
        50245
    }
}
