use std::fmt::Display;

/// Ast of a parsed query
#[derive(Debug, Clone, PartialEq)]
pub enum Q3Ast {
    /// An Id of type `#{id1}`
    Id(String),
    /// Anything except an Id
    Other(String),
}

impl Display for Q3Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Id(id) => {
                write!(f, "{}", id)
            }
            Self::Other(other) => {
                write!(f, "{}", other)
            }
        }
    }
}
