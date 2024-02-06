use super::vector2d::*;
use crate::vec2d;

#[derive(Debug)]
pub struct Matrix2d {
    v: [f64; 4],
}

impl Matrix2d {
   pub fn get_rows(&self) -> i32 {
        2
   }

   pub fn get_cols(&self) -> i32 {
        2
   }

   pub fn new(v: [f64; 4]) -> Self {
        Matrix2d {
            v
        }
   }

   pub fn new_from_constant(c: f64) -> Self {
        Matrix2d {
            v: [c, c, c, c]
        }
   }
}

// now we can also clone a matrix
impl Copy for Matrix2d { }
impl Clone for Matrix2d {
    fn clone(&self) -> Self {
        *self
    }
}

// we can use the subscripts with the matrix
impl std::ops::Index<usize> for Matrix2d {
    type Output = f64;
        
    fn index(&self, i: usize) -> &Self::Output {
        &self.v[i]
    }
}

// we can modify a single entry inside of the matrix using indexmut
impl std::ops::IndexMut<usize> for Matrix2d {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.v[i]
    }
}

// we can multiply a matrix by a scalar
impl std::ops::Mul<f64> for Matrix2d {
    type Output = Matrix2d;

    fn mul(self, l: f64) -> Self::Output {
        let new_v = [
            self.v[0] * l,
            self.v[1] * l,
            self.v[2] * l,
            self.v[3] * l,
        ];

        Matrix2d {
            v: new_v
        }
    }
}

// dot product
impl std::ops::Mul<Matrix2d> for Matrix2d {
    type Output = Matrix2d;

    fn mul(self, o: Matrix2d) -> Self::Output {
        Self {
            v: [
                self[0] * o[0] + self[1] * o[2],
                self[0] * o[1] + self[1] * o[3],

                self[2] * o[0] + self[3] * o[2],
                self[2] * o[1] + self[3] * o[3],
            ]
        }
    }
}

impl std::ops::Mul<Vector2d> for Matrix2d {
    type Output = Vector2d;
    fn mul(self, v: Vector2d) -> Self::Output {
        Self::Output {
            x: self[0] * v.x + self[1] * v.y,
            y: self[2] * v.x + self[3] * v.y,
        }
    }
}

// addition between 2 matrices
impl std::ops::Add<Matrix2d> for Matrix2d {
    type Output = Matrix2d;

    fn add(self, o: Matrix2d) -> Self::Output {
        Matrix2d {
            v: [self.v[0] + o.v[0], self.v[1] + o.v[1],self.v[2] + o.v[2],self.v[3] + o.v[3]]
        }
    }
}

// addition between a matrix and a scalar
impl std::ops::Add<f64> for Matrix2d {
    type Output = Matrix2d;

    fn add(self, l: f64) -> Self::Output {
        Matrix2d {
            v: [self.v[0] + l, self.v[1] + l, self.v[2] + l, self.v[3] + l]
        }
    }
}

// just like add
impl std::ops::Sub<Matrix2d> for Matrix2d {
    type Output = Matrix2d;

    fn sub(self, o: Matrix2d) -> Self::Output {
        Matrix2d {
            v: [self.v[0] - o.v[0], self.v[1] - o.v[1],self.v[2] - o.v[2],self.v[3] - o.v[3]]
        }
    }
}

// just like add
impl std::ops::Sub<f64> for Matrix2d {
    type Output = Matrix2d;

    fn sub(self, l: f64) -> Self::Output {
        Matrix2d {
            v: [self.v[0] - l, self.v[1] - l, self.v[2] - l, self.v[3] - l]
        }
    }
}

// Equality for Matrix2d
impl PartialEq for Matrix2d {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..4 {
            if self[i] != other[i] {
                return false;
            }
        }
        true
    }
}

impl Eq for Matrix2d {}

#[macro_export]
macro_rules! mat2d {
    ($c:expr) => {
        Matrix2d::new_from_constant($c)
    };

    ( $($e:expr),* ) => {
        Matrix2d::new([$($e),*])
    };

}

/* TESTS
 * 
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_init_macros() {
        let m = mat2d![1.0, 1.0, 1.0, 1.0];
        let m2 = mat2d![1.0];
            
        assert_eq!(m, m2);
    }

    #[test]
    fn matrix_add_sub() {
        let m = mat2d![1.0, 2.0, 3.0, 4.0];
        let m2 = mat2d![1.0, 2.0, 3.0, 4.0];
        assert_eq!(m + m2, mat2d![2.0, 4.0, 6.0, 8.0 ]);
        assert_eq!(m - m2, mat2d![0.0]);
    }

    #[test]
    fn matrix_mul() {
        let m = mat2d![1.0, 2.0, 
                       3.0, 4.0];
        let m2 = mat2d![1.0, 2.0, 
                        3.0, 4.0];
        assert_eq!(m * m2, mat2d![1.0 * 1.0 + 2.0 * 3.0, 1.0 * 2.0 + 2.0 * 4.0,
                                  3.0 * 1.0 + 4.0 * 3.0, 3.0 * 2.0 + 4.0 * 4.0
                                ])
    }
    
    #[test]
    fn matrix_scale() {
        let m = mat2d![1.0];
        let m2 = m * 5.0;
        assert_eq!(m2, mat2d![5.0]);
    }

    #[test]
    fn matrix_add_scalar() {
        let m = mat2d![1.0];
        let scalar = 1.0;
        assert_eq!(m + scalar, mat2d![2.0]);
    }

    #[test]
    fn matrix_vector_mul() {
        let m = mat2d![1.0, 0.0,
                       0.0, 1.0];

        let v = vec2d![2.0, 5.0];
        let voutput = m * v;
        assert_eq!(v, voutput);

        let m2 = mat2d![2.0, 4.0,
                        5.0, 6.0
                        ];
        
        let result = vec2d![2.0 * 2.0 + 4.0 * 5.0, 5.0 * 2.0 + 6.0 * 5.0];
        assert_eq!(result, m2 * v);
    }
    








}
