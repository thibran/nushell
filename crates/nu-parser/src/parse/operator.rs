use nu_source::{b, DebugDocBuilder, PrettyDebug};
use serde::{Deserialize, Serialize};

use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum Operator {
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Dot,
    Contains,
    NotContains,
}

impl PrettyDebug for Operator {
    fn pretty(&self) -> DebugDocBuilder {
        b::operator(self.as_str())
    }
}

impl Operator {
    pub fn print(&self) -> String {
        self.as_str().to_string()
    }

    pub fn as_str(&self) -> &str {
        match *self {
            Operator::Equal => "==",
            Operator::NotEqual => "!=",
            Operator::LessThan => "<",
            Operator::GreaterThan => ">",
            Operator::LessThanOrEqual => "<=",
            Operator::GreaterThanOrEqual => ">=",
            Operator::Dot => ".",
            Operator::Contains => "=~",
            Operator::NotContains => "!~",
        }
    }
}

impl From<&str> for Operator {
    fn from(input: &str) -> Operator {
        Operator::from_str(input).unwrap()
    }
}

impl FromStr for Operator {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, <Self as std::str::FromStr>::Err> {
        match input {
            "==" => Ok(Operator::Equal),
            "!=" => Ok(Operator::NotEqual),
            "<" => Ok(Operator::LessThan),
            ">" => Ok(Operator::GreaterThan),
            "<=" => Ok(Operator::LessThanOrEqual),
            ">=" => Ok(Operator::GreaterThanOrEqual),
            "." => Ok(Operator::Dot),
            "=~" => Ok(Operator::Contains),
            "!~" => Ok(Operator::NotContains),
            _ => Err(()),
        }
    }
}
