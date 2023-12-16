use clap::ValueEnum;
use std::fmt::{Display, Formatter, Result};

#[derive(ValueEnum, Clone, Debug)]
pub enum Languages {
    Node,
    Rust,
}

impl Display for Languages {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Languages::Node => write!(f, "node"),
            Languages::Rust => write!(f, "rust"),
        }
    }
}
