use std::collections::HashMap;

pub struct Request {
    url: String,
    headers: HashMap<String, String>,
    req_type: RequestType,
}

pub enum RequestType {
    GET,
    PUT,
    POST,
}

pub trait RequestBuilder {
    fn build(self) -> Request;
    // Accepting `self` and returning `Self` (which are moves) preclude this trait from being
    // object safe and thus the target of dynamic dispatch.
    fn with_url(self, url: impl Into<String>) -> Self;
    fn with_header(self, header: impl Into<String>, value: impl Into<String>) -> Self;
}

pub struct GetRequestBuilder {
    req: Request,
}

impl GetRequestBuilder {
    fn new() -> Self {
        Self {
            req: Request {
                url: "".to_string(),
                headers: HashMap::new(),
                req_type: RequestType::GET,
            },
        }
    }
}

impl RequestBuilder for GetRequestBuilder {
    fn build(self) -> Request {
        self.req
    }

    fn with_url(mut self, url: impl Into<String>) -> Self {
        self.req.url = url.into();
        self
    }

    fn with_header(mut self, header: impl Into<String>, value: impl Into<String>) -> Self {
        self.req.headers.insert(header.into(), value.into());
        self
    }
}
