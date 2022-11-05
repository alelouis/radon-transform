mod array;
mod csv_rw;
mod radon;

fn main() {
    let n_rows = 100_usize;
    let n_cols = 100_usize;
    let input_path = "input.csv".to_string();
    let g = csv_rw::read_csv(input_path).expect("Error reading input.");
    let g_radon = radon::radon(g, n_rows, n_cols);
    csv_rw::write_csv(g_radon, "rust_output.csv".to_string()).expect("Error writing output csv.");
}
