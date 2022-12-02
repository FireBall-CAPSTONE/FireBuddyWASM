use std::ops::{Index, IndexMut, Mul, MulAssign, Add, AddAssign};

use super::{vec3::Vector3, quaternion::Quaternion};



#[derive(Debug, Clone, Copy)]
pub struct Matrix4 {
    pub data: [f32; 16]
}

#[allow(dead_code)]
impl Matrix4 {

    // Constructors

    /// Create a new 4x4 identity matrix
    /// 
    /// The identity matrix is a square matrix in which all the elements along
    /// the center diagonal are 1.0
    /// 
    /// # Examples
    /// ```
    /// let identity = Matrix::identity();
    /// 
    /// asserteq!(identity.data, [
    ///     1.0, 0.0, 0.0, 0.0,
    ///     0.0, 1.0, 0.0, 0.0,
    ///     0.0, 0.0, 1.0, 0.0,
    ///     0.0, 0.0, 0.0, 1.0
    /// ]);
    /// ```
    pub fn identity() -> Self {
        Self { data: [
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0
        ] }
    }

    /// Create a new 4x4 empty matrix
    /// 
    /// All the elements of the matrix are 0.0
    /// 
    /// # Examples
    /// ```
    /// let zero = Matrix::zero();
    /// 
    /// asserteq!(zero.data, [
    ///     0.0, 0.0, 0.0, 0.0,
    ///     0.0, 0.0, 0.0, 0.0,
    ///     0.0, 0.0, 0.0, 0.0,
    ///     0.0, 0.0, 0.0, 0.0
    /// ]);
    /// ```
    pub fn zero() -> Self {
        Self { data: [
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0
        ] }
    }

    /// Create a new 3D perspective projection matrix.
    /// 
    /// Takes in a field of view, aspect ratio, near plane distance and far plane distance
    /// and calculates a clipping space matrix for 3D perspective projection.
    /// 
    /// FOV is measured in radians.
    /// 
    /// # Examples
    /// ```
    /// // TODO: add examples
    /// ```
    pub fn perspective(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        
        let f = 1.0 / (fov/2.0).tan();
        let range_inv = 1.0 / (near - far);

        Self { data: [
            f/aspect_ratio, 0.0,                          0.0,  0.0,
                       0.0,   f,                          0.0,  0.0,
                       0.0, 0.0,     (near + far) * range_inv, -1.0,
                       0.0, 0.0, near * far * range_inv * 2.0,  0.0
        ] }
    }
    
    /// Create a new view matrix.
    /// 
    /// The view matrix is used to orient the camera in world space
    /// 
    /// # Examples
    /// ```
    /// // TODO: add examples
    /// ```
    pub fn view(position: Vector3, up: Vector3, forward: Vector3) -> Self {
        // TODO: Determine if this needs to be transposed or not

        let right = Vector3::cross(up, forward);
        Self { data: [
            right[0], up[0], forward[0], 0.0,
            right[1], up[1], forward[1], 0.0,
            right[2], up[2], forward[2], 0.0,
            Vector3::dot(right, -position), Vector3::dot(up, -position), Vector3::dot(forward, -position), 1.0
        ] }

    }

    pub fn view_xz(position: Vector3, right: Vector3, forward: Vector3) -> Self {
        
        let up = Vector3::cross(right, forward);
        Self { data: [
            right[0], up[0], forward[0], 0.0,
            right[1], up[1], forward[1], 0.0,
            right[2], up[2], forward[2], 0.0,
            Vector3::dot(right, -position), Vector3::dot(up, -position), Vector3::dot(forward, -position), 1.0
        ] }
    }

    /// Create a new translation matrix.
    /// 
    /// Matrix to translate an object in its coordinate system.
    /// 
    /// # Examples
    /// ```
    /// let x: f32 = 9.0;
    /// let y: f32 = 8.0;
    /// let z: f32 = 7.0;
    /// let transform = Matrix4::translate(9.0, 8.0, 7.0);
    /// 
    /// asserteq!(transform.data, [
    ///     1.0, 0.0, 0.0, 0.0,
    ///     0.0, 1.0, 0.0, 0.0,
    ///     0.0, 0.0, 1.0, 0.0,
    ///       x,   y,   z, 1.0
    /// ]);
    /// ```
    pub fn translate(x: f32, y: f32, z:f32) -> Self {
        Self { data: [
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
              x,   y,   z, 1.0
        ] }
    }

    /// Rotate based on a specified quaternion
    pub fn rotate(q: Quaternion) -> Self {
        let s = q.data[3];
        let q1 = q.data[0];
        let q2 = q.data[1];
        let q3 = q.data[2];

        Self { data: [
            (s * s) + (q1 * q1) - (q2 * q2) - (q3 * q3), 2.0 * (q1 * q2 - s * q3), 2.0 * (s * q2 + q1 * q3), 0.0,
            2.0 * (q1 * q2 + s * q3), (s * s) - (q1 * q1) + (q2 * q2) - (q3 * q3), 2.0 * (q2 * q3 - s * q1), 0.0,
            2.0 * (q1 * q3 - s * q2), 2.0 * (s * q1 + q2 * q3), (s * s) - (q1 * q1) - (q2 * q2) + (q3 * q3), 0.0,
            0.0, 0.0, 0.0, 1.0
        ] }
    }

