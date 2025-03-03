#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::time::Instant;
use clap::{Parser, ArgAction};
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

#[derive(Parser, Debug)]
#[command(name = "juneau", version, about = "Time series CLI", long_about = None)]
struct Cli {
    #[arg(long, short = 'd', use_value_delimiter = true, value_delimiter = ',', action = ArgAction::Append)]
    data: Vec<String>,

    #[arg(long, short = 'm', default_value = "prophet")]
    model: String,

    #[arg(long, short = 'r')]
    regressors: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if cli.model.to_lowercase() != "prophet" {
        eprintln!("Only Prophet is supported right now. I will implement more models soon.");
        std::process::exit(1);
    }

    let start_time = Instant::now();

    let mut all_results: Vec<ProphetPredictionResult> = Vec::new();

    for (file_idx, data_file) in cli.data.iter().enumerate() {
        println!("Processing file #{}: {}", file_idx + 1, data_file);

        let dataset = parse_csv_file(data_file)?;
        
        for (row_idx, row) in dataset.iter().enumerate() {
            let prediction = prophet_model(row.timestamps.clone(), row.values.clone());

            let result = ProphetPredictionResult {
                identifier: format!("File {} Row {}", file_idx + 1, row_idx),
                unix_timestamp: prediction.yhat.point.clone(),
                yhat_point: prediction.yhat.point.clone(),
                yhat_lower: prediction.yhat.lower.unwrap(),
                yhat_upper: prediction.yhat.upper.unwrap(),
                regressors: vec![0.0],
            };

            all_results.push(result);
        }
    }

    println!("Results: {all_results:#?}");

    let duration = start_time.elapsed();
    println!("Execution time: {:?}", duration);

    if let Some(reg) = cli.regressors {
        println!("Regressors flag provided: {}", reg);
        // TODO: handle regressors. connect to FRED API for economic data.
    }

    Ok(())
}
