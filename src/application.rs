use crate::persistence::PersistenceError;
use crate::http::HttpError;

pub enum ApplicationError {
    Persistence(PersistenceError),
    Http(HttpError),
}