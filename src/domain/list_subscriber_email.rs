pub struct ListSubscriberEmail(String);

impl AsRef<str> for ListSubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for ListSubscriberEmail {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        if validator::validate_email(&s) {
            Ok(Self(s))
        } else {
            Err(format!("Email {} failed validation.", s))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation_works() {
        let email: String = "email@domain.test".into();
        let user = ListSubscriberEmail::try_from(email.clone()).unwrap();
        assert_eq!(user.as_ref(), email);
    }
    #[test]
    fn no_at_sign() {
        let email: String = "emaildomain.test".into();
        let user = ListSubscriberEmail::try_from(email.clone());
        assert!(user.is_err());
    }
    #[test]
    fn no_user() {
        let email: String = "@domain.test".into();
        let user = ListSubscriberEmail::try_from(email.clone());
        assert!(user.is_err());
    }
    #[test]
    fn blank() {
        let email: String = "".into();
        let user = ListSubscriberEmail::try_from(email.clone());
        assert!(user.is_err());
    }
}
