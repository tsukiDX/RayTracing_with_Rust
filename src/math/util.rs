pub fn equal(a: f32, b: f32) -> bool {
    (a - b).abs() < 1e-6
}