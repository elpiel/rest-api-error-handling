use hyper::Body;

pub mod response_impl;

pub trait ResponseError {
    fn response_status(&self) -> (hyper::StatusCode, String);

    fn as_response(&self) -> Result<hyper::Response<hyper::Body>, http::Error> {
        hyper::Response::builder()
            .status(self.response_status().0)
            .body(self.response_status().1.into())
    }
}