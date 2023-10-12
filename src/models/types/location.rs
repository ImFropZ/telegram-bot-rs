pub struct Location {
    longtitude: f32,
    latitude: f32,
    horizontal_accuracy: Option<f32>,
    live_period: Option<i32>,
    heading: Option<i32>,
    proximity_alert_radius: Option<i32>,
}
