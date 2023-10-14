use serde::{Deserialize, Serialize};

use super::user::User;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProximityAlertTriggered {
    pub traveler: User,
    pub watcher: User,
    pub distance: u32,
}
