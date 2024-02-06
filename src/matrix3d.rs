use super::vector3d::*;
use crate::vec3d;

#[derive(Debug)]
pub struct Matrix3d {
    v: [f64; 9],
}

impl Matrix3d {
   pub fn get_rows(&self) -> i32 {
        3
   }

   pub fn get_cols(&self) -> i32 {
        3
   }

   pub fn new(v: [f64; 9]) -> Self {
        Matrix3d {
            v
        }
   }

   pub fn new_from_constant(c: f64) -> Self {
        Matrix3d {
            v: [c, c, c, 
                c, c, c, 
                c, c, c
            ]
        }
   }
}

// now we can also clone a matrix
impl Copy for Matrix3d { }
impl Clone for Matrix3d {
    fn clone(&self) -> Self {
        *self
    }
}

// we can use the subscripts with the matrix
impl std::ops::Index<usize> for Matrix3d {
    type Output = f64;
        
    fn index(&self, i: usize) -> &Self::Output {
        &self.v[i]
    }
}

// we can modify a single entry inside of the matrix using indexmut
impl std::ops::IndexMut<usize> for Matrix3d {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.v[i]
    }
}

// we can multiply a matrix by a scalar
impl std::ops::Mul<f64> for Matrix3d {
    type Output = Matrix3d;

    fn mul(self, l: f64) -> Self::Output {
        let new_v = [
            self.v[0] * l,
            self.v[1] * l,
            self.v[2] * l,
            self.v[3] * l,
            self.v[4] * l,
            self.v[5] * l,
            self.v[6] * l,
            self.v[7] * l,
            self.v[8] * l,
        ];

        Matrix3d {
            v: new_v
        }
    }
}

// dot product
impl std::ops::Mul<Matrix3d> for Matrix3d {
    type Output = Matrix3d;

    fn mul(self, o: Matrix3d) -> Self::Output {
        Self {
            v: [
                self[0] * o[0] + self[1] * o[3] + self[2] * o[6],
                self[0] * o[1] + self[1] * o[4] + self[2] * o[7],
                self[0] * o[2] + self[1] * o[5] + self[2] * o[8],

                self[3] * o[0] + self[4] * o[3] + self[5] * o[6],
                self[3] * o[1] + self[4] * o[4] + self[5] * o[7],
                self[3] * o[2] + self[4] * o[5] + self[5] * o[8],

                self[6] * o[0] + self[7] * o[3] + self[8] * o[6],
                self[6] * o[1] + self[7] * o[4] + self[8] * o[7],
                self[6] * o[2] + self[7] * o[5] + self[8] * o[8],

            ]
        }
    }
}

impl std::ops::Mul<Vector3d> for Matrix3d {
    type Output = Vector3d;

    fn mul(self, v: Vector3d) -> Self::Output {
        let x = self[0] * v.x + self[1] * v.y + self[2] * v.z;
        let y = self[3] * v.x + self[4] * v.y + self[5] * v.z;
        let z = self[6] * v.x + self[7] * v.y + self[8] * v.z;

        Vector3d::new_from([x, y, z])
    }
}

// addition between 2 matrices
impl std::ops::Add<Matrix3d> for Matrix3d {
    type Output = Matrix3d;

    fn add(self, o: Matrix3d) -> Self::Output {
        Matrix3d {
            v: [
                self.v[0] + o.v[0], self.v[1] + o.v[1], self.v[2] + o.v[2],
                self.v[3] + o.v[3], self.v[4] + o.v[4], self.v[5] + o.v[5],
                self.v[6] + o.v[6], self.v[7] + o.v[7], self.v[8] + o.v[8],
            ]
        }
    }
}

// addition between a matrix and a scalar
impl std::ops::Add<f64> for Matrix3d {
    type Output = Matrix3d;

    fn add(self, l: f64) -> Self::Output {
        Matrix3d {
            v: [
                self.v[0] +l, self.v[1] +l, self.v[2] +l,
                self.v[3] +l, self.v[4] +l, self.v[5] +l,
                self.v[6] +l, self.v[7] +l, self.v[8] +l,
            ]
        }
    }
}

// just like add
impl std::ops::Sub<Matrix3d> for Matrix3d {
    type Output = Matrix3d;

    fn sub(self, o: Matrix3d) -> Self::Output {
        Matrix3d {
            v: [
                self.v[0] - o.v[0], self.v[1] - o.v[1], self.v[2] - o.v[2],
                self.v[3] - o.v[3], self.v[4] - o.v[4], self.v[5] - o.v[5],
                self.v[6] - o.v[6], self.v[7] - o.v[7], self.v[8] - o.v[8],
            ]
        }
    }
}

// just like add
impl std::ops::Sub<f64> for Matrix3d {
    type Output = Matrix3d;

    fn sub(self, l: f64) -> Self::Output {
        Matrix3d {
            v: [
                self.v[0] - l, self.v[1] - l, self.v[2] - l,
                self.v[3] - l, self.v[4] - l, self.v[5] - l,
                self.v[6] - l, self.v[7] - l, self.v[8] - l,
            ]
        }
    }
}

// Equality for Matrix3d
impl PartialEq for Matrix3d {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..9 {
            if self[i] != other[i] {
                return false;
            }
        }
        true
    }
}

impl Eq for Matrix3d { }

#[macro_export]
macro_rules! mat3d {
    ($c:expr) => {
        Matrix3d::new_from_constant($c)
    };

    ($($e:expr),*) => {
        Matrix3d::new( [ $($e),*] )
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_init_macros() {
        let m = mat3d![1.0];
        assert_eq!(m, mat3d![1.0, 1.0, 1.0, 
                             1.0, 1.0, 1.0, 
                             1.0, 1.0, 1.0]);
    }

    #[test]
    fn matrix_vector_mul() {
        let m = mat3d![1.0, 2.0, 3.0, 
                       4.0, 5.0, 6.0, 
                       7.0, 8.0, 9.0];
        let v = vec3d![2.0, 3.0, 4.0];
        let result = m * v;
        let expected_result = vec3d![2.0 + 6.0 + 12.0, 
                                     8.0 + 15.0 + 24.0, 
                                     14.0 + 24.0 + 36.0];
        assert_eq!(result, expected_result);
    }

    // TODO: implement the remaining tests for matrix multiplication, addition, subtraction
}
