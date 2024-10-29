use serde_json::Value;

/// Content enumeration that stores all content types in a single storage
#[derive(Clone, Default)]
pub enum Content {
    /// Stores [`String`] typed content
    String(String),
    /// Stores [`Value`] typed content
    JSON(Value),
    /// Empty content type
    #[default]
    Empty,
}

impl From<String> for Content {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for Content {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<Value> for Content {
    fn from(value: Value) -> Self {
        Self::JSON(value)
    }
}

impl Content {
    /// Returns content value as a [`String`]
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::enums::content::Content;
    /// use serde_json::json;
    ///
    /// let json = json!({"foo": "bar"});
    /// let content = Content::from(json);
    /// let empty = Content::default();
    ///
    /// assert_eq!(content.as_string(), "{\"foo\":\"bar\"}");
    /// assert_eq!(empty.as_string(), "");
    /// ```
    pub fn as_string(self) -> String {
        match self {
            Self::String(content) => content,
            Self::JSON(json) => json.to_string(),
            Self::Empty => String::new(),
        }
    }
    /// Returns content value as a [`Value`]
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::enums::content::Content;
    /// use serde_json::json;
    ///
    /// let json = json!({"foo": "bar"});
    /// let content = Content::from("{\"foo\":\"bar\"}");
    ///
    /// assert_eq!(content.as_json(), json);
    /// ```
    pub fn as_json(self) -> Value {
        match self {
            Self::String(content) => serde_json::from_str(&content).unwrap_or_default(),
            Self::JSON(json) => json,
            Self::Empty => Value::Null,
        }
    }
    /// Returns [`true`] if content is empty otherwise [`false`]
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::enums::content::Content;
    /// use serde_json::Value;
    ///
    /// let empty = Content::Empty;
    /// let non_empty = Content::from("foo");
    ///
    /// assert!(empty.is_empty());
    /// assert!(!non_empty.is_empty());
    ///
    /// assert!(Content::from("").is_empty());
    /// assert!(Content::from(Value::Null).is_empty());
    /// ```
    pub fn is_empty(self) -> bool {
        match self {
            Self::String(content) => content.is_empty(),
            Self::JSON(json) => json == Value::Null,
            Self::Empty => true,
        }
    }
}
