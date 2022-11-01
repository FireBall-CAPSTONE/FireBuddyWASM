use std::ops::{Index, IndexMut, Mul, MulAssign, Add, AddAssign, Neg, Sub, SubAssign};

pub struct Vector2 {
    pub data: [f32; 2]

}

impl Vector2 {

    /// Constructor for a new Vector2
    pub fn new(x: f32, y: f32) -> Self {
        Self { data: [
            x,
            y,
        ] }
    }

    /// New Vector2 where all components are `0.0`
    pub fn zero() -> Self {
        Self { data: [
            0.0,
            0.0
        ] }
    }

    /// Unit vector pointing absolute up (`+y`)
    pub fn up() -> Self {
        Self { data: [
            0.0,
            1.0
        ] }
    }

    /// Unit vector pointing absolute right (`+x`)
    pub fn right() -> Self {
        Self { data: [
            1.0,
            0.0
        ] }
    }

    /// Dot product of two vectors
    pub fn dot(a: Vector2, b: Vector2) -> f32 {
        a[0] * b[0] +
        a[1] * b[1]
    }

    /// Magnitude of a two component vector
    pub fn magnitude(&self) -> f32 {
        (
            (self[0] * self[0]) +
            (self[1] * self[1])
        ).sqrt()
    }

    /// Magnitude squared of a two component vector
    pub fn sqr_magnitude(&self) -> f32 {
        (self[0] * self[0]) +
        (self[1] * self[1])
    }

    pub fn normalize(&self) -> Self {
        todo!()
    }
}

impl Index<i32> for Vector2 {
    type Output = f32;
    
    fn index(&self, index: i32) -> &Self::Output {
        &self.data[index as usize]
    }
}

impl IndexMut<i32> for Vector2 {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        &mut self.data[index as usize]
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { data: [
            self.data[0] + rhs.data[0],
            self.data[1] + rhs.data[1]
        ] }
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        self.data[0] += rhs.data[0];
        self.data[1] += rhs.data[1];
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { data: [
            self.data[0] - rhs.data[0],
            self.data[1] - rhs.data[1]
        ] }
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.data[0] -= rhs.data[0];
        self.data[1] -= rhs.data[1];
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self { data: [
            self.data[0] * rhs,
            self.data[1] * rhs
        ] }
    }
}

impl MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.data[0] *= rhs;
        self.data[1] *= rhs;
    }
}

impl Neg for Vector2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { data: [
            -self.data[0],
            -self.data[1]
        ] }
    }
}