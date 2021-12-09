use crate::five_in_a_row::mv::FiveInRowMove;

#[derive(Debug)]
pub enum Direction {
    Row(i32),
    Column(i32),
    Diagonal(i32, i32),
    CrossDiagonal(i32, i32),
}

impl Direction {
    pub fn create_list_from_move(mv: &FiveInRowMove) -> [Direction; 4] {
        let dirs: [Direction; 4] = [
            Direction::Row(mv.get_y()),
            Direction::Column(mv.get_x()),
            Direction::Diagonal(mv.get_x(), mv.get_y()),
            Direction::CrossDiagonal(mv.get_x(), mv.get_y()),
        ];
        dirs
    }

    pub fn is_in_direction(&self, x: i32, y: i32) -> bool {
        match &self {
            Self::Row(py) => *py == y,
            Self::Column(px) => *px == x,
            Self::Diagonal(px, py) => *py - *px == y - x,
            Self::CrossDiagonal(px, py) => *py + *px == y + x,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::five_in_a_row::mv::FiveInRowMove;

    #[test]
    fn it_creates_iterator() {
        let mv = FiveInRowMove::Mine(0, 0);
        let i = Direction::create_list_from_move(&mv);
        assert_eq!(i.len(), 4);
    }

    #[test]
    fn it_check_row_direction() {
        assert_eq!(Direction::Row(3).is_in_direction(1, 2), false);
        assert_eq!(Direction::Row(3).is_in_direction(2, 3), true);
        assert_eq!(Direction::Row(3).is_in_direction(3, 4), false);
    }

    #[test]
    fn it_check_col_direction() {
        assert_eq!(Direction::Column(3).is_in_direction(1, 2), false);
        assert_eq!(Direction::Column(3).is_in_direction(2, 3), false);
        assert_eq!(Direction::Column(3).is_in_direction(3, 4), true);
    }

    #[test]
    fn it_check_rising_diagonal_direction() {
        assert_eq!(Direction::Diagonal(1, 3).is_in_direction(0, 2), true);
        assert_eq!(Direction::Diagonal(1, 3).is_in_direction(0, 3), false);
        assert_eq!(Direction::Diagonal(1, 3).is_in_direction(0, 4), false);
        assert_eq!(Direction::Diagonal(1, 3).is_in_direction(0, 5), false);
        assert_eq!(Direction::Diagonal(1, 3).is_in_direction(2, 2), false);
        assert_eq!(Direction::Diagonal(1, 3).is_in_direction(2, 3), false);
        assert_eq!(Direction::Diagonal(1, 3).is_in_direction(2, 4), true);
        assert_eq!(Direction::Diagonal(1, 3).is_in_direction(2, 5), false);
    }

    #[test]
    fn it_check_falling_diagonal_direction() {
        assert_eq!(Direction::CrossDiagonal(1, 3).is_in_direction(0, 2), false);
        assert_eq!(Direction::CrossDiagonal(1, 3).is_in_direction(0, 3), false);
        assert_eq!(Direction::CrossDiagonal(1, 3).is_in_direction(0, 4), true);
        assert_eq!(Direction::CrossDiagonal(1, 3).is_in_direction(0, 5), false);
        assert_eq!(Direction::CrossDiagonal(1, 3).is_in_direction(2, 2), true);
        assert_eq!(Direction::CrossDiagonal(1, 3).is_in_direction(2, 3), false);
        assert_eq!(Direction::CrossDiagonal(1, 3).is_in_direction(2, 4), false);
        assert_eq!(Direction::CrossDiagonal(1, 3).is_in_direction(2, 5), false);
    }
}
