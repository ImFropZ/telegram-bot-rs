use super::user::User;

pub struct ProximityAlertTriggered {
    traveler: User,
    watcher: User,
    distance: u32,
}
