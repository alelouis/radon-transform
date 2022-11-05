use crate::array::linear_space;
use std::cmp::{max, min};
use std::f32::consts::PI as pi;

/// Computes the angles between two bounds
fn compute_angles(min_angle: f32, max_angle: f32, n_angles: i32) -> Vec<f32> {
    linear_space(min_angle, max_angle, n_angles)
}

/// Computes slopes from angles
fn compute_slopes(angles: &Vec<f32>) -> Vec<f32> {
    angles.iter().map(|alpha| alpha.tan()).collect()
}

/// Computes offsets (b in ax+b) for constant distance between rays
fn compute_offsets(n_rows: usize, n_rays: usize, angles: &Vec<f32>, k: usize) -> Vec<f32> {
    linear_space(-(n_rows as f32) / 2., (n_rows as f32) / 2., n_rays as i32)
        .iter()
        .map(|x| x / (pi / 2. - angles[k]).sin())
        .collect()
}

/// Computes rows loop bounds for efficiency
fn compute_m_limits(n_rows: usize, n_cols: usize, alpha: f32, beta: f32) -> (usize, usize) {
    let mut m_min: usize = 0;
    let mut m_max: usize = n_rows - 1;
    if alpha > 0. {
        m_min = max(0, (-(beta + 0.5) / alpha).ceil() as usize);
        m_max = min(
            n_rows - 1,
            ((n_cols as f32 - 0.5 - beta) / alpha).floor() as usize,
        );
    } else if alpha < 0. {
        m_min = max(0, ((n_cols as f32 - 0.5 - beta) / alpha).ceil() as usize);
        m_max = min(n_rows - 1, (-(beta + 0.5) / alpha).floor() as usize);
    }
    (m_min, m_max)
}

/// Computes the discrete radon transform
pub fn radon(
    image: &Vec<Vec<f32>>,
    n_rows: usize,
    n_cols: usize,
    n_rays: usize,
    n_slopes: usize,
) -> Vec<Vec<f32>> {
    let x_min: f32 = -(n_rows as f32) / 2.;
    let y_min: f32 = -(n_cols as f32) / 2.;
    let angles: Vec<f32> = compute_angles(-pi / 4., pi / 4., n_slopes as i32);
    let slopes: Vec<f32> = compute_slopes(&angles);
    let mut radon_transform: Vec<Vec<f32>> = vec![vec![0.; n_rays]; n_slopes];
    let mut s: f32;
    for k in 0..n_slopes {
        let offsets = compute_offsets(n_rows, n_rays, &angles, k);
        for h in 0..n_rays {
            let (alpha, beta) = (slopes[k], slopes[k] * x_min + offsets[h] - y_min);
            let (m_min, m_max) = compute_m_limits(n_rows, n_cols, alpha, beta);
            let mut n_acc = beta + m_min as f32 * alpha;
            s = 0.;
            for m in m_min..m_max {
                let n = n_acc as usize;
                if n < n_cols {
                    s += image[m][n];
                }
                n_acc += alpha;
            }
            radon_transform[k][h] = s;
        }
    }
    radon_transform
}
