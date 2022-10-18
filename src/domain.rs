//! Domain Types
//!
//! This module contains types to validate data used internally to the crate.

/// A struct used to validate subscriber names meet the database requirements.
use unicode_segmentation::UnicodeSegmentation;

pub struct ListSubscriber {
    pub name: ListSubscriberName,
    pub email: String,
}

impl ListSubscriber {
    pub fn try_new(name: String, email: String) -> Result<Self, &'static str> {
        let name = ListSubscriberName::try_from(name)?;
        Ok(Self { name, email })
    }
}

pub struct ListSubscriberName(String);

impl AsRef<str> for ListSubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for ListSubscriberName {
    type Error = &'static str;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        let is_blank = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let contains_bad_chars = {
            let bad_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
            s.chars().any(|c| bad_chars.contains(&c))
        };
        if is_blank || is_too_long || contains_bad_chars {
            Err("Validation on Subscriber Name Failed!")
        } else {
            Ok(Self(s))
        }
    }
}
