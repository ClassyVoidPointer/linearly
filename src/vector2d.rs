#[derive(Debug, Clone, Copy)]
pub struct Vector2d {
    pub x: f64,
    pub y: f64,
}

impl Vector2d {
    pub fn new_from(values: [f64; 2]) -> Self {
        Self {
            x: values[0],
            y: values[1],
        }
    }

    pub fn new_from_const(c: f64) -> Self {
        Self {
            x: c,
            y: c,
        }
    }

    pub fn new() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn to_list(&self) -> [f64; 2] {
        [self.x, self.y]
    }

    pub fn to_vec(&self) -> Vec<f64> {
        vec![self.x, self.y]
    }
}


impl std::ops::Add<Vector2d> for Vector2d {
    type Output = Vector2d;
    fn add(self, o: Self) -> Self::Output {
        Self {
            x: self.x + o.x,
            y: self.y + o.y,
        }
    }
}

impl std::ops::Add<f64> for Vector2d {
    type Output = Vector2d;
    fn add(self, value: f64) -> Self::Output {
        Self {
            x: self.x + value,
            y: self.y + value,
        }
    }
}

impl std::ops::Sub<Vector2d> for Vector2d {
    type Output = Vector2d;
    fn sub(self, o: Self) -> Self::Output {
        Self {
            x: self.x - o.x,
            y: self.y - o.y,
        }
    }
}

impl std::ops::Sub<f64> for Vector2d {
    type Output = Vector2d;
    fn sub(self, value: f64) -> Self::Output {
        Self {
            x: self.x - value,
            y: self.y - value,
        }
    }
}

impl std::ops::Mul<Vector2d> for Vector2d {
    type Output = f64;
    fn mul(self, o: Self) -> Self::Output {
        self.x * o.x + self.y * o.y 
    }
}

impl std::ops::Mul<f64> for Vector2d {
    type Output = Vector2d;
    fn mul(self, o: f64) -> Self::Output {
        Self {
            x: o * self.x,
            y: o * self.y
        }
    }
}

impl std::ops::Div<f64> for Vector2d {
    type Output = Vector2d;
    fn div(self, o: f64) -> Self::Output {
        Self {
            x: self.x / o,
            y: self.y / o 
        }
    }
}

impl PartialEq for Vector2d {
    fn eq(&self, o: &Self) -> bool {
        self.x == o.x && self.y == o.y
    }
}

impl Eq for Vector2d { }

#[macro_export]
macro_rules! vec2d {
    ($($e: expr),*) => {
        Vector2d::new_from( [ $($e),* ] )
    };

    ( $e: expr ) => {
        Vector2d::new_from_const($e);
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_init_and_cmp() {
        let v1 = vec2d![1.0, 2.0];
        assert_eq!(v1.x, 1.0);
        assert_eq!(v1.y, 2.0);

        let v2 = vec2d![1.0, 2.0];
        assert_eq!(v1, v2);
    }

    #[test]
    fn dot_product() {
        let v1 = vec2d![1.0, 2.0];
        let v2 = vec2d![1.0, 2.0];
        let dotty = v1 * v2;
        assert_eq!(dotty, 1.0 + 4.0);
    }
}
