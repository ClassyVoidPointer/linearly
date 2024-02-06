use super::vector4d::*;

pub struct Matrix4d {
    v: [f64; 16],
}

impl Matrix4d {
    pub fn get_rows(&self) -> i32 {
        4
    }

    pub fn get_cols(&self) -> i32 {
        4
    }

    pub fn new(v: [f64; 16]) -> Self {
        Self { v }
    }

    pub fn new_from_constant(c: f64) -> Self {
        Self {
            v: [c; 16],
        }
    }
}

impl Copy for Matrix4d {}
impl Clone for Matrix4d {
    fn clone(&self) -> Self {
        *self
    }
}

impl std::ops::Index<usize> for Matrix4d {
    type Output = f64;

    fn index(&self, i: usize) -> &Self::Output {
        &self.v[i]
    }
}

impl std::ops::IndexMut<usize> for Matrix4d {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.v[i]
    }
}

impl std::ops::Mul<f64> for Matrix4d {
    type Output = Matrix4d;

    fn mul(self, l: f64) -> Self::Output {
        Self {
            v: [
                self.v[0] * l, self.v[1] * l, self.v[2] * l, self.v[3] * l,
                self.v[4] * l, self.v[5] * l, self.v[6] * l, self.v[7] * l,
                self.v[8] * l, self.v[9] * l, self.v[10] * l, self.v[11] * l,
                self.v[12] * l, self.v[13] * l, self.v[14] * l, self.v[15] * l,
            ]
        }
    }
}

impl std::ops::Mul<Matrix4d> for Matrix4d {
    type Output = Matrix4d;

    fn mul(self, o: Matrix4d) -> Self::Output {
        Self {
            v: [
                self[0] * o[0] + self[1] * o[4] + self[2] * o[8] + self[3] * o[12],
                self[0] * o[1] + self[1] * o[5] + self[2] * o[9] + self[3] * o[13],
                self[0] * o[2] + self[1] * o[6] + self[2] * o[10] + self[3] * o[14],
                self[0] * o[3] + self[1] * o[7] + self[2] * o[11] + self[3] * o[15],

                self[4] * o[0] + self[5] * o[4] + self[6] * o[8] + self[7] * o[12],
                self[4] * o[1] + self[5] * o[5] + self[6] * o[9] + self[7] * o[13],
                self[4] * o[2] + self[5] * o[6] + self[6] * o[10] + self[7] * o[14],
                self[4] * o[3] + self[5] * o[7] + self[6] * o[11] + self[7] * o[15],

                self[8] * o[0] + self[9] * o[4] + self[10] * o[8] + self[11] * o[12],
                self[8] * o[1] + self[9] * o[5] + self[10] * o[9] + self[11] * o[13],
                self[8] * o[2] + self[9] * o[6] + self[10] * o[10] + self[11] * o[14],
                self[8] * o[3] + self[9] * o[7] + self[10] * o[11] + self[11] * o[15],

                self[12] * o[0] + self[13] * o[4] + self[14] * o[8] + self[15] * o[12],
                self[12] * o[1] + self[13] * o[5] + self[14] * o[9] + self[15] * o[13],
                self[12] * o[2] + self[13] * o[6] + self[14] * o[10] + self[15] * o[14],
                self[12] * o[3] + self[13] * o[7] + self[14] * o[11] + self[15] * o[15],
            ],
        }
    }
}

impl std::ops::Mul<Vector4d> for Matrix4d {
    type Output = Vector4d;

    fn mul(self, v: Vector4d) -> Self::Output {
        let x = self[0] * v.x + self[1] * v.y + self[2] * v.z + self[3] * v.w;
        let y = self[4] * v.x + self[5] * v.y + self[6] * v.z + self[7] * v.w;
        let z = self[8] * v.x + self[9] * v.y + self[10] * v.z + self[11] * v.w;
        let w = self[12] * v.x + self[13] * v.y + self[14] * v.z + self[15] * v.w;

        Vector4d::new_from([x, y, z, w])
    }
}


impl std::ops::Add<Matrix4d> for Matrix4d {
    type Output = Matrix4d;

    fn add(self, o: Matrix4d) -> Self::Output {
        Self {
            v: [
                self.v[0] + o.v[0], self.v[1] + o.v[1], self.v[2] + o.v[2], self.v[3] + o.v[3],
                self.v[4] + o.v[4], self.v[5] + o.v[5], self.v[6] + o.v[6], self.v[7] + o.v[7],
                self.v[8] + o.v[8], self.v[9] + o.v[9], self.v[10] + o.v[10], self.v[11] + o.v[11],
                self.v[12] + o.v[12], self.v[13] + o.v[13], self.v[14] + o.v[14], self.v[15] + o.v[15],
            ],
        }
    }
}

impl std::ops::Add<f64> for Matrix4d {
    type Output = Matrix4d;

    fn add(self, l: f64) -> Self::Output {
        Self {
            v: [
                self[0] + l, self[1] + l, self[2] + l, self[3] + l,
                self[4] + l, self[5] + l, self[6] + l, self[7] + l,
                self[8] + l, self[9] + l, self[10] + l, self[11] + l,
                self[12] + l, self[13] + l, self[14] + l, self[15] + l,
            ]
        }
    }
}
// Equality for Matrix4d
impl PartialEq for Matrix4d {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..16 {
            if self[i] != other[i] {
                return false;
            }
        }
        true
    }
}

impl Eq for Matrix4d {}

#[macro_export]
macro_rules! mat4d {
    ([$($e:expr),*]) => {
        Matrix4d::new([$($e),*])
    };
    ($c:expr) => {
        Matrix4d::new_from_constant($c)
    };
}
