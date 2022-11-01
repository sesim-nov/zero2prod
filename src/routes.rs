//! Endpoints for the REST API exposed by this crate.
//!
//! This module contains the handlers for the various endpoints exposed by this application's REST
//! API.
mod greet;
mod health_check;
mod subscriptions;

pub use greet::*;
pub use health_check::*;
pub use subscriptions::*;
