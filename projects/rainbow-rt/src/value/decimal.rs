use super::*;

/// ```note
/// 0.0
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Decimal {
    handler: Option<String>,
    value: BigDecimal,
}

impl Display for Decimal {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.value)?;
        match &self.handler {
            None => (),
            Some(s) => write!(f, "{}", s)?,
        }
        Ok(())
    }
}

impl Deref for Decimal {
    type Target = BigDecimal;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl From<f32> for Decimal {
    fn from(v: f32) -> Self {
        Self { handler: None, value: BigDecimal::try_from(v).unwrap_or_default() }
    }
}

impl From<f32> for Value {
    fn from(v: f32) -> Self {
        match BigDecimal::try_from(v) {
            Ok(v) => Value::from(v),
            Err(_) => Value::Null,
        }
    }
}

impl From<f64> for Decimal {
    fn from(v: f64) -> Self {
        Self { handler: None, value: BigDecimal::try_from(v).unwrap_or_default() }
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        match BigDecimal::try_from(v) {
            Ok(v) => Value::from(v),
            Err(_) => Value::Null,
        }
    }
}

impl From<BigDecimal> for Decimal {
    fn from(v: BigDecimal) -> Self {
        Self { handler: None, value: v }
    }
}

impl From<BigDecimal> for Value {
    fn from(v: BigDecimal) -> Self {
        Value::Decimal(Box::new(v.into()))
    }
}

impl From<Decimal> for Value {
    fn from(v: Decimal) -> Self {
        Self::Decimal(Box::new(v))
    }
}

impl Decimal {
    ///
    pub fn set_handler(&mut self, handler: impl Into<String>) {
        self.handler = Some(handler.into())
    }
    ///
    pub fn get_handler(&self) -> Option<String> {
        self.handler.to_owned()
    }
}
