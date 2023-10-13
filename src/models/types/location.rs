pub struct Location {
    pub longtitude: f32,
    pub latitude: f32,
    pub horizontal_accuracy: Option<f32>,
    pub live_period: Option<i32>,
    pub heading: Option<i32>,
    pub proximity_alert_radius: Option<i32>,
}
