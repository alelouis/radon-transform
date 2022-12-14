use csv::Writer;
use std::error::Error;
use std::fs::File;

#[allow(dead_code)]
/// Write matrix to .csv
pub fn write_mat_csv(g: &Vec<Vec<f32>>, path: String) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(path)?;
    for line in g.iter() {
        wtr.write_record(line.iter().map(|x| x.to_string()))?;
    }
    wtr.flush()?;
    Ok(())
}

#[allow(dead_code)]
/// Write vector to .csv
pub fn write_vec_csv(g: &Vec<f32>, path: String) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(path)?;
    wtr.write_record(g.iter().map(|x| x.to_string()))?;
    wtr.flush()?;
    Ok(())
}

/// Read .csv to matrix
#[allow(dead_code)]
pub fn read_csv(path: String) -> Result<Vec<Vec<f32>>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);
    let mut mat: Vec<Vec<f32>> = vec![];
    for result in rdr.records() {
        let record = result?;
        let line: Vec<f32> = record
            .iter()
            .map(|x| x.parse::<f32>().expect("Couldn't parse string."))
            .collect();
        mat.push(line);
    }
    Ok(mat)
}
