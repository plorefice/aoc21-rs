use std::{
    cmp::{self, Reverse},
    collections::{BinaryHeap, HashSet},
    hash::{Hash, Hasher},
    mem::swap,
};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Map {
    rooms: [[Option<u8>; 2]; 4],
    hallway: [Option<u8>; 11],
}

impl Map {
    pub fn organized(&self) -> bool {
        self.rooms[0] == [Some(0), Some(0)]
            && self.rooms[1] == [Some(1), Some(1)]
            && self.rooms[2] == [Some(2), Some(2)]
            && self.rooms[3] == [Some(3), Some(3)]
    }

    pub fn available_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();

        // Moves from hallway to room
        for (i, amph) in self
            .hallway
            .iter()
            .enumerate()
            .filter_map(|(i, x)| x.map(|x| (i, x)))
        {
            let room = amph as usize;
            let room_x = (room + 1) * 2;

            let range = if i < room_x {
                i + 1..=room_x
            } else {
                room_x..=i - 1
            };

            // Can't move to a room if the hallway is occupied
            if self.hallway[range].iter().any(|x| x.is_some()) {
                continue;
            }

            if self.rooms[room] == [None, None] {
                moves.push(Move {
                    who: amph,
                    from: Spot::Hallway(i),
                    to: Spot::Room(room, 1),
                })
            }
            if self.rooms[room] == [None, Some(amph)] {
                moves.push(Move {
                    who: amph,
                    from: Spot::Hallway(i),
                    to: Spot::Room(room, 0),
                })
            }
        }

        // Moves from room to hallway
        for (i, &[front, back]) in self.rooms.iter().enumerate() {
            match (front, back) {
                (Some(a), Some(b)) if a == b && a == i as u8 => {
                    // Room is organized: do not scramble it
                }
                (Some(a), _) => {
                    for spot in [0, 1, 3, 5, 7, 9, 10] {
                        if self.hallway[spot].is_none()
                            && !self.hallway[spot.min((i + 1) * 2)..=spot.max((i + 1) * 2)]
                                .iter()
                                .any(|x| x.is_some())
                        {
                            moves.push(Move {
                                who: a,
                                from: Spot::Room(i, 0),
                                to: Spot::Hallway(spot),
                            })
                        }
                    }
                }
                (None, Some(b)) if b != i as u8 => {
                    for spot in [0, 1, 3, 5, 7, 9, 10] {
                        if self.hallway[spot].is_none()
                            && !self.hallway[spot.min((i + 1) * 2)..=spot.max((i + 1) * 2)]
                                .iter()
                                .any(|x| x.is_some())
                        {
                            moves.push(Move {
                                who: b,
                                from: Spot::Room(i, 1),
                                to: Spot::Hallway(spot),
                            })
                        }
                    }
                }
                (None, None) | (None, Some(_)) => (),
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
            [Some(front[3] - b'A'), Some(back[3] - b'A')],
            [Some(front[5] - b'A'), Some(back[5] - b'A')],
            [Some(front[7] - b'A'), Some(back[7] - b'A')],
            [Some(front[9] - b'A'), Some(back[9] - b'A')],
        ],
        hallway: [None; 11],
    }
}

pub fn part_1(map: Map) -> usize {
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
    }
}
