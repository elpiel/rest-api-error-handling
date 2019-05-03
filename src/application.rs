use crate::persistence::PersistenceError;
use crate::http::HttpError;

pub enum ApplicationError {
    Persistence(PersistenceError),
    Http(HttpError),
}

impl Into<ApplicationError> for PersistenceError {
    fn into(self) -> ApplicationError {
        ApplicationError::Persistence(self)
    }
}

impl Into<ApplicationError> for HttpError {
    fn into(self) -> ApplicationError {
        ApplicationError::Http(self)
    }
}