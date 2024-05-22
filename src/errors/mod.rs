use std::fmt::{Display, Formatter};

pub enum ConnectionError{
    CreateServerError(String)
}

impl Display for ConnectionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            ConnectionError::CreateServerError(ref desc) => {
                write!(f, "Error while creating server. {}", desc)
            }
        }
    }
}

