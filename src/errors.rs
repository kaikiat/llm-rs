use std::convert::From;
use std::error;
use std::fmt;
use llm_chain::traits::ExecutorCreationError;

#[derive(Debug)]
pub enum Error {
    CustomInput(String),
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::CustomInput(ref msg) => write!(f, "{}", msg),
        }
    }
}

impl From<ExecutorCreationError> for Error {
    fn from(err: ExecutorCreationError) -> Self {
        // Convert the specific error type to your custom error type.
        Error::CustomInput(format!("an error occured: {:?}", err))
    }
}