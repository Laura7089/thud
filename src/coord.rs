use crate::ThudError;

/// A checked container for a coordinate to address into a [`Board`](strucy.Board.html).
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn check_coords(x: usize, y: usize) -> Result<(), ThudError> {
        let sum = x + y;

        // Check flat bounds first
        let invalid = (x > 14)
            || (y > 14)
            // Extreme corners
            || (sum < 5)
            || (sum > 23)
            // Outer corners
            || (15 - x + y < 6)
            || (15 + x - y < 6);

        if invalid {
            Err(ThudError::InvalidPosition)
        } else {
            Ok(())
        }
    }

    pub fn zero_based(x: usize, y: usize) -> Result<Self, ThudError> {
        Coord::check_coords(x, y)?;
        Ok(Coord { x, y })
    }

    pub fn one_based(x: usize, y: usize) -> Result<Self, ThudError> {
        let (x, y) = (x - 1, y - 1);
        Self::zero_based(x, y)
    }

    /// Get the values inside the coordinate, zero-based
    pub fn value(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn max(&self) -> usize {
        if self.x > self.y {
            self.x
        } else {
            self.y
        }
    }
}

impl From<(usize, usize)> for Coord {
    fn from((x, y): (usize, usize)) -> Self {
        Coord::zero_based(x, y).expect(&*format!(
            "Tried to do a 'from' conversion using invalid coordinates x={}, y={}",
            x, y
        ))
    }
}

impl std::ops::Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let new_x = if self.x > rhs.x {
            self.x - rhs.x
        } else {
            rhs.x - self.x
        };
        let new_y = if self.y > rhs.y {
            self.y - rhs.y
        } else {
            rhs.y - self.y
        };

        Coord { x: new_x, y: new_y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(6, 9)]
    #[test_case(2, 3)]
    #[test_case(5, 0)]
    #[test_case(9, 0)]
    #[test_case(10, 2)]
    #[test_case(13, 5)]
    #[test_case(14, 6)]
    #[test_case(14, 8)]
    fn valid_coordinates(x: usize, y: usize) -> Coord {
        (x, y).into()
    }

    #[test_case(0, 0 => panics "no")]
    #[test_case(4, 0 => panics "no")]
    #[test_case(10, 0 => panics "no")]
    #[test_case(14, 4 => panics "no")]
    #[test_case(14, 10 => panics "no")]
    fn invalid_coordinates(x: usize, y: usize) {
        Coord::zero_based(x, y).expect("no");
    }

    #[test_case(0, 9)]
    #[test_case(2, 3)]
    #[test_case(5, 0)]
    #[test_case(9, 0)]
    #[test_case(10, 2)]
    #[test_case(13, 5)]
    #[test_case(14, 6)]
    fn into(x: usize, y: usize) {
        let _coord: Coord = (x, y).into();
    }

    #[test_case((7,7), (8,8) => 1)]
    #[test_case((8,8), (7,7) => 1)]
    #[test_case((7,8), (8,8) => 1)]
    #[test_case((7,7), (9,8) => 2)]
    #[test_case((7,7), (10,7) => 3)]
    #[test_case((8,7), (9,7) => 1)]
    fn subtract_then_max(lhs: (usize, usize), rhs: (usize, usize)) -> usize {
        (Coord::zero_based(lhs.0, lhs.1).unwrap() - Coord::zero_based(rhs.0, rhs.1).unwrap()).max()
    }
}
