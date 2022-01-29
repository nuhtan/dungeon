use std::ops::Range;

use super::Direction;

const PADDING: u32 = 5;

///
pub struct Floor {
    /// A list of all the rooms that the floor contains.
    rooms: Vec<Room>,
    /// A list of all hallways that connect the rooms.
    // hallways: Vec<Hallway>,
    /// The abosolute dimensions of the floor, should be significantly larger than the dimensions of any possible rooms.
    dimensions: (u32, u32),
}

struct Hallway {
    rel_loc: (u32, u32),
    length: u32,
    direction: Direction,
}

#[derive(Clone, Copy)]
struct Room {
    /// The bottom left hand corner of the room
    rel_loc: (u32, u32),
    /// The x y dimensions of the room
    dimensions: (u32, u32),
    /// How many hallways connect to the room
    connections: u32,
}

impl Floor {
    fn room_intersects_room(
        room_loc: (u32, u32),
        room_dims: (u32, u32),
        rooms: &Vec<Room>,
    ) -> bool {
        rooms.iter().any(|other: &Room| {
            (room_loc.0 >= other.rel_loc.0 - 1
                && room_loc.0 <= other.rel_loc.0 + other.dimensions.0 + 1
                || room_loc.0 + room_dims.0 >= other.rel_loc.0 - 1
                    && room_loc.0 + room_dims.0 <= other.rel_loc.0 + other.dimensions.0 + 1)
                && (room_loc.1 >= other.rel_loc.1 - 1
                    && room_loc.1 <= other.rel_loc.1 + other.dimensions.1 + 1
                    || room_loc.1 + room_dims.1 >= other.rel_loc.1 - 1
                        && room_loc.1 + room_dims.1 <= other.rel_loc.1 + other.dimensions.1 + 1)
        })
    }

    pub fn point_in_room(&self, location: (u32, u32)) -> bool {
        self.rooms.iter().any(|room| {
            (room.rel_loc.0..room.rel_loc.0 + room.dimensions.0).contains(&location.0)
                && (room.rel_loc.1..room.rel_loc.1 + room.dimensions.1).contains(&location.1)
        })
    }

    fn hallway_intersects_room(
        starting_point: (u32, u32),
        direction: Direction,
        length: u32,
        rooms: Vec<Room>,
    ) -> bool {
        rooms.iter().any(|room| {
            // true if the starting point is within the bounds of one of both axis, this should never occur i think
            ((starting_point.0 <= room.rel_loc.0 && starting_point.0 <= room.rel_loc.0 + room.dimensions.0) && (starting_point.1 <= room.rel_loc.1 && starting_point.1 <= room.rel_loc.1 + room.dimensions.1)) ||
            // true if the starting point is outside of the bounds but the length causes it to intersect, one dimension is within the bounds
            (
                // inside horizontal bounds
                (starting_point.0 <= room.rel_loc.0 && starting_point.0 <= room.rel_loc.0 + room.dimensions.0) && (
                    // check if below
                    direction == Direction::Down && starting_point.1 + length >= room.rel_loc.1 ||
                    // check if above
                    direction == Direction::Up && starting_point.1 - length <= room.rel_loc.1 + room.dimensions.1
                ) ||
                // inside vertical bounds
                (starting_point.1 <= room.rel_loc.1 && starting_point.1 <= room.rel_loc.1 + room.dimensions.1) && (
                    // check if to the right
                    direction == Direction::Right && starting_point.0 + length >= room.rel_loc.0 ||
                    // check if to the left
                    direction == Direction::Left && starting_point.0 - length <= room.rel_loc.0 - room.dimensions.0
                )
            )
        })
    }

    ///
    pub fn gen_floor(
        floor_dims: (u32, u32),
        room_count_range: Range<u32>,
        room_dims_range: Range<u32>,
    ) -> Floor {
        let mut rooms = Vec::new();
        let room_count = fastrand::u32(room_count_range);
        for _ in 0..room_count {
            let room_dims = (
                fastrand::u32(room_dims_range.clone()),
                fastrand::u32(room_dims_range.clone()),
            );
            let mut placed = false;
            let mut tries = 0;
            while !placed {
                let pos_loc = (
                    fastrand::u32(0..floor_dims.0 - room_dims.0 - PADDING),
                    fastrand::u32(0..floor_dims.1 - room_dims.1 - PADDING),
                );
                let intersect = Floor::room_intersects_room(pos_loc, room_dims, &rooms);
                if !intersect {
                    rooms.push(Room {
                        dimensions: room_dims,
                        rel_loc: pos_loc,
                        connections: 0,
                    });
                    placed = true;
                }
                tries += 1;

                if tries == 1000 {
                    break;
                }
            }
        }

        for room in &rooms {
            for _ in 0..fastrand::u32(0..4) {
                let target_room = rooms.get(fastrand::usize(..rooms.len())).unwrap();
                let dir_start = match fastrand::u32(0..4) {
                    0 => Direction::Up,
                    1 => Direction::Down,
                    2 => Direction::Right,
                    3 => Direction::Left,
                    _ => unreachable!("error in rng generation for direction"),
                };
                let dir_target = match fastrand::u32(0..4) {
                    0 => Direction::Up,
                    1 => Direction::Down,
                    2 => Direction::Right,
                    3 => Direction::Left,
                    _ => unreachable!("error in rng generation for direction"),
                };
                let start_point = match dir_start {
                    Direction::Up => (fastrand::u32(room.rel_loc.0..room.rel_loc.0 + room.dimensions.0), room.rel_loc.1 + room.dimensions.1 + 1),
                    Direction::Down => (fastrand::u32(room.rel_loc.0..room.rel_loc.0 + room.dimensions.0), room.rel_loc.1 - 1),
                    Direction::Left => (room.rel_loc.0 - 1, fastrand::u32(room.rel_loc.1..room.rel_loc.1 + room.dimensions.1)),
                    Direction::Right => (room.rel_loc.0 + room.dimensions.0 + 1, fastrand::u32(room.rel_loc.1..room.rel_loc.1 + room.dimensions.1)),
                    _ => unreachable!("naw")
                };

                let end_point = match dir_start {
                    Direction::Up => (fastrand::u32(target_room.rel_loc.0..target_room.rel_loc.0 + target_room.dimensions.0), target_room.rel_loc.1 + target_room.dimensions.1 + 1),
                    Direction::Down => (fastrand::u32(target_room.rel_loc.0..target_room.rel_loc.0 + target_room.dimensions.0), target_room.rel_loc.1 - 1),
                    Direction::Left => (target_room.rel_loc.0 - 1, fastrand::u32(target_room.rel_loc.1..target_room.rel_loc.1 + target_room.dimensions.1)),
                    Direction::Right => (target_room.rel_loc.0 + target_room.dimensions.0 + 1, fastrand::u32(target_room.rel_loc.1..target_room.rel_loc.1 + target_room.dimensions.1)),
                    _ => unreachable!("naw")
                };
            }
        }

        return Floor {
            rooms,
            dimensions: floor_dims,
        };
    }
}
