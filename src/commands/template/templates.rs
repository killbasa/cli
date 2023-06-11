use clap::ValueEnum;
use std::fmt::{Display, Formatter, Result};

#[derive(ValueEnum, Clone, Debug)]
pub enum Templates {
    NodeTS,
}

impl Display for Templates {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Templates::NodeTS => write!(f, "node-ts"),
        }
    }
}
