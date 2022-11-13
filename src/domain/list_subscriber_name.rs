use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone, Debug)]
pub struct ListSubscriberName(String);

impl AsRef<str> for ListSubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for ListSubscriberName {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        let is_blank = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let contains_bad_chars = {
            let bad_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
            s.chars().any(|c| bad_chars.contains(&c))
        };
        if is_blank || is_too_long || contains_bad_chars {
            Err("Validation on Subscriber Name Failed!".into())
        } else {
            Ok(Self(s))
        }
    }
}
