use std::ops::{Mul, MulAssign, AddAssign, SubAssign, Sub, Add, Neg, Index, IndexMut};

use nalgebra::Vector;

use super::{mat4::Matrix4, axis::Axis};


#[derive(Debug, Clone, Copy)]
pub struct Vector4 {
    data: [f32; 4]
}

impl Vector4 {

    // Constructors

    pub fn new(x:f32, y:f32, z:f32, w:f32) -> Vector4 {
        Self { data: [
            x,
            y,
            z,
            w
        ] }
    }

    pub fn zero() -> Self {
        Self { data: [
            0.0,
            0.0,
            0.0,
            0.0
        ] }
    }

    pub fn zero_point() -> Self {
        Self { data: [
            0.0,
            0.0,
            0.0,
            1.0
        ] }
    }

    pub fn forward() -> Self {
        Self { data: [
            0.0,
            0.0,
            1.0,
            0.0
        ] }
    }

    pub fn right() -> Self {
        Self { data: [
            1.0,
            0.0,
            0.0,
            0.0,
        ] }
    }

    pub fn up() -> Self {
        Self { data: [
            0.0,
            1.0,
            0.0,
            0.0
        ] }
    }

    pub fn dot(a: Vector4, b: Vector4) -> f32 {
        todo!()
    }

    pub fn magnitude(&self) -> f32 {
        (
            (self.data[0] * self.data[0]) +
            (self.data[1] * self.data[1]) +
            (self.data[2] * self.data[2]) +
            (self.data[3] * self.data[3])
        ).sqrt()
    }

    pub fn sqr_magnitude(&self) -> f32 {
        (self.data[0] * self.data[0]) +
        (self.data[1] * self.data[1]) +
        (self.data[2] * self.data[2]) +
        (self.data[3] * self.data[3])
    }

    pub fn normalize(&self) -> Vector4 {
        let mag = self.magnitude();

        // Evaluate if magnitude is 0.0 to prevent divide by 0 errors
        if mag != 0.0 {
            return *self * (1.0/mag);
        }

        *self

    }

}

impl Index<usize> for Vector4 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Vector4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl Index<Axis> for Vector4 {
    type Output = f32;

    fn index(&self, index: Axis) -> &Self::Output {
        match index {
            Axis::X => &self.data[0],
            Axis::Y => &self.data[1],
            Axis::Z => &self.data[2],
            Axis::W => &self.data[3]
        }
    }
}

impl IndexMut<Axis> for Vector4 {
    fn index_mut(&mut self, index: Axis) -> &mut Self::Output {
        match index {
            Axis::X => &mut self.data[0],
            Axis::Y => &mut self.data[1],
            Axis::Z => &mut self.data[2],
            Axis::W => &mut self.data[3]
        }
    }
}

impl Mul<f32> for Vector4 {
    type Output = Vector4;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output { data: [
            self.data[0] * rhs,
            self.data[1] * rhs,
            self.data[2] * rhs,
            self.data[3] * rhs
        ] }
    }
}

impl MulAssign<f32> for Vector4 {
    fn mul_assign(&mut self, rhs: f32) {
        self.data[0] *= rhs;
        self.data[1] *= rhs;
        self.data[2] *= rhs;
        self.data[3] *= rhs;
    }
}

impl Mul<Matrix4> for Vector4 {
    type Output = Vector4;

    fn mul(self, rhs: Matrix4) -> Self::Output {
        todo!()
    }
}

impl MulAssign<Matrix4> for Vector4 {
    fn mul_assign(&mut self, rhs: Matrix4) {
        todo!()
    }
}

impl Add<Vector4> for Vector4 {
    type Output = Vector4;

    fn add(self, rhs: Vector4) -> Self::Output {
        Self::Output { data: [
            self.data[0] + rhs.data[0],
            self.data[1] + rhs.data[1],
            self.data[2] + rhs.data[2],
            self.data[3] + rhs.data[3]
        ] }
    }
}

impl AddAssign<Vector4> for Vector4 {
    fn add_assign(&mut self, rhs: Vector4) {
        self.data[0] += rhs.data[0];
        self.data[1] += rhs.data[1];
        self.data[2] += rhs.data[2];
        self.data[3] += rhs.data[3];
    }
}

impl Sub<Vector4> for Vector4 {
    type Output = Vector4;

    fn sub(self, rhs: Vector4) -> Self::Output {
        Self::Output { data: [
            self.data[0] - rhs.data[0],
            self.data[1] - rhs.data[1],
            self.data[2] - rhs.data[2],
            self.data[3] - rhs.data[3]
        ] }
    }
}

impl SubAssign<Vector4> for Vector4 {
    fn sub_assign(&mut self, rhs: Vector4) {
        self.data[0] -= rhs.data[0];
        self.data[1] -= rhs.data[1];
        self.data[2] -= rhs.data[2];
        self.data[3] -= rhs.data[3];
    }
}

impl Neg for Vector4 {
    type Output = Vector4;

    fn neg(self) -> Self::Output {
        Self::Output { data: [
            -self.data[0],
            -self.data[1],
            -self.data[2],
            -self.data[3]
        ] }
    }
}