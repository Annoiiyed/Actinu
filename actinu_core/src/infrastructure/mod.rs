pub mod static_data;

impl static_data::StaticData {
    pub fn new(json: &str) -> Self {
        serde_json::from_str(json).expect("Could not parse static data")
    }
}
