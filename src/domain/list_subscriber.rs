use crate::domain::list_subscriber_email::ListSubscriberEmail;
use crate::domain::list_subscriber_name::ListSubscriberName;

pub struct ListSubscriber {
    pub name: ListSubscriberName,
    pub email: ListSubscriberEmail,
}

impl ListSubscriber {
    pub fn try_new(name: String, email: String) -> Result<Self, &'static str> {
        let name = ListSubscriberName::try_from(name)?;
        let email = ListSubscriberEmail::try_from(email)?;
        Ok(Self { name, email })
    }
}
