use std::ops::{Add, Div, Mul};

use Expr::*;

#[derive(Debug, Clone)]
pub enum Expr {
    Sum {
        one: f64,
        zero: f64,
        nonlinear: Vec<Expr>,
    },
    Min {
        a: Box<Expr>,
        b: Box<Expr>,
    },
}

impl Expr {
    pub const fn constant(a: f64) -> Expr {
        Sum {
            one: 0.0,
            zero: a,
            nonlinear: Vec::new(),
        }
    }

    pub const fn self_consistent(x: f64) -> Expr {
        Sum {
            one: x,
            zero: 0.0,
            nonlinear: Vec::new(),
        }
    }

    pub fn min(self, other: Expr) -> Expr {
        Min {
            a: Box::new(self),
            b: Box::new(other),
        }
    }

    pub fn eval(&self, x: f64) -> f64 {
        match self {
            Sum {
                one,
                zero,
                nonlinear,
            } => *one * x + *zero + nonlinear.iter().map(|e| e.eval(x)).sum::<f64>(),
            Min { a, b } => {
                let a = a.eval(x);
                let b = b.eval(x);
                if a <= b {
                    a
                } else {
                    b
                }
            }
        }
    }

    pub fn bisect(&self) -> f64 {
        let mut lo = 0.5;
        while self.eval(lo) < lo {
            lo /= 2.0
        }
        let mut hi = 1.0;
        while hi < self.eval(hi) {
            hi *= 2.0;
        }
        while hi - lo > 1e-12 {
            let mid = (hi + lo) / 2.0;
            let v = self.eval(mid);
            if v < mid {
                hi = mid;
            } else {
                lo = mid;
            }
        }
        hi
    }
}

impl Add<Expr> for Expr {
    type Output = Expr;
    fn add(self, rhs: Expr) -> Self::Output {
        match self {
            Sum {
                one,
                zero,
                mut nonlinear,
            } => match rhs {
                Sum {
                    one: r_one,
                    zero: r_zero,
                    nonlinear: mut r_nonlinear,
                } => {
                    nonlinear.append(&mut r_nonlinear);
                    Sum {
                        one: one + r_one,
                        zero: zero + r_zero,
                        nonlinear,
                    }
                }
                e => {
                    nonlinear.push(e);
                    Sum {
                        one,
                        zero,
                        nonlinear,
                    }
                }
            },
            e => match rhs {
                Sum {
                    one,
                    zero,
                    mut nonlinear,
                } => {
                    nonlinear.push(e);
                    Sum {
                        one,
                        zero,
                        nonlinear,
                    }
                }
                ee => Sum {
                    one: 0.0,
                    zero: 0.0,
                    nonlinear: vec![e, ee],
                },
            },
        }
    }
}

impl Mul<f64> for Expr {
    type Output = Expr;
    fn mul(self, rhs: f64) -> Expr {
        match self {
            Sum {
                one,
                zero,
                nonlinear,
            } => Sum {
                one: one * rhs,
                zero: zero * rhs,
                nonlinear: nonlinear.into_iter().map(|e| e * rhs).collect(),
            },
            Min { a, b } => Min {
                a: Box::new(a.mul(rhs)),
                b: Box::new(b.mul(rhs)),
            },
        }
    }
}

impl Div<f64> for Expr {
    type Output = Expr;
    fn div(self, rhs: f64) -> Expr {
        match self {
            Sum {
                one,
                zero,
                nonlinear,
            } => Sum {
                one: one / rhs,
                zero: zero / rhs,
                nonlinear: nonlinear.into_iter().map(|e| e / rhs).collect(),
            },
            Min { a, b } => Min {
                a: Box::new(a.div(rhs)),
                b: Box::new(b.div(rhs)),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn linear(x: f64, a: f64) -> Expr {
        Expr::self_consistent(x) + Expr::constant(a)
    }

    #[test]
    fn eval_linear() {
        assert_eq!(Expr::constant(3.456).eval(-3.0), 3.456);
        assert_eq!(Expr::self_consistent(1.0).eval(3.0), 3.0);
        assert_eq!(
            (Expr::self_consistent(1.5) + Expr::constant(1.0)).eval(10.0),
            16.0
        );
        assert_eq!(
            (Expr::self_consistent(-0.5) + Expr::constant(2.0)).eval(3.0),
            0.5
        );
    }

    #[test]
    fn eval_min() {
        let m = linear(2.0, -1.0).min(linear(-0.5, 4.0));
        assert_eq!(m.eval(0.0), -1.0);
        assert_eq!(m.eval(1.0), 1.0);
        assert_eq!(m.eval(2.0), 3.0);
        assert_eq!(m.eval(3.0), 2.5);
    }

    #[test]
    fn bisect() {
        let f = linear(0.5, 0.5).min(linear(0.0, 2.0));
        let g = linear(0.4, 1.0).min(linear(0.0, 3.0));
        let h = linear(0.6, 2.0).min(linear(0.0, 1.5));
        let e = (f + g + h) / 3.0;
        let x = e.bisect();
        assert!((e.eval(x) - x).abs() < 1e-6);
    }
}
