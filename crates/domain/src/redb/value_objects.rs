use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RedbKey(String);

impl RedbKey {
    pub fn new(key: String) -> Self {
        Self(key)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RedbValue(serde_json::Value);

impl RedbValue {
    pub fn new(value: serde_json::Value) -> Self {
        Self(value)
    }

    pub fn as_json(&self) -> &serde_json::Value {
        &self.0
    }

    pub fn into_inner(self) -> serde_json::Value {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum KeyOperation {
    Insert,
    Update,
    Delete,
}

impl std::fmt::Display for KeyOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyOperation::Insert => write!(f, "insert"),
            KeyOperation::Update => write!(f, "update"),
            KeyOperation::Delete => write!(f, "delete"),
        }
    }
}