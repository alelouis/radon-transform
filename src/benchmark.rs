use crate::csv_rw::read_csv;
use std::time::Instant;

mod array;
mod csv_rw;
mod radon;

fn main() {
    let n_rows = 512_usize;
    let n_cols = 512_usize;
    let n_rays = 200_usize;
    let n_slopes = 200_usize;

    let input_path = "lena.csv".to_string();
    let image = read_csv(input_path).expect("Error reading input.");

    let n_trials = 10;
    let mut times: Vec<f32> = vec![];

    for _ in 0..n_trials {
        let t_start = Instant::now();
        radon::radon(&image, n_rows, n_cols, n_rays, n_slopes);
        let t_stop = t_start.elapsed();
        times.push(t_stop.as_secs_f32());
    }
    let mean: f32 = times.iter().sum::<f32>() / times.len() as f32;
    let var: f32 = times.iter().map(|x| x.powi(2)).sum::<f32>() / times.len() as f32;
    println!("mean: {:?}, var: {:?}", mean * 1000., var * 1000.);
}
