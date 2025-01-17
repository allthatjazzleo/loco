//! This module contains a base routes related to health checks and status
//! reporting. These routes are commonly used to monitor the health of the
//! application and its dependencies.

use axum::routing::get;
use serde::Serialize;

use super::{format, routes::Routes};
use crate::{controller::Json, Result};

/// Represents the health status of the application.
#[derive(Serialize)]
struct Health {
    pub ok: bool,
}

/// Check application ping endpoint
async fn ping() -> Result<Json<Health>> {
    format::json(Health { ok: true })
}

/// Defines and returns the health-related routes.
pub fn routes() -> Routes {
    Routes::new().add("/_ping", get(ping))
}
