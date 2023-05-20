use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize)]
pub struct DepartureTime {
    pub hh: Option<i64>,
    pub mn: Option<i64>,
}
