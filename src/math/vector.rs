use std::ops;

#[derive(Copy, Clone, Default)]
pub struct Vector2
{
    pub x: f32,
    pub y: f32
}

#[derive(Copy, Clone, Default)]
pub struct Vector3
{
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[derive(Copy, Clone, Default)]
pub struct Vector4
{
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

#[allow(dead_code)]
impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x: x,
            y: y
        }
    }

    pub fn mono(&mut self, v: f32) -> &mut Self {
        self.x = v;
        self.y = v;

        self
    }

    pub fn set(&mut self, vx: f32, vy: f32) -> &mut Self {
        self.x = vx;
        self.y = vy;

        self
    }

    pub fn magnitude_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn normalize(&mut self) -> &mut Self{
        let d = 1. / (self.x * self.x + self.y * self.y).sqrt();
        self.x *= d;
        self.y *= d;

        self
    }

    pub fn normalized(&self) -> Self {
        let d = 1. / (self.x * self.x + self.y * self.y).sqrt();
        Self {
            x: self.x * d,
            y: self.y * d
        }
    }

    pub fn lerp(a: Self, b: Self, t: f32) -> Self {
        a - (a - b) * t
    }

    pub fn floor(&self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor()
        }
    }

    pub fn ceil(&self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
        }
    }

    pub fn fract(&self) -> Self {
        Self {
            x: self.x.fract(),
            y: self.y.fract()
        }
    }

    pub fn rot(&self, t: f32) -> Self {
        let cos = t.cos();
        let sin = t.sin();

        Self { x: cos * self.x - sin * self.y, y: sin * self.x + cos * self.y }
    }

    pub fn zero() -> Self {
        Self { x: 0., y: 0. }
    }

    pub fn right() -> Self {
        Self { x: 1.0, y: 0.0 }
    }

    pub fn up() -> Self {
        Self {x: 0.0, y: 1.0}
    }
}



// ======== operators ========
// Add

// --- vector copy ---
impl ops::Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl ops::AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

// --- scalar ---
impl ops::Add<f32> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl ops::AddAssign<f32> for Vector2 {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
    }
}


// Sub

// --- vector copy---
impl ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::SubAssign for Vector2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

// scalar
impl ops::Sub<f32> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl ops::SubAssign<f32> for Vector2 {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
    }
}



// Mul

// --- vector copy ---
impl ops::Mul<Vector2> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

impl ops::MulAssign for Vector2 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}


// --- scalar ---
impl ops::Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}



// Mul

// --- vector copy ---
impl ops::Div<Vector2> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: Vector2) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl ops::DivAssign for Vector2 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}


// --- scalar ---
impl ops::Div<f32> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: f32) -> Self::Output {
        let d: f32 = 1.0 / rhs;

        Self {
            x: self.x * d,
            y: self.y * d,
        }
    }
}

impl ops::DivAssign<f32> for Vector2 {
    fn div_assign(&mut self, rhs: f32) {
        let d: f32 = 1.0 / rhs;

        self.x *= d;
        self.y *= d;
    }
}

impl ops::Neg for Vector2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y
        }
    }
}


pub type Point2D = Vector2;

#[allow(dead_code)]
impl Vector3 {

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z
        }
    }

    pub fn mono(&mut self, v: f32) -> &mut Self {
        self.x = v;
        self.y = v;
        self.z = v;

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

    pub fn dot(&self, other: Self) -> f32 {
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

    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }

    pub fn lerp(a: Self, b: Self, t: f32) -> Self {
        a - (a - b) * t
    }

    pub fn floor(&self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
            z: self.z.floor()
        }
    }

    pub fn ceil(&self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
            z: self.z.ceil()
        }
    }

    pub fn fract(&self) -> Self {
        Self {
            x: self.x.fract(),
            y: self.y.fract(),
            z: self.z.fract()
        }
    }

    pub fn zero() -> Self {
        Self { x: 0., y: 0., z: 0. }
    }

    pub fn right() -> Self {
        Self {
            x: 1.0,
            y: 0.0,
            z: 0.0
        }
    }

    pub fn up() -> Self {
        Self {
            x: 0.0,
            y: -1.0,
            z: 0.0
        }
    }

    pub fn forward() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 1.0
        }
    }

    pub fn rotx(&self, t: f32) -> Self {
        let p = Vector2::rot(&Vector2 { x: self.y, y: self.z }, t);
        Self { x: self.x, y: p.x, z: p.y }
    }

    pub fn roty(&self, t: f32) -> Self {
        let p = Vector2::rot(&Vector2 { x: self.x, y: self.z }, t);
        Self { x: p.x, y: self.y, z: p.y }
    }

    pub fn rotz(&self, t: f32) -> Self {
        let p = Vector2::rot(&Vector2 { x: self.x, y: self.y }, t);
        Self { x: self.x, y: p.x, z: p.y }
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

impl ops::Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}


pub type Point3D = Vector3;



impl Vector4 {

    pub fn new(v: f32) -> Self {
        Self {
            x: v,
            y: v,
            z: v,
            w: v
        }
    }

    pub fn mono(&mut self, v: f32) -> &mut Self {
        self.x = v;
        self.y = v;
        self.z = v;
        self.w = v;

        self
    }

    pub fn set(&mut self, vx: f32, vy: f32, vz: f32, vw: f32) -> &mut Self {
        self.x = vx;
        self.y = vy;
        self.z = vz;
        self.w = vw;

        self
    }

    pub fn magnitude_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn magnitude(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn normalize(&mut self) -> &mut Self{
        let d = 1. / (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt();
        self.x *= d;
        self.y *= d;
        self.z *= d;
        self.w *= d;

        self
    }

    pub fn normalized(self) -> Self {
        let d = 1. / (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt();
        Self {
            x: self.x * d,
            y: self.y * d,
            z: self.z * d,
            w: self.w * d
        }
    }

    pub fn lerp(a: Self, b: Self, t: f32) -> Self {
        a - (a - b) * t
    }

    pub fn floor(self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
            z: self.z.floor(),
            w: self.w.floor()
        }
    }

    pub fn ceil(self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
            z: self.z.ceil(),
            w: self.w.ceil()
        }
    }

    pub fn fract(self) -> Self {
        Self {
            x: self.x.fract(),
            y: self.y.fract(),
            z: self.z.fract(),
            w: self.w.fract()
        }
    }

    pub fn zero() -> Self {
        Self { x: 0., y: 0., z: 0., w: 0. }
    }
}



// ======== operators ========
// Add

// --- vector copy ---
impl ops::Add<Vector4> for Vector4 {
    type Output = Vector4;

    fn add(self, rhs: Vector4) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w
        }
    }
}

impl ops::AddAssign for Vector4 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

// --- scalar ---
impl ops::Add<f32> for Vector4 {
    type Output = Vector4;

    fn add(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
            w: self.w + rhs
        }
    }
}

impl ops::AddAssign<f32> for Vector4 {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
        self.w += rhs;
    }
}


// Sub

// --- vector copy---
impl ops::Sub<Vector4> for Vector4 {
    type Output = Vector4;

    fn sub(self, rhs: Vector4) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w
        }
    }
}

impl ops::SubAssign for Vector4 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}

// scalar
impl ops::Sub<f32> for Vector4 {
    type Output = Vector4;

    fn sub(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
            w: self.w - rhs
        }
    }
}

impl ops::SubAssign<f32> for Vector4 {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
        self.w -= rhs;
    }
}



// Mul

// --- vector copy ---
impl ops::Mul<Vector4> for Vector4 {
    type Output = Vector4;

    fn mul(self, rhs: Vector4) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w
        }
    }
}

impl ops::MulAssign for Vector4 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self.w *= rhs.w;
    }
}

// --- scalar ---
impl ops::Mul<f32> for Vector4 {
    type Output = Vector4;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs
        }
    }
}

impl ops::MulAssign<f32> for Vector4 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}



// Mul

// --- vector copy ---
impl ops::Div<Vector4> for Vector4 {
    type Output = Vector4;

    fn div(self, rhs: Vector4) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            w: self.w / rhs.w
        }
    }
}

impl ops::DivAssign for Vector4 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
        self.w /= rhs.w;
    }
}

// --- scalar ---
impl ops::Div<f32> for Vector4 {
    type Output = Vector4;

    fn div(self, rhs: f32) -> Self::Output {
        let d: f32 = 1.0 / rhs;

        Self {
            x: self.x * d,
            y: self.y * d,
            z: self.z * d,
            w: self.w * d
        }
    }
}

impl ops::DivAssign<f32> for Vector4 {
    fn div_assign(&mut self, rhs: f32) {
        let d: f32 = 1.0 / rhs;

        self.x *= d;
        self.y *= d;
        self.z *= d;
        self.w *= d;
    }
}

impl ops::Neg for Vector4 {
    type Output = Vector4;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w
        }
    }
}