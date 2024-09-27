use serde_json::Value;

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Content {
    String(String),
    JSON(Value),
    #[default]
    Empty,
}

impl Content {
    pub fn to_json(&self) -> Value {
        match self {
            Self::String(str) => serde_json::from_str(str).unwrap_or_default(),
            Self::JSON(json) => json.clone(),
            Self::Empty => Value::default(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::JSON(json) => json == &Value::Null,
            Self::String(str) => str.is_empty(),
            Self::Empty => true,
        }
    }
}

impl From<String> for Content {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for Content {
    fn to_string(&self) -> String {
        match self {
            Self::String(str) => str.clone(),
            Self::JSON(json) => json.to_string(),
            Self::Empty => String::new(),
        }
    }
}
