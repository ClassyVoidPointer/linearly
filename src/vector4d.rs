#[derive(Debug, Clone, Copy)]
pub struct Vector4d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vector4d {
    pub fn new_from(values: [f64; 4]) -> Self {
        Self {
            x: values[0],
            y: values[1],
            z: values[2],
            w: values[3],
        }
    }

    pub fn new_from_const(c: f64) -> Self {
        Self {
            x: c,
            y: c,
            z: c,
            w: c,
        }
    }

    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    pub fn to_list(&self) -> [f64; 4] {
        [self.x, self.y, self.z, self.w]
    }
    pub fn to_vec(&self) -> Vec<f64> {
        vec![self.x, self.y, self.z, self.w]
    }
}

impl std::ops::Add<Vector4d> for Vector4d {
    type Output = Vector4d;
    fn add(self, o: Self) -> Self::Output {
        Self {
            x: self.x + o.x,
            y: self.y + o.y,
            z: self.z + o.z,
            w: self.w + o.w,
        }
    }
}

impl std::ops::Add<f64> for Vector4d {
    type Output = Vector4d;
    fn add(self, value: f64) -> Self::Output {
        Self {
            x: self.x + value,
            y: self.y + value,
            z: self.z + value,
            w: self.w + value,
        }
    }
}

impl std::ops::Sub<Vector4d> for Vector4d {
    type Output = Vector4d;
    fn sub(self, o: Self) -> Self::Output {
        Self {
            x: self.x - o.x,
            y: self.y - o.y,
            z: self.z - o.z,
            w: self.w - o.w,
        }
    }
}

impl std::ops::Sub<f64> for Vector4d {
    type Output = Vector4d;
    fn sub(self, value: f64) -> Self::Output {
        Self {
            x: self.x - value,
            y: self.y - value,
            z: self.z - value,
            w: self.w - value,
        }
    }
}

impl std::ops::Mul<Vector4d> for Vector4d {
    type Output = f64;
    fn mul(self, o: Self) -> Self::Output {
        self.x * o.x + self.y * o.y + self.z * o.z + self.w * o.w
    }
}

impl std::ops::Mul<f64> for Vector4d {
    type Output = Vector4d;
    fn mul(self, o: f64) -> Self::Output {
        Self {
            x: o * self.x,
            y: o * self.y,
            z: o * self.z,
            w: o * self.w,
        }
    }
}

impl std::ops::Div<f64> for Vector4d {
    type Output = Vector4d;
    fn div(self, o: f64) -> Self::Output {
        Self {
            x: self.x / o,
            y: self.y / o,
            z: self.z / o,
            w: self.w / o,
        }
    }
}

impl PartialEq for Vector4d {
    fn eq(&self, o: &Self) -> bool {
        self.x == o.x && self.y == o.y && self.z == o.z && self.w == o.w
    }
}

impl Eq for Vector4d {}

#[macro_export]
macro_rules! vec4d {
    ($($e:expr),*) => {
        Vector4d::new_from([$($e),*])
    };

    ($e:expr) => {
        Vector4d::new_from_const($e)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector4d_init_and_cmp() {
        let v1 = vec4d![1.0, 2.0, 3.0, 4.0];
        assert_eq!(v1.x, 1.0);
        assert_eq!(v1.y, 2.0);
        assert_eq!(v1.z, 3.0);
        assert_eq!(v1.w, 4.0);

        let v2 = vec4d![1.0, 2.0, 3.0, 4.0];
        assert_eq!(v1, v2);
    }

    #[test]
    fn dot_product_4d() {
        let v1 = vec4d![1.0, 2.0, 3.0, 4.0];
        let v2 = vec4d![1.0, 2.0, 3.0, 4.0];
        let dotty = v1 * v2;
        assert_eq!(dotty, 1.0 + 4.0 + 9.0 + 16.0);
    }
}

