use crate::array::linear_space;

pub fn radon(g: Vec<Vec<f32>>, n_rows: usize, n_cols: usize) -> Vec<Vec<f32>> {
    let pi = std::f32::consts::PI;
    let n_rays = 100_usize;
    let n_slopes = 100_usize;
    let x_min: f32 = -(n_rows as f32) / 2.;
    let y_min: f32 = -(n_rows as f32) / 2.;
    let alphas: Vec<f32> = linear_space(-pi / 4., pi / 4., n_slopes as i32);
    let p: Vec<f32> = alphas.iter().map(|alpha| alpha.tan()).collect();
    let mut g_radon: Vec<Vec<f32>> = vec![vec![0.; n_rays]; n_slopes];
    for k in 0..n_slopes {
        let mut tau = linear_space(-(n_rows as f32) / 2., (n_rows as f32) / 2., n_rays as i32);
        tau = tau.iter().map(|x| x / (pi / 2. - alphas[k]).sin()).collect();
        for h in 0..n_rays {
            let beta: f32 = p[k] * x_min + tau[h] - y_min;
            let alpha = p[k];
            let mut s: f32 = 0.;
            for m in 0..n_rows {
                let n = (alpha * m as f32 + beta) as usize;
                if n < n_cols {
                    s += g[m][n];
                }
            }
            g_radon[k][h] = s;
        }
    }
    g_radon
}
