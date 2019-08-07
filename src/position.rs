#[derive(Clone, Copy)]
pub struct Size {
    pub width: usize, pub height: usize
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Position {
    pub x: usize, pub y: usize
}

pub enum Direction {
    UpLeft, Up, UpRight, Right, DownRight, Down, DownLeft, Left
}

static ALL_DIRECTIONS: [Direction; 8] = [
    Direction::UpLeft,
    Direction::Up,
    Direction::UpRight,
    Direction::Right,
    Direction::DownRight,
    Direction::Down,
    Direction::DownLeft,
    Direction::Left
];

impl Direction {
    pub fn iter_all() -> impl Iterator<Item=&'static Direction> {
        ALL_DIRECTIONS.iter()
    }
}

impl Position {
    pub fn advance(&self, direction: &Direction, limits: &Size) -> Option<Position> {
        let horizontal_inc = |p: &Position| {
            if p.x + 1 > limits.width - 1 { return None; }
            Some(Position{x: p.x + 1, y: p.y})
        };

        let horizontal_dec = |p: &Position| {
            if p.x < 1 { return None; }
            Some(Position{x: p.x - 1, y: p.y})
        };
        
        let verticl_inc = |p: &Position| {
            if p.y + 1 > limits.height - 1 { return None; }
            Some(Position{x: p.x, y: p.y + 1})
        };

        let verticl_dec = |p: &Position| {
            if p.y < 1 { return None; }
            Some(Position{x: p.x, y: p.y - 1})
        };
        
        match direction {
            Direction::UpLeft => verticl_dec(self).as_ref().and_then(horizontal_dec),
            Direction::Up => verticl_dec(self),
            Direction::UpRight => verticl_dec(self).as_ref().and_then(horizontal_inc),
            Direction::Right => horizontal_inc(self),
            Direction::DownRight => verticl_inc(self).as_ref().and_then(horizontal_inc),
            Direction::Down => verticl_inc(self),
            Direction::DownLeft => verticl_inc(self).as_ref().and_then(horizontal_dec),
            Direction::Left => horizontal_dec(self)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Position, Direction, Size};

    #[test]
    fn position_add_test() {
        assert_eq!(Position{x: 1, y: 1}.advance(&Direction::Left, &Size{width: 6, height: 6}), Some(Position{x: 0, y: 1}));
        assert_eq!(Position{x: 1, y: 5}.advance(&Direction::UpLeft, &Size{width: 6, height: 6}), Some(Position{x: 0, y: 4}));
        assert_eq!(Position{x: 2, y: 1}.advance(&Direction::DownLeft, &Size{width: 6, height: 6}), Some(Position{x: 1, y: 2}));
        assert_eq!(Position{x: 1, y: 1}.advance(&Direction::UpRight, &Size{width: 6, height: 6}), Some(Position{x: 2, y: 0}));
        assert_eq!(Position{x: 1, y: 0}.advance(&Direction::Left, &Size{width: 6, height: 6}), Some(Position{x: 0, y: 0}));
        assert_eq!(Position{x: 0, y: 1}.advance(&Direction::Up, &Size{width: 6, height: 6}), Some(Position{x: 0, y: 0}));
    }

    #[test]
    fn position_add_test_underflow() {
        assert_eq!(Position{x: 0, y: 1}.advance(&Direction::Left, &Size{width: 4, height: 6}), None);
        assert_eq!(Position{x: 0, y: 0}.advance(&Direction::DownLeft, &Size{width: 4, height: 6}), None);
        assert_eq!(Position{x: 1, y: 0}.advance(&Direction::UpRight, &Size{width: 4, height: 6}), None);
        assert_eq!(Position{x: 0, y: 1}.advance(&Direction::DownLeft, &Size{width: 4, height: 6}), None);
        assert_eq!(Position{x: 0, y: 0}.advance(&Direction::UpLeft, &Size{width: 4, height: 6}), None);
    }

    #[test]
    fn position_add_test_overflow() {
        assert_eq!(Position{x: 3, y: 1}.advance(&Direction::Right, &Size{width: 4, height: 6}), None);
        assert_eq!(Position{x: 3, y: 5}.advance(&Direction::Down, &Size{width: 4, height: 6}), None);
        assert_eq!(Position{x: 0, y: 3}.advance(&Direction::DownLeft, &Size{width: 4, height: 4}), None);
    }
}
