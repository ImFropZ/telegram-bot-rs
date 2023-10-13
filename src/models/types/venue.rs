use super::location::Location;

pub struct Venue {
    location: Location,
    title: String,
    address: String,
    foursquare_id: Option<String>,
    foursquare_type: Option<String>,
    google_place_id: Option<String>,
    google_place_type: Option<String>,
}
