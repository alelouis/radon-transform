use crate::array::{mean, var};
use crate::csv_rw::{read_csv, write_vec_csv};
use std::time::Instant;

mod array;
mod csv_rw;
mod radon;

fn main() {
    // Radon transform parameters
    let n_rows = 512_usize;
    let n_cols = 512_usize;
    let n_rays = 200_usize;
    let n_slopes = 200_usize;

    // Load input image
    let input_path = "inputs/lena.csv".to_string();
    let image = read_csv(input_path).expect("Error reading input.");

    // Benchmark loop
    let n_trials = 10;
    let mut times: Vec<f32> = vec![];
    for _ in 0..n_trials {
        let t_start = Instant::now();
        radon::radon(&image, n_rows, n_cols, n_rays, n_slopes);
        let t_stop = t_start.elapsed();
        times.push(t_stop.as_secs_f32());
    }

    // Saving times
    write_vec_csv(&times, "outputs/rs_times.csv".to_string()).expect("Couldn't save times.");

    // Printing statistics
    println!(
        "mean: {:.2} ms, var: {:.2}",
        mean(&times) * 1000.,
        var(&times) * 1000.
    );
}
