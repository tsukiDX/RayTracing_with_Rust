pub struct FractSin {}

#[allow(dead_code)]
impl FractSin {
    pub fn rand11(x: f32) -> f32 {
        (1000. * x.sin()).fract().abs()
    }

    pub fn rand21(x: f32, y: f32) -> f32 {
        (((x * 12.9898 + y * 78.233).sin()) * 43758.544).fract().abs()
    }
}