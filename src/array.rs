pub fn linear_space(start: f32, stop: f32, steps: i32) -> Vec<f32> {
    let dx = (stop - start) / (steps as f32 - 1.0);
    (0..steps).map(|x| start + x as f32 * dx).collect()
}
