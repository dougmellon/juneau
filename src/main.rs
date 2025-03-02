use crate::csv_parser::parse_csv_file;
use crate::prophet_model::prophet_model;

mod csv_parser;
mod prophet_model;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dataset = parse_csv_file("src/datasets/single_dataset.csv")?;

    for (i, row) in dataset.iter().enumerate() {
        let prediction = prophet_model(row.timestamps.clone(), row.values.clone());
    }

    Ok(())
}
