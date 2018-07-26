type Real = f64;
#[derive(Clone, Copy, Debug)]
struct Point {
    x: Real,
    y: Real,
}

impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x+rhs.x,
            y: self.y+rhs.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x-rhs.x,
            y: self.y-rhs.y,
        }
    }
}

impl std::ops::Mul<Real> for Point {
    type Output = Point;
    fn mul(self, rhs: Real) -> Self::Output {
        Point {
            x: self.x*rhs,
            y: self.y*rhs,
        }
    }
}

impl Point {
    fn new(x: Real, y: Real) -> Point {
        Point {
            x: x,
            y: y,
        }
    }

    fn rotate(&self, theta: Real) -> Point {
        Point {
            x: self.x*theta.cos() - self.y*theta.sin(),
            y: self.x*theta.sin() + self.y*theta.cos(),
        }
    }

    fn manhatan(&self) -> Point {
        Point {
            x: self.x-self.y,
            y: self.x+self.y,
        }
    }

    fn round(&self) -> Point {
        Point {
            x: self.x.round(),
            y: self.y.round(),
        }
    }
}