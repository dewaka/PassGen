use std::borrow::Cow;

#[derive(Debug, PartialEq)]
pub struct Password<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> Password<'a> {
    pub fn new(value: impl Into<Cow<'a, str>>) -> Self {
        Self {
            value: value.into(),
        }
    }
}
