mod accounts;
mod searches;
mod stations;
mod taxis;
mod trips;

pub use accounts::routes as accounts_routes;
pub use searches::routes as searches_routes;
pub use stations::routes as stations_routes;
pub use taxis::routes as taxis_routes;
pub use trips::routes as trips_routes;
