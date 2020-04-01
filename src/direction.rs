use crate::coord::Coord;
use crate::ThudError;

/// Represents a cardinal direction on a `Board`
#[derive(Debug, PartialEq, Copy, Clone)]
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
    /// Get a `Vec` containing all the different possible `Direction`s
    pub fn all() -> Vec<Self> {
        use Direction::*;
        vec![Up, UpRight, Right, DownRight, Down, DownLeft, Left, UpLeft]
    }

    /// Use two `Coord`s to get a `Direction` from one to the other
    ///
    /// Returns `Err(ThudError::MathError)` if:
    ///
    /// - The two `Coord`s are not plottable on a straight line together
    /// - The two `Coord`s are equal
    pub fn from_route(start: Coord, end: Coord) -> Result<Direction, ThudError> {
        if start == end {
            return Err(ThudError::MathError);
        }

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
            Err(ThudError::MathError)
        }
    }

    /// Get a tuple representing the changes needed to a `Coord` in order to "move" it in the
    /// `Direction` given by `self`.
    ///
    /// Example:
    /// ```
    /// use thud::Direction;
    ///
    /// assert_eq!(Direction::Up.modifier(), (0, 1));
    /// assert_eq!(Direction::DownLeft.modifier(), (-1, -1));
    /// ```
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

    /// Return a `Coord` equal to `loc` offset by one square in the direction given by `self`.
    ///
    /// Returns `Err(ThudError::MathError)` if this movement would place the `Coord` out of bounds.
    pub fn modify(&self, loc: Coord) -> Result<Coord, ThudError> {
        let modifier = self.modifier();
        let (x, y) = loc.value();
        let new = (
            match modifier.0 {
                1 => x + 1,
                -1 => x - 1,
                _ => x,
            },
            match modifier.1 {
                1 => y + 1,
                -1 => y - 1,
                _ => y,
            },
        );

        if let Ok(coord) = Coord::zero_based(new.0, new.1) {
            Ok(coord)
        } else {
            Err(ThudError::MathError)
        }
    }

    /// Get the "opposite" `Direction` to `self`.
    ///
    /// For example:
    /// ```
    /// use thud::Direction;
    ///
    /// assert_eq!(Direction::Up.opposite(), Direction::Down);
    /// ```
    pub fn opposite(&self) -> Self {
        use Direction::*;
        match self {
            Up => Down,
            UpRight => DownLeft,
            Right => Left,
            DownRight => UpLeft,
            Down => Up,
            DownLeft => UpRight,
            Left => Right,
            UpLeft => DownRight,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Direction::*;
    use super::*;
    use test_case::test_case;

    #[test_case((7,7), (8,8) => Direction::UpRight)]
    #[test_case((8,7), (0,7) => Direction::Left)]
    #[test_case((7,7), (7,7) => panics "")]
    #[test_case((7,7), (8,9) => panics "")]
    fn from_route(src: (usize, usize), dest: (usize, usize)) -> Direction {
        Direction::from_route(src.into(), dest.into()).expect("")
    }

    #[test_case(UpLeft, (10, 10) => (9, 11))]
    #[test_case(Down, (7, 3) => (7, 2))]
    #[test_case(Down, (7, 0) => panics "")]
    #[test_case(UpRight, (14, 7) => panics "")]
    fn modify(dir: Direction, loc: (usize, usize)) -> (usize, usize) {
        dir.modify(loc.into()).expect("").value()
    }
}
