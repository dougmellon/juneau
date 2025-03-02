use std::fs::ReadDir;
use crate::csv_parser::parse_csv_file;
use crate::prophet_model::prophet_model;

mod csv_parser;
mod prophet_model;

#[derive(Debug)]
pub struct ProphetPredictionResult {
    pub identifier: String,  //TODO: Need to hash the identifier for uniqueness
    pub unix_timestamp: Vec<f64>,
    pub yhat_point: Vec<f64>,
    pub yhat_lower: Vec<f64>,
    pub yhat_upper: Vec<f64>,
    pub regressors: Vec<f64>, //TODO: Sort out how to handle regressor responses
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dataset = parse_csv_file("src/datasets/single_dataset.csv")?;

    let mut results: Vec<ProphetPredictionResult> = Vec::new();

    for (i, row) in dataset.iter().enumerate() {
        let prediction = prophet_model(row.timestamps.clone(), row.values.clone());

        results.push(ProphetPredictionResult {
            identifier: format!("Row {}", i), //TODO: Need to hash the result for uniqueness
            unix_timestamp: prediction.yhat.point.clone(),
            yhat_point: prediction.yhat.point.clone(),
            yhat_lower: prediction.yhat.lower.unwrap(),
            yhat_upper: prediction.yhat.upper.unwrap(),
            regressors: vec![0.0], //TODO: Sort out how to handle regressor responses
        });
    }

    println!("{:?}", results[0]);

    Ok(())
}
