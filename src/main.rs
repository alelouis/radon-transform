mod array;
mod csv_rw;
mod radon;

fn main() {
    let n_rows = 512_usize;
    let n_cols = 512_usize;
    let n_rays = 1000_usize;
    let n_slopes = 1000_usize;

    let input_path = "inputs/lena.csv".to_string();
    let image = csv_rw::read_csv(input_path).expect("Error reading input.");
    let radon_transform = radon::radon(&image, n_rows, n_cols, n_rays, n_slopes);
    csv_rw::write_mat_csv(&radon_transform, "outputs/radon_transform.csv".to_string())
        .expect("Error writing output csv.");
}
