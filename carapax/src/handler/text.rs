use crate::{
    context::Context,
    handler::{Handler, HandlerFuture, HandlerResult},
};
use failure::Error;
use regex::Regex;
use tgbot::types::{Message, Text};

/// Rule for text handler
pub trait TextRule {
    /// Whether rule accepts given text
    fn accepts(&self, text: &Text) -> bool;
}

#[doc(hidden)]
pub struct TextRuleContains {
    substring: String,
}

impl TextRule for TextRuleContains {
    fn accepts(&self, text: &Text) -> bool {
        text.data.contains(&self.substring)
    }
}

#[doc(hidden)]
pub struct TextRuleEquals {
    text: String,
}

impl TextRule for TextRuleEquals {
    fn accepts(&self, text: &Text) -> bool {
        text.data == self.text
    }
}

#[doc(hidden)]
pub struct TextRuleMatches {
    pattern: Regex,
}

impl TextRule for TextRuleMatches {
    fn accepts(&self, text: &Text) -> bool {
        self.pattern.is_match(&text.data)
    }
}

impl<F> TextRule for F
where
    F: Fn(&Text) -> bool,
{
    fn accepts(&self, text: &Text) -> bool {
        (self)(text)
    }
}

/// A rules based message text handler
pub struct TextHandler<R, H> {
    rule: R,
    handler: H,
}

impl<R, H> TextHandler<R, H> {
    /// Creates a new handler
    pub fn new(rule: R, handler: H) -> Self {
        Self { rule, handler }
    }
}

impl<H> TextHandler<TextRuleContains, H> {
    /// Create a handler for messages contains given text
    pub fn contains<S>(text: S, handler: H) -> Self
    where
        S: Into<String>,
    {
        Self::new(TextRuleContains { substring: text.into() }, handler)
    }
}

impl<H> TextHandler<TextRuleEquals, H> {
    /// Create a handler for messages equals given text
    pub fn equals<S>(text: S, handler: H) -> Self
    where
        S: Into<String>,
    {
        Self::new(TextRuleEquals { text: text.into() }, handler)
    }
}

impl<H> TextHandler<TextRuleMatches, H> {
    /// Create a handler for messages matches given text
    ///
    /// See [regex](https://docs.rs/regex) crate for more information about patterns
    pub fn matches<S>(pattern: S, handler: H) -> Result<Self, Error>
    where
        S: AsRef<str>,
    {
        Ok(Self::new(
            TextRuleMatches {
                pattern: Regex::new(pattern.as_ref())?,
            },
            handler,
        ))
    }
}

impl<T, H, O> Handler for TextHandler<T, H>
where
    T: TextRule,
    H: Handler<Input = Message, Output = O>,
    O: Into<HandlerFuture>,
{
    type Input = Message;
    type Output = HandlerFuture;

    fn handle(&self, context: &mut Context, message: Self::Input) -> Self::Output {
        if message.get_text().map(|text| self.rule.accepts(text)).unwrap_or(false) {
            self.handler.handle(context, message).into()
        } else {
            HandlerResult::Continue.into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handler::{FnHandler, FromUpdate};
    use futures::Future;

    fn handle_message(_context: &mut Context, _message: Message) -> HandlerResult {
        HandlerResult::Stop
    }

    #[test]
    fn contains() {
        let mut context = Context::default();
        let handler = TextHandler::contains("substring", FnHandler::from(handle_message));
        for (update, result) in vec![
            // Rule accepts text
            (
                serde_json::json!({
                    "update_id": 1,
                    "message": {
                        "message_id": 1111,
                        "date": 0,
                        "from": {"id": 1, "is_bot": false, "first_name": "test"},
                        "chat": {"id": 1, "type": "private", "first_name": "test"},
                        "text": "test substring contains"
                    }
                }),
                HandlerResult::Stop,
            ),
            // Rule does not accept text
            (
                serde_json::json!({
                    "update_id": 1,
                    "message": {
                        "message_id": 1111,
                        "date": 0,
                        "from": {"id": 1, "is_bot": false, "first_name": "test"},
                        "chat": {"id": 1, "type": "private", "first_name": "test"},
                        "text": "test skip"
                    }
                }),
                HandlerResult::Continue,
            ),
        ] {
            let message = Message::from_update(serde_json::from_value(update).unwrap()).unwrap();
            assert_eq!(handler.handle(&mut context, message).wait().unwrap(), result);
        }
    }

    #[test]
    fn equals() {
        let mut context = Context::default();
        let handler = TextHandler::equals("test equals", FnHandler::from(handle_message));
        for (update, result) in vec![
            (
                // Rule accepts text
                serde_json::json!({
                    "update_id": 1,
                    "message": {
                        "message_id": 1111,
                        "date": 0,
                        "from": {"id": 1, "is_bot": false, "first_name": "test"},
                        "chat": {"id": 1, "type": "private", "first_name": "test"},
                        "text": "test equals"
                    }
                }),
                HandlerResult::Stop,
            ),
            // Rule does not accept text
            (
                serde_json::json!({
                    "update_id": 1,
                    "message": {
                        "message_id": 1111,
                        "date": 0,
                        "from": {"id": 1, "is_bot": false, "first_name": "test"},
                        "chat": {"id": 1, "type": "private", "first_name": "test"},
                        "text": "test skip"
                    }
                }),
                HandlerResult::Continue,
            ),
        ] {
            let message = Message::from_update(serde_json::from_value(update).unwrap()).unwrap();
            assert_eq!(handler.handle(&mut context, message).wait().unwrap(), result);
        }
    }

    #[test]
    fn matches() {
        let mut context = Context::default();
        let handler = TextHandler::matches("matches$", FnHandler::from(handle_message)).unwrap();
        for (update, result) in vec![
            (
                serde_json::json!({
                    "update_id": 1,
                    "message": {
                        "message_id": 1111,
                        "date": 0,
                        "from": {"id": 1, "is_bot": false, "first_name": "test"},
                        "chat": {"id": 1, "type": "private", "first_name": "test"},
                        "text": "test matches"
                    }
                }),
                HandlerResult::Stop,
            ),
            // Rule does not accept text
            (
                serde_json::json!({
                    "update_id": 1,
                    "message": {
                        "message_id": 1111,
                        "date": 0,
                        "from": {"id": 1, "is_bot": false, "first_name": "test"},
                        "chat": {"id": 1, "type": "private", "first_name": "test"},
                        "text": "test skip"
                    }
                }),
                HandlerResult::Continue,
            ),
        ] {
            let message = Message::from_update(serde_json::from_value(update).unwrap()).unwrap();
            assert_eq!(handler.handle(&mut context, message).wait().unwrap(), result);
        }
    }

    #[test]
    fn predicate() {
        let mut context = Context::default();
        let handler = TextHandler::new(
            |text: &Text| text.data.contains("predicate"),
            FnHandler::from(handle_message),
        );
        for (update, result) in vec![
            (
                serde_json::json!({
                    "update_id": 1,
                    "message": {
                        "message_id": 1111,
                        "date": 0,
                        "from": {"id": 1, "is_bot": false, "first_name": "test"},
                        "chat": {"id": 1, "type": "private", "first_name": "test"},
                        "text": "test predicate"
                    }
                }),
                HandlerResult::Stop,
            ),
            // Rule does not accept text
            (
                serde_json::json!({
                    "update_id": 1,
                    "message": {
                        "message_id": 1111,
                        "date": 0,
                        "from": {"id": 1, "is_bot": false, "first_name": "test"},
                        "chat": {"id": 1, "type": "private", "first_name": "test"},
                        "text": "test skip"
                    }
                }),
                HandlerResult::Continue,
            ),
        ] {
            let message = Message::from_update(serde_json::from_value(update).unwrap()).unwrap();
            assert_eq!(handler.handle(&mut context, message).wait().unwrap(), result);
        }
    }
}
