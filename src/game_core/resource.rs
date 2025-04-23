use std::fmt;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "PascalCase")]
pub enum Resource {
    Minerals,
    Gas,
    Energy,
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Resource::Minerals => write!(f, "Minerals"),
            Resource::Gas => write!(f, "Gas"),
            Resource::Energy => write!(f, "Energy"),
        }
    }
}
