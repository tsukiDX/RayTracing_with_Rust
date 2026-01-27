use crate::math::array::Array;

type VInt3 = Array<i32, 3>;

pub struct ResultImage {
    width: i32,
    height: i32,
    aspect_ratio: f32,
    pub pixels: Vec<VInt3>
}

impl ResultImage {
    pub fn new(w: i32, h: i32) -> ResultImage {
        Self {
            width: w,
            height: h,
            aspect_ratio: w as f32 / h as f32,
            pixels: vec![VInt3::new(); (w * h)as usize]
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }
}