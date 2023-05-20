pub mod password;
pub mod time;

use crate::utils::time::DepartureTime;
use ::time::{Duration, Instant, Time};
use chrono::{DateTime, Timelike, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const ONE_HOUR_IN_MN: i64 = 60;

#[derive(Serialize, Deserialize)]
pub struct Link {
    pub href: String,
}

pub fn get_uuid_from_station_url(url: &str) -> Uuid {
    url.split("/").last().unwrap().parse().unwrap()
}

// pub fn departure(d: &Time) -> DepartureTime {
//     // let rt = (d - Instant::now()).num_minutes();
//     DepartureTime {
//         hh: get_hour(rt),
//         mn: get_mn(rt),
//     }
// }
//
// fn get_hour(h: i64) -> Option<i64> {
//     match h / ONE_HOUR_IN_MN {
//         0 => None,
//         h => Some(h),
//     }
// }

fn get_mn(h: i64) -> Option<i64> {
    match h % ONE_HOUR_IN_MN {
        0 => None,
        h => Some(h),
    }
}
