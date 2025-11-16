pub trait Message {
    fn format(&self) -> String;
}

pub struct StringMessage {
    string: String,
}

impl StringMessage {
    fn new(s: String) -> Self {
        StringMessage { string: s }
    }
}

impl Message for StringMessage {
    fn format(&self) -> String {
        self.string.clone()
    }
}

pub struct UppercaseMessage<T>
where
    T: Message,
{
    inner: T,
}

impl<T> UppercaseMessage<T>
where
    T: Message,
{
    fn new(message: T) -> Self {
        UppercaseMessage { inner: message }
    }
}

impl<T> Message for UppercaseMessage<T>
where
    T: Message,
{
    fn format(&self) -> String {
        let lower = self.inner.format();
        lower.to_uppercase()
    }
}

pub struct BracketedMessage<T>
where
    T: Message,
{
    inner: T,
}

impl<T> BracketedMessage<T>
where
    T: Message,
{
    fn new(message: T) -> Self {
        BracketedMessage { inner: message }
    }
}

impl<T> Message for BracketedMessage<T>
where
    T: Message,
{
    fn format(&self) -> String {
        format!("[{}]", self.inner.format())
    }
}

#[cfg(test)]
mod tests {
    use crate::decorator::decorator::Message;

    use super::{BracketedMessage, StringMessage, UppercaseMessage};

    #[test]
    fn test_composed_decorators() {
        let message = BracketedMessage::new(UppercaseMessage::new(StringMessage::new(
            "Hello, World!".to_string(),
        )));
        assert_eq!(message.format(), "[HELLO, WORLD!]")
    }
}
