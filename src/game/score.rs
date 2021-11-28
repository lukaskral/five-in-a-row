use core::cmp::Ordering;
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy)]
pub enum Score {
    Numeric(f64),
    Win,
    Loss,
}

impl Score {
    fn max<'a>(one: &'a Score, other: &'a Score) -> &'a Score {
        if *one > *other {
            one
        } else {
            other
        }
    }
    fn min<'a>(one: &'a Score, other: &'a Score) -> &'a Score {
        if *one < *other {
            one
        } else {
            other
        }
    }
}

impl Eq for Score {}
impl Ord for Score {
    fn cmp(&self, other: &Score) -> Ordering {
        match self {
            Score::Win => match *other {
                Score::Win => Ordering::Equal,
                _ => Ordering::Greater,
            },
            Score::Numeric(score) => match *other {
                Score::Win => Ordering::Less,
                Score::Numeric(other_score) => {
                    let dif = score - other_score;
                    if dif < -0.01 {
                        Ordering::Less
                    } else if dif > 0.01 {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                }
                Score::Loss => Ordering::Greater,
            },
            Score::Loss => match *other {
                Score::Loss => Ordering::Equal,
                _ => Ordering::Less,
            },
        }
    }
}
impl PartialEq for Score {
    fn eq(&self, other: &Score) -> bool {
        Ord::cmp(self, other) == Ordering::Equal
    }
}
impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Score) -> Option<Ordering> {
        Some(Ord::cmp(&self, &other))
    }
}

impl Add for Score {
    type Output = Score;
    fn add(self, other: Score) -> Score {
        match self {
            Score::Win => {
                if other == Score::Loss {
                    Score::Numeric(0.0)
                } else {
                    Score::Win
                }
            }
            Score::Numeric(score) => match other {
                Score::Win => Score::Win,
                Score::Numeric(other_score) => Score::Numeric(score + other_score),
                Score::Loss => Score::Loss,
            },
            Score::Loss => {
                if other == Score::Win {
                    Score::Numeric(0.0)
                } else {
                    Score::Loss
                }
            }
        }
    }
}

impl Add<f64> for Score {
    type Output = Score;
    fn add(self, other: f64) -> Score {
        match self {
            Score::Win => Score::Win,
            Score::Numeric(score) => Score::Numeric(score + other),
            Score::Loss => Score::Loss,
        }
    }
}

impl Mul<f64> for Score {
    type Output = Score;
    fn mul(self, other: f64) -> Score {
        match self {
            Score::Win => Score::Win,
            Score::Numeric(score) => Score::Numeric(score * other),
            Score::Loss => Score::Loss,
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
