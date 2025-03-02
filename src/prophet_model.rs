use augurs::prophet::{Prophet, TrainingData, wasmstan::WasmstanOptimizer, Error, Predictions};

pub fn prophet_model (timestamps: Vec<i64>, values: Vec<f64>) -> Predictions {
    let data = TrainingData::new(timestamps, values);

    let optimizer = WasmstanOptimizer::new();
    let mut prophet = Prophet::new(Default::default(), optimizer);
    
    prophet.fit(data.expect("Failed to gather data to fit"), Default::default()).expect("Failed to fit model");
    
    prophet.predict(None).expect("Failed to make predictions")
}