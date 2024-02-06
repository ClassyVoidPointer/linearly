#[derive(Debug, Clone, Copy)]
pub struct Vector3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3d {
    pub fn new_from(values: [f64; 3]) -> Self {
        Self {
            x: values[0],
            y: values[1],
            z: values[2],
        }
    }

    pub fn new_from_const(c: f64) -> Self {
        Self {
            x: c,
            y: c,
            z: c,
        }
    }

    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn to_list(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }

    pub fn to_vec(&self) -> Vec<f64> {
        vec![self.x, self.y, self.z]
    }
}

impl std::ops::Add<Vector3d> for Vector3d {
    type Output = Vector3d;
    fn add(self, o: Self) -> Self::Output {
        Self {
            x: self.x + o.x,
            y: self.y + o.y,
            z: self.z + o.z,
        }
    }
}

impl std::ops::Add<f64> for Vector3d {
    type Output = Vector3d;
    fn add(self, value: f64) -> Self::Output {
        Self {
            x: self.x + value,
            y: self.y + value,
            z: self.z + value,
        }
    }
}

impl std::ops::Sub<Vector3d> for Vector3d {
    type Output = Vector3d;
    fn sub(self, o: Self) -> Self::Output {
        Self {
            x: self.x - o.x,
            y: self.y - o.y,
            z: self.z - o.z,
        }
    }
}

impl std::ops::Sub<f64> for Vector3d {
    type Output = Vector3d;
    fn sub(self, value: f64) -> Self::Output {
        Self {
            x: self.x - value,
            y: self.y - value,
            z: self.z - value,
        }
    }
}

impl std::ops::Mul<Vector3d> for Vector3d {
    type Output = f64;
    fn mul(self, o: Self) -> Self::Output {
        self.x * o.x + self.y * o.y + self.z * o.z
    }
}

impl std::ops::Mul<f64> for Vector3d {
    type Output = Vector3d;
    fn mul(self, o: f64) -> Self::Output {
        Self {
            x: o * self.x,
            y: o * self.y,
            z: o * self.z,
        }
    }
}

impl std::ops::Div<f64> for Vector3d {
    type Output = Vector3d;
    fn div(self, o: f64) -> Self::Output {
        Self {
            x: self.x / o,
            y: self.y / o,
            z: self.z / o,
        }
    }
}

impl PartialEq for Vector3d {
    fn eq(&self, o: &Self) -> bool {
        self.x == o.x && self.y == o.y && self.z == o.z
    }
}

impl Eq for Vector3d {}

#[macro_export]
macro_rules! vec3d {
    ($($e:expr),*) => {
        Vector3d::new_from([$($e),*])
    };

    ($e:expr) => {
        Vector3d::new_from_const($e)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector3d_init_and_cmp() {
        let v1 = vec3d![1.0, 2.0, 3.0];
        assert_eq!(v1.x, 1.0);
        assert_eq!(v1.y, 2.0);
        assert_eq!(v1.z, 3.0);

        let v2 = vec3d![1.0, 2.0, 3.0];
        assert_eq!(v1, v2);
    }

    #[test]
    fn dot_product_3d() {
        let v1 = vec3d![1.0, 2.0, 3.0];
        let v2 = vec3d![1.0, 2.0, 3.0];
        let dotty = v1 * v2;
        assert_eq!(dotty, 1.0 + 4.0 + 9.0);
    }

    #[test]
    fn vector3d_addition() {
        let v1 = vec3d![1.0, 2.0, 3.0];
        let v2 = vec3d![2.0, 3.0, 4.0];
        let sum = v1 + v2;
        assert_eq!(sum.x, 3.0);
        assert_eq!(sum.y, 5.0);
        assert_eq!(sum.z, 7.0);
    }

    #[test]
    fn vector3d_add_scalar() {
        let v1 = vec3d![1.0, 2.0, 3.0];
        let sum = v1 + 2.0;
        assert_eq!(sum.x, 3.0);
        assert_eq!(sum.y, 4.0);
        assert_eq!(sum.z, 5.0);
    }

    #[test]
    fn vector3d_subtraction() {
        let v1 = vec3d![1.0, 2.0, 3.0];
        let v2 = vec3d![2.0, 3.0, 4.0];
        let diff = v1 - v2;
        assert_eq!(diff.x, -1.0);
        assert_eq!(diff.y, -1.0);
        assert_eq!(diff.z, -1.0);
    }

    #[test]
    fn vector3d_sub_scalar() {
        let v1 = vec3d![1.0, 2.0, 3.0];
        let diff = v1 - 2.0;
        assert_eq!(diff.x, -1.0);
        assert_eq!(diff.y, 0.0);
        assert_eq!(diff.z, 1.0);
    }

    #[test]
    fn vector3d_multiplication_scalar() {
        let v1 = vec3d![1.0, 2.0, 3.0];
        let product = v1 * 2.0;
        assert_eq!(product.x, 2.0);
        assert_eq!(product.y, 4.0);
        assert_eq!(product.z, 6.0);
    }

    #[test]
    fn vector3d_division_scalar() {
        let v1 = vec3d![2.0, 4.0, 6.0];
        let quotient = v1 / 2.0;
        assert_eq!(quotient.x, 1.0);
        assert_eq!(quotient.y, 2.0);
        assert_eq!(quotient.z, 3.0);
    }
}
