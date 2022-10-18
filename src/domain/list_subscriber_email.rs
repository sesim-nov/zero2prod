pub struct ListSubscriberEmail(String);

impl AsRef<str> for ListSubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for ListSubscriberEmail {
    type Error = &'static str;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        // TODO: Validation
        Ok(Self(s))
    }
}

// TODO: Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation_works() {
        let email: String = "email@domain.test".into();
        let user = ListSubscriberEmail::try_from(email.clone()).unwrap();
        assert_eq!(user.as_ref(), email);
    }
}
