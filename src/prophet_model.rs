use augurs::prophet::{Prophet, TrainingData, wasmstan::WasmstanOptimizer, Predictions};

// TODO: Need to establish a check to verify data size. https://github.com/facebook/prophet/issues/783
pub fn prophet_model (timestamps: Vec<i64>, values: Vec<f64>) -> Predictions {
    // TODO: Additional support https://docs.augu.rs/tutorials/forecasting-with-prophet.html
    let data = TrainingData::new(timestamps, values);

    let optimizer = WasmstanOptimizer::new();
    let mut prophet = Prophet::new(Default::default(), optimizer);
    
    prophet.fit(data.expect("Failed to gather data to fit"), Default::default()).expect("Failed to fit model");

    prophet.predict(None).expect("Failed to make predictions")
}