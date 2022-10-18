use std::ops::{Index, Mul, MulAssign, Add, AddAssign, Neg, Sub, SubAssign, IndexMut};

use nalgebra::coordinates::X;

use super::axis::{Axis, self};

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub data: [f32; 3]

}

impl Vector3 {

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { data: [
            x,
            y,
            z
        ] }
    }

    // Spatial Vectors

    pub fn zero() -> Self {
        Self { data: [
            0.0,
            0.0,
            0.0
        ] }
    }

    /// Unit vector pointing to absolute forward (`+z`)
    pub fn forward() -> Self {
        Self { data: [
            0.0,
            0.0,
            1.0
        ] }
    }

    /// Unit vector pointing to absolute right (`+x`)
    pub fn right() -> Self {
        Self { data: [
            1.0,
            0.0,
            0.0
        ] }
    }

    /// Unit vector pointing to absolute up (`+y`)
    pub fn up() -> Self {
        Self { data: [
            0.0,
            1.0,
            0.0
        ] }
    }


    // Vector operations

    pub fn dot(a: Vector3, b: Vector3) -> f32 {
        a[0] * b[0] +
        a[1] * b[1] +
        a[2] * b[2]
    }

    /// Cross product of two vectors
    /// 
    /// The cross product of two unit vectors results in a unit vector,
    /// perpandicular to the plane formed by a and b
    pub fn cross(a: Vector3, b: Vector3) -> Self {

        Vector3 { data: [
            a[Axis::Y] * b[Axis::Z] - a[Axis::Z] * b[Axis::Y],
            a[Axis::Z] * b[Axis::X] - a[Axis::X] * b[Axis::Y],
            a[Axis::X] * b[Axis::Y] - a[Axis::Y] * b[Axis::X]
        ] }
    }

    /// Return the magnitude of the vector
    /// 
    /// The length of the vector defined by `sqrt(x^2 + y^2 + z^2)`
    pub fn magnitude(&self) -> f32 {
        (
            (self.data[0] * self.data[0]) +
            (self.data[1] * self.data[1]) +
            (self.data[2] * self.data[2])
        ).sqrt()
    }

    /// Return the squared magnitude of the vector
    /// 
    /// The squared length of the vector defined by `x^2 + y^2 + z^2`
    /// 
    /// Largely unnecessary optimization as `sqrt` is pretty fast
    pub fn sqr_magnitude(&self) -> f32 {
        (self.data[0] * self.data[0]) +
        (self.data[1] * self.data[1]) +
        (self.data[2] * self.data[2])
    }

    /// Return a new unit vector with the same direction
    /// 
    /// Divide each field by the magnitude of the vector and return
    /// the result as a new vector.
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();

        // Evaluate if magnitude is 0.0 to prevent divide by 0 errors
        if mag != 0.0 {
            return *self * (1.0/mag);
        }

        *self
    }



}

impl Default for Vector3 {
    fn default() -> Self {
        Self { data: [
            0.0,
            0.0,
            0.0
        ] }
    }
}

/// Index a Vector3 by i32
impl Index<i32> for Vector3 {
    type Output = f32;

    fn index(&self, index: i32) -> &Self::Output {
        &self.data[index as usize]
    }
}

impl IndexMut<i32> for Vector3 {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        &mut self.data[index as usize]
    }
}

/// Index a Vector3 by u32
impl Index<u32> for Vector3 {
    type Output = f32;

    fn index(&self, index: u32) -> &Self::Output {
        &self.data[index as usize]
    }
}

impl IndexMut<u32> for Vector3 {
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        &mut self.data[index as usize]
    }
}

/// Index a Vector3 by usize
impl Index<usize> for Vector3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

/// Index a Vector3 by Axis
impl Index<Axis> for Vector3 {
    type Output = f32;

    fn index(&self, axis: Axis) -> &Self::Output {
        match axis {
            Axis::X => &self.data[0],
            Axis::Y => &self.data[1],
            Axis::Z => &self.data[2],
            _ => &0.0
        }
    }
}

impl IndexMut<Axis> for Vector3 {
    fn index_mut(&mut self, axis: Axis) -> &mut Self::Output {
        match axis {
            Axis::X => &mut self.data[0],
            Axis::Y => &mut self.data[1],
            Axis::Z => &mut self.data[2],
            _ => &mut self.data[3]
        }
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs:f32) -> Self::Output {
        Vector3 { data: [
            self.data[0] * rhs,
            self.data[1] * rhs,
            self.data[2] * rhs
        ] }
    }
}

impl Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, rhs:Vector3) -> Self::Output {
        Vector3 { data: [
            rhs.data[0] * self,
            rhs.data[1] * self,
            rhs.data[2] * self
        ] }
    }
}

impl MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.data[0] *= rhs;
        self.data[1] *= rhs;
        self.data[2] *= rhs;
    }
}

impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Self::Output { data: [
            self.data[0] + rhs.data[0],
            self.data[1] + rhs.data[1],
            self.data[2] + rhs.data[2]
        ] }
    }
}

impl AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: Vector3) {
        self.data[0] += rhs.data[0];
        self.data[1] += rhs.data[1];
        self.data[2] += rhs.data[2];
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Self::Output { data: [
            self.data[0] - rhs.data[0],
            self.data[1] - rhs.data[1],
            self.data[2] - rhs.data[2]
        ] }
    }
}

impl SubAssign<Vector3> for Vector3 {
    fn sub_assign(&mut self, rhs: Vector3) {
        self.data[0] -= rhs.data[0];
        self.data[1] -= rhs.data[1];
        self.data[2] -= rhs.data[2];
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Self::Output { data: [
            -self.data[0],
            -self.data[1],
            -self.data[2]
        ] }
    }
}

impl Neg for &Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Self::Output { data: [
            -self.data[0],
            -self.data[1],
            -self.data[2]
        ] }
    }
}
