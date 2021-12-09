use crate::api::jobs_cz::status::Coordinate;
use crate::game::GameMove;
use core::cmp::Ordering;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FiveInRowMove {
    Mine(i32, i32),
    Rivals(i32, i32),
}

impl FiveInRowMove {
    pub fn from_api_coordinates(current_user_id: &str, c: &Coordinate) -> Self {
        if c.playerId.eq(current_user_id) {
            FiveInRowMove::Mine(c.x, c.y)
        } else {
            FiveInRowMove::Rivals(c.x, c.y)
        }
    }

    pub fn get_x(&self) -> i32 {
        match self {
            FiveInRowMove::Mine(x, _) => *x,
            FiveInRowMove::Rivals(x, _) => *x,
        }
    }
    pub fn get_y(&self) -> i32 {
        match self {
            FiveInRowMove::Mine(_, y) => *y,
            FiveInRowMove::Rivals(_, y) => *y,
        }
    }
    pub fn is_same_type(&self, maybe_other: Option<&FiveInRowMove>) -> bool {
        if let Some(other) = maybe_other {
            GameMove::is_mine(self) == GameMove::is_mine(other)
        } else {
            false
        }
    }

    pub fn get_distance(&self, mv: &FiveInRowMove) -> i32 {
        let dif_x = -mv.get_x() + self.get_x();
        let dif_y = -mv.get_y() + self.get_y();
        if i32::abs(dif_x) > i32::abs(dif_y) {
            dif_x
        } else {
            dif_y
        }
    }

    pub fn get_distance_from_moves(&self, vec: &Vec<FiveInRowMove>) -> i32 {
        vec.iter().fold(i32::MAX, |ret, mv| {
            i32::min(ret, self.get_distance(mv).abs())
        })
    }
}

impl GameMove for FiveInRowMove {
    fn is_mine(&self) -> bool {
        match self {
            FiveInRowMove::Mine(_, _) => true,
            FiveInRowMove::Rivals(_, _) => false,
        }
    }
}

impl Eq for FiveInRowMove {}
impl Ord for FiveInRowMove {
    fn cmp(&self, other: &FiveInRowMove) -> Ordering {
        let dist = self.get_distance(other);
        if dist == 0 {
            Ordering::Equal
        } else if dist < 0 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}
impl PartialOrd for FiveInRowMove {
    fn partial_cmp(&self, other: &FiveInRowMove) -> Option<Ordering> {
        Some(Ord::cmp(&self, &other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_checks_type() {
        let mv = FiveInRowMove::Mine(1, 2);
        assert_eq!(mv.is_same_type(Some(&FiveInRowMove::Rivals(3, 4))), false);
        assert_eq!(mv.is_same_type(Some(&FiveInRowMove::Mine(3, 4))), true);
        assert_eq!(mv.is_same_type(None), false);

        assert_eq!(GameMove::is_mine(&mv), true);
        assert_eq!(GameMove::is_mine(&FiveInRowMove::Rivals(1, 2)), false);
    }

    #[test]
    fn it_computes_distance() {
        let mv = FiveInRowMove::Mine(1, 2);
        assert_eq!(mv.get_distance(&FiveInRowMove::Rivals(1, 2)), 0);
        assert_eq!(mv.get_distance(&FiveInRowMove::Rivals(2, 2)), -1);
        assert_eq!(mv.get_distance(&FiveInRowMove::Rivals(-2, -2)), 4);
        assert_eq!(FiveInRowMove::Rivals(-2, -2).get_distance(&mv), -4);
    }

    #[test]
    fn it_sorts_vec_of_moves() {
        let mut v: Vec<FiveInRowMove> = Vec::new();
        v.push(FiveInRowMove::Mine(1, 2));
        v.push(FiveInRowMove::Mine(1, 4));
        v.push(FiveInRowMove::Mine(1, 8));
        v.push(FiveInRowMove::Rivals(1, 3));
        v.push(FiveInRowMove::Rivals(1, 1));
        v.sort();
        assert_eq!(v.get(0).unwrap().get_y(), 1);
        assert_eq!(v.get(1).unwrap().get_y(), 2);
        assert_eq!(v.get(2).unwrap().get_y(), 3);
        assert_eq!(v.get(3).unwrap().get_y(), 4);
        assert_eq!(v.get(4).unwrap().get_y(), 8);
    }
}
