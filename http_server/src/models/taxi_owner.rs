use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct TaxiOwner {
    pub id: String,
    pub name: String,
    pub taxi_id: Uuid,
}
