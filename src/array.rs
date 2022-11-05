/// Construct a linear spaced vector between two values
pub fn linear_space(start: f32, stop: f32, steps: i32) -> Vec<f32> {
    let dx = (stop - start) / (steps as f32 - 1.0);
    (0..steps).map(|x| start + x as f32 * dx).collect()
}

/// Compute mean of vector
#[allow(dead_code)]
pub fn mean(array: &Vec<f32>) -> f32 {
    array.iter().sum::<f32>() / array.len() as f32
}

/// Compute variance of vector
#[allow(dead_code)]
pub fn var(array: &Vec<f32>) -> f32 {
    array.iter().map(|x| x.powi(2)).sum::<f32>() / array.len() as f32
}
