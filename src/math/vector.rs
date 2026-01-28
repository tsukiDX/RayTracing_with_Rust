use std::ops;

#[derive(Copy, Clone, Default)]
pub struct Vector3
{
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector3 {

    pub fn new(vx: f32, vy: f32, vz: f32) -> Self {
        Self {
            x: vx,
            y: vy,
            z: vz
        }
    }

    pub fn mono(&mut self, v: f32) -> &mut Self {
        self.x = v;
        self.y = v;
        self.z = v;

        self
    }

    pub fn zero(&mut self) -> &mut Self {
        self.x = 0.;
        self.y = 0.;
        self.z = 0.;

        self
    }

    pub fn set(&mut self, vx: f32, vy: f32, vz: f32) -> &mut Self {
        self.x = vx;
        self.y = vy;
        self.z = vz;

        self
    }

    pub fn magnitude_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn normalize(&mut self) -> &mut Self{
        let d = 1. / (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        self.x *= d;
        self.y *= d;
        self.z *= d;

        self
    }

    pub fn normalized(&self) -> Self {
        let d = 1. / (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Self {
            x: self.x * d,
            y: self.y * d,
            z: self.z * d
        }
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }

    pub fn lerp(a: &Self, b: &Self, t: f32) -> Self {
        *a - (*a - *b) * t
    }
}



// ======== operators ========
// Add

// --- vector copy ---
impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl ops::AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

// --- vector move ---
impl ops::Add<&Vector3> for &Vector3 {
    type Output = Vector3;

    fn add(self, rhs: &Vector3) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

// --- scalar ---
impl ops::Add<f32> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs
        }
    }
}

impl ops::AddAssign<f32> for Vector3 {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}


// Sub

// --- vector copy---
impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl ops::SubAssign for Vector3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

// --- vector move ---
impl ops::Sub<&Vector3> for &Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: &Vector3) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

// scalar
impl ops::Sub<f32> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs
        }
    }
}

impl ops::SubAssign<f32> for Vector3 {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}



// Mul

// --- vector copy ---
impl ops::Mul<Vector3> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

impl ops::MulAssign for Vector3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

// --- vector move ---
impl ops::Mul<&Vector3> for &Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: &Vector3) -> Self::Output {
        Vector3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

// --- scalar ---
impl ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl ops::MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}



// Mul

// --- vector copy ---
impl ops::Div<Vector3> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: Vector3) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z
        }
    }
}

impl ops::DivAssign for Vector3 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

// --- vector move ---
impl ops::Div<&Vector3> for &Vector3 {
    type Output = Vector3;

    fn div(self, rhs: &Vector3) -> Self::Output {
        Vector3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z
        }
    }
}

// --- scalar ---
impl ops::Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f32) -> Self::Output {
        let d: f32 = 1.0 / rhs;

        Self {
            x: self.x * d,
            y: self.y * d,
            z: self.z * d
        }
    }
}

impl ops::DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, rhs: f32) {
        let d: f32 = 1.0 / rhs;

        self.x *= d;
        self.y *= d;
        self.z *= d;
    }
}


pub type Point3D = Vector3;