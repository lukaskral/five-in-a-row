use core::cmp::Ordering;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub enum Score {
    Numeric(f64),
    Win,
    Loss,
}

impl Score {
    fn max<'a>(one: &'a Self, other: &'a Self) -> &'a Self {
        if *one > *other {
            one
        } else {
            other
        }
    }
    fn min<'a>(one: &'a Self, other: &'a Self) -> &'a Self {
        if *one < *other {
            one
        } else {
            other
        }
    }
    pub fn abs(&self) -> Self {
        if self > &Self::Numeric(0.0) {
            *self
        } else {
            *self * -1.0
        }
    }
}

impl Eq for Score {}
impl Ord for Score {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Self::Win => match *other {
                Self::Win => Ordering::Equal,
                _ => Ordering::Greater,
            },
            Self::Numeric(score) => match *other {
                Self::Win => Ordering::Less,
                Self::Numeric(other_score) => {
                    let dif = score - other_score;
                    if dif < -0.01 {
                        Ordering::Less
                    } else if dif > 0.01 {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                }
                Self::Loss => Ordering::Greater,
            },
            Self::Loss => match *other {
                Self::Loss => Ordering::Equal,
                _ => Ordering::Less,
            },
        }
    }
}
impl PartialEq for Score {
    fn eq(&self, other: &Self) -> bool {
        Ord::cmp(self, other) == Ordering::Equal
    }
}
impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Score) -> Option<Ordering> {
        Some(Ord::cmp(&self, &other))
    }
}

impl Add<Score> for Score {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        match self {
            Self::Win => {
                if other == Self::Loss {
                    Self::Numeric(0.0)
                } else {
                    Self::Win
                }
            }
            Self::Numeric(score) => match other {
                Self::Win => Self::Win,
                Self::Numeric(other_score) => Self::Numeric(score + other_score),
                Self::Loss => Self::Loss,
            },
            Self::Loss => {
                if other == Self::Win {
                    Self::Numeric(0.0)
                } else {
                    Self::Loss
                }
            }
        }
    }
}

impl Sub<Score> for Score {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        self + (other * -1.0)
    }
}

impl Add<f64> for Score {
    type Output = Self;
    fn add(self, other: f64) -> Self {
        match self {
            Self::Win => Self::Win,
            Self::Numeric(score) => Self::Numeric(score + other),
            Self::Loss => Self::Loss,
        }
    }
}

impl Sub<f64> for Score {
    type Output = Self;
    fn sub(self, other: f64) -> Self {
        match self {
            Self::Win => Self::Win,
            Self::Numeric(score) => Self::Numeric(score - other),
            Self::Loss => Self::Loss,
        }
    }
}

impl Mul<f64> for Score {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        match self {
            Self::Win => {
                if other > 0.0 {
                    Self::Win
                } else {
                    Self::Loss
                }
            }
            Self::Numeric(score) => Self::Numeric(score * other),
            Self::Loss => {
                if other > 0.0 {
                    Self::Win
                } else {
                    Self::Loss
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_is_comparable() {
        assert!(Score::Win == Score::Win);
        assert!(Score::Win > Score::Numeric(1.0));
        assert!(Score::Win > Score::Loss);

        assert!(Score::Numeric(1.0) < Score::Win);
        assert!(Score::Numeric(1.0) == Score::Numeric(1.0));
        assert!(Score::Numeric(1.0) > Score::Loss);

        assert!(Score::Numeric(1.0) < Score::Numeric(2.0));
        assert!(Score::Numeric(1.0) == Score::Numeric(1.0));
        assert!(Score::Numeric(1.0) > Score::Numeric(0.0));

        assert!(Score::Numeric(1.009) == Score::Numeric(1.0));
        assert!(Score::Numeric(1.0) == Score::Numeric(1.0));
        assert!(Score::Numeric(0.991) == Score::Numeric(1.0));

        assert!(Score::Loss < Score::Win);
        assert!(Score::Loss < Score::Numeric(1.0));
        assert!(Score::Loss == Score::Loss);
    }

    #[test]
    fn it_returns_greater_value() {
        assert_eq!(Score::max(&Score::Win, &Score::Loss), &Score::Win);
        assert_eq!(Score::max(&Score::Win, &Score::Numeric(7.0)), &Score::Win);
        assert_eq!(
            Score::max(&Score::Loss, &Score::Numeric(7.0)),
            &Score::Numeric(7.0)
        );
        assert_eq!(
            Score::max(&Score::Numeric(9.0), &Score::Numeric(7.0)),
            &Score::Numeric(9.0)
        );
    }
}
