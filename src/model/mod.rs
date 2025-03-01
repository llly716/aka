pub mod proxy;
pub mod user;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SubscriptionUserinfo {
    pub upload: i128,
    pub download: i128,
    pub total: i128,
    pub expire: i128,
}
