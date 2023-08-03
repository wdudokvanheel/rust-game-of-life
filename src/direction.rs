use rand::{Rng, thread_rng};
use strum::{EnumCount, FromRepr};

#[derive(FromRepr, Debug, PartialEq, EnumCount)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction{
    pub fn get_random_direction() -> Direction {
        let mut rng = thread_rng();
        return Direction::from_repr(rng.gen_range(0..Direction::COUNT))
            .unwrap_or(Direction::North);
    }
}
