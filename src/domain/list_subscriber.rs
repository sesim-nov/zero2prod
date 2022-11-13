use crate::domain::list_subscriber_email::ListSubscriberEmail;
use crate::domain::list_subscriber_name::ListSubscriberName;

#[derive(Clone, Debug)]
pub struct ListSubscriber {
    pub name: ListSubscriberName,
    pub email: ListSubscriberEmail,
}

impl ListSubscriber {
    pub fn try_new(name: String, email: String) -> Result<Self, String> {
        let name = ListSubscriberName::try_from(name)?;
        let email = ListSubscriberEmail::try_from(email)?;
        Ok(Self { name, email })
    }
}
