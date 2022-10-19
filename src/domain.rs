//! Domain Types
//!
//! This module contains types to validate data used internally to the crate.

/// A struct used to validate subscriber names meet the database requirements.
mod list_subscriber;
mod list_subscriber_email;
mod list_subscriber_name;

pub use list_subscriber::ListSubscriber;
pub use list_subscriber_email::ListSubscriberEmail;
pub use list_subscriber_name::ListSubscriberName;
