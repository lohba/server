Use std:: str::FromStr;

pub enum Method {
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl