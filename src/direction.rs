use crate::coord::Coord;

pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    pub fn modifier(&self) -> (isize, isize) {
        match self {
            Self::Up => (0, 1),
            Self::UpRight => (1, 1),
            Self::Right => (1, 0),
            Self::DownRight => (1, -1),
            Self::Down => (0, -1),
            Self::DownLeft => (-1, -1),
            Self::Left => (-1, 0),
            Self::UpLeft => (-1, 1),
        }
    }

    pub fn obtain(start: Coord, end: Coord) -> Result<Self, &'static str> {
        let (src, dest) = (start.value(), end.value());
        let diff = start.diff(end).value();

        if src.0 == dest.0 {
            if src.1 < dest.1 {
                Ok(Direction::Up)
            } else {
                Ok(Direction::Down)
            }
        } else if src.1 == dest.1 {
            if src.0 < dest.0 {
                Ok(Direction::Right)
            } else {
                Ok(Direction::Left)
            }
        } else if diff.0 == diff.1 {
            if src.0 < dest.0 {
                if src.1 < dest.1 {
                    Ok(Direction::UpRight)
                } else {
                    Ok(Direction::UpLeft)
                }
            } else {
                if src.1 < dest.1 {
                    Ok(Direction::DownRight)
                } else {
                    Ok(Direction::DownLeft)
                }
            }
        } else {
            Err("No straight path available")
        }
    }
}
