use http::StatusCode;

use crate::application::ApplicationError;
use crate::response::ResponseError;

impl ResponseError for ApplicationError {
    fn response_status(&self) -> (hyper::StatusCode, String) {
        use crate::application::ApplicationError::*;

        let not_found = StatusCode::NOT_FOUND;
        let bad_request = StatusCode::BAD_REQUEST;
        let internal_server_error = StatusCode::INTERNAL_SERVER_ERROR;

        match self {
            Http(http_error) => {
                use crate::http::HttpError::*;

                match http_error {
                    RouteNotFound => (not_found, "Route not found".into()),
                    NoHost => (bad_request, "No host found".into()),
                    InvalidHostValue => (bad_request, "Invalid host found".into()),
                }
            }
            Persistence(persistence_error) => {
                use crate::persistence::PersistenceError::*;

                match persistence_error {
                    NoResult => (not_found, "Result not found".into()),
                    FailedToFetch => (internal_server_error, "Whoops! Something went wrong".into()),
                    BadQuery => (internal_server_error, "Whoops! Something went wrong".into()),
                }
            }
        }
    }
}