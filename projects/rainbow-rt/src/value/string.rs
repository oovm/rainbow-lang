use super::*;

///
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Text {
    pub(crate) handler: Option<String>,
    pub(crate) delimiter: TextDelimiter,
    pub(crate) value: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TextDelimiter {
    Bare,
    // "
    Quotation(usize),
    // '
    Apostrophe(usize),
    /// `‹`: U+2039
    /// `›`: U+203A
    SingleAngleQuotation,
    /// `«`: U+00AB
    /// `»`: U+00BB
    DoubleAngleQuotation,
}

impl Display for Text {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.handler {
            None => (),
            Some(s) => write!(f, "{}", s)?,
        }
        match self.delimiter {
            TextDelimiter::Bare => {
                write!(f, "{}", self.value)?;
            }
            TextDelimiter::Quotation(n) => {
                write!(f, "{}", "\"".repeat(n))?;
                write!(f, "{}", self.value)?;
                write!(f, "{}", "\"".repeat(n))?;
            }
            TextDelimiter::Apostrophe(n) => {
                write!(f, "{}", "\'".repeat(n))?;
                write!(f, "{}", self.value)?;
                write!(f, "{}", "\'".repeat(n))?;
            }
            TextDelimiter::DoubleAngleQuotation => {
                write!(f, "«")?;
                write!(f, "{}", self.value)?;
                write!(f, "»")?;
            }
            TextDelimiter::SingleAngleQuotation => {
                write!(f, "‹")?;
                write!(f, "{}", self.value)?;
                write!(f, "»")?;
            }
        }
        Ok(())
    }
}

impl Default for Text {
    fn default() -> Self {
        Self { handler: None, delimiter: TextDelimiter::Bare, value: String::new() }
    }
}

macro_rules! native2string {
    ($T:ty) => {
    impl From<$T> for Text {
        fn from(v: $T) -> Self {
            Self {
                handler: None,
                delimiter: TextDelimiter::Quotation(1),
                value: String::from(v)
            }
        }
    }
    };
    ($($T:ty), +) => {
        $(wrap_native!($T);)+
    };
}

macro_rules! native2value {
    ($T:ty) => {
    native2string!($T);
    impl From<$T> for Value {
        fn from(v: $T) -> Self {
            Value::String(Box::new(v.into()))
        }
    }
    };
    ($($T:ty), +) => {
        $(native2value!($T);)+
    };
}

native2value![char, &str, String, &String];

impl From<Text> for Value {
    fn from(v: Text) -> Self {
        Value::String(Box::new(v))
    }
}

impl Text {
    /// Set handler
    pub fn set_handler(&mut self, handler: impl Into<String>) {
        self.handler = Some(handler.into())
    }
    /// Get handler
    pub fn get_handler(&self) -> Option<String> {
        self.handler.to_owned()
    }
    /// Get the &str
    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }
}

impl Text {
    /// Check if string is empty
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}