    /// Create a new scaling matrix.
    /// 
    /// Scales an object on its local x, y, and z.
    /// 
    /// # Examples
    /// ```
    /// let x: f32 = 2.0;
    /// let y: f32 = 3.0;
    /// let z: f32 = 0.5;
    /// let scale = Matrix4::scale(2.0, 3.0, 0.5);
    /// 
    /// assert_eq!(scale.data, [
    ///       x, 0.0, 0.0, 0.0,
    ///     0.0,   y, 0.0, 0.0,
    ///     0.0, 0.0,   z, 0.0,
    ///     0.0, 0.0, 0.0, 1.0
    /// ]);
    /// ```
    pub fn scale(x: f32, y:f32, z:f32) -> Self {
        Self { data: [
              x, 0.0, 0.0, 0.0,
            0.0,   y, 0.0, 0.0,
            0.0, 0.0,   z, 0.0,
            0.0, 0.0, 0.0, 1.0
        ] }
    }

    /// Create a new uniform scaling matrix.
    /// 
    /// Similar to the scaling matrix, the uniform scaling matrix is
    /// uniform along each axis.
    /// 
    /// # Examples
    /// ```
    /// let s: f32 = 3.0;
    /// let scale = Matrix4::scale_uniform(3.0);
    /// 
    /// asserteq!(scale.data, [
    ///       s, 0.0, 0.0, 0.0,
    ///     0.0,   s, 0.0, 0.0,
    ///     0.0, 0.0,   s, 0.0,
    ///     0.0, 0.0, 0.0, 1.0
    /// ])
    /// ```
    pub fn scale_uniform(scale: f32) -> Self {
        Self::scale(scale, scale, scale)
    }

    pub fn transpose(self) -> Self {

        let mut out = self.clone();

        for x in 0 as usize..4 {
            for y in 0 as usize..4 {
                out[(x, y)] = out[(y, x)]
            }
        }

        out
    }
}

impl Default for Matrix4 {
    // Default for Matrix4 is identity
    fn default() -> Self {
        Self { data: [
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0
        ] }
    }
}

// Operators

impl Index<(usize, usize)> for Matrix4 {
    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[ index.0 + 4 * index.1 ]
    }
}

impl IndexMut<(usize, usize)> for Matrix4 {

    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[ index.0 + 4 * index.1 ]
    }
}

impl Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, rhs: Matrix4) -> Self::Output {
        let mut out = Matrix4::identity();
        
        for x in 0 as usize..4 {
            for y in 0 as usize..4 {
                // Dot product of collumn and row
                out[(x, y)] = 
                    self[(0, y)] * rhs[(x, 0)] +
                    self[(1, y)] * rhs[(x, 1)] +
                    self[(2, y)] * rhs[(x, 2)] +
                    self[(3, y)] * rhs[(x, 3)] 
            }
        }

        out
    }
}

impl MulAssign<Matrix4> for Matrix4 {
    fn mul_assign(&mut self, rhs: Matrix4) {
        let mut out = Matrix4::identity();
        
        for x in 0 as usize..4 {
            for y in 0 as usize..4 {
                // Dot product of collumn and row
                out[(x, y)] = 
                    self[(0, y)] * rhs[(x, 0)] +
                    self[(1, y)] * rhs[(x, 1)] +
                    self[(2, y)] * rhs[(x, 2)] +
                    self[(3, y)] * rhs[(x, 3)] 
            }
        }

        // Copy the contents of out to self
        self.data = out.data
    }
}

impl Add<Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn add(self, rhs: Matrix4) -> Self::Output {
        let mut out = Matrix4::identity();

        for x in 0 as usize..4 {
            for y in 0 as usize..4 {
                // Dot product of collumn and row
                out[(x, y)] = self[(x, y)] + rhs[(x, y)]
            }
        }

        out
    }
}

impl AddAssign<Matrix4> for Matrix4 {
    fn add_assign(&mut self, rhs: Matrix4) {
        for x in 0 as usize..4 {
            for y in 0 as usize..4 {
                // Dot product of collumn and row
                self[(x, y)] += rhs[(x, y)]
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_matrix_index() {
        let mut mat = Matrix4 { data: [
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 10.0, 11.0, 12.0,
            13.0, 14.0, 15.0, 16.0
        ] };

        let i = mat[(1, 2)];

        assert_eq!(i, 10.0);

        mat[(1, 2)] = 15.0;

        assert_eq!(mat[(1, 2)], 15.0);
    }

    #[test]
    fn test_matrix_multiplication() {

    }

    #[test]
    fn test_matrix_addition() {

    }
}

