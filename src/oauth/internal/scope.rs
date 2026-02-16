use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Scope {
    pub name: String,
    pub description: Option<String>,
    pub claims: Vec<Claim>,
}

impl Scope {
    pub fn new(name: String, description: Option<String>, claims: Vec<Claim>) -> Self {
        Self { name, description, claims }
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }
    pub fn get_claims(&self) -> &[Claim] {
        &self.claims
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Claim {
    name: String,
    description: Option<String>,
}

impl Claim {
    pub fn new(name: String, description: Option<String>) -> Self {
        Self { name, description }
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

pub struct ValidClaims {
    claims: HashMap<String, Claim>,
}