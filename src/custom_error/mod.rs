use std::fmt;

#[derive(Debug)]
pub struct Error {
    kind: Kind,
    message: String,
}

impl Error {
    pub fn new(kind: Kind, message: String) -> Error {
        Error { kind, message }
    }

    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn kind(&self) -> &Kind {
        &self.kind
    }
}

#[derive(Debug)]
pub enum Kind {
    ConnectionError,
    RepositoryError,
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self.kind {
            Kind::ConnectionError => "Error connecting to database",
            Kind::RepositoryError => "Error querying database", 
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            Kind::ConnectionError => write!(f, "{}", self.message()),
            Kind::RepositoryError => write!(f, "{}", self.message()),
        }   
    }
}