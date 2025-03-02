use augurs::prophet::{Prophet, TrainingData, wasmstan::WasmstanOptimizer, Error, Predictions};

pub fn prophet_model (timestamps: Vec<i64>, values: Vec<f64>) -> Predictions {
    let data = TrainingData::new(timestamps, values);

    let optimizer = WasmstanOptimizer::new();
    let mut prophet = Prophet::new(Default::default(), optimizer);
    
    prophet.fit(data.expect("Failed to gather data to fit"), Default::default()).expect("Failed to fit model");
    
    // Make in-sample predictions
    let predictions = prophet.predict(None).expect("Failed to make predictions");
    
    println!("Predictions: {:?}", predictions.yhat.point);
    println!("Lower bounds: {:?}", predictions.yhat.lower.as_ref().unwrap());
    println!("Upper bounds: {:?}", predictions.yhat.upper.as_ref().unwrap());
    
    predictions
}