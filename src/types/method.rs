#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum HttpRequestMethod {
    GET,
    HEAD,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTIONS,
    CONNECT,
    TRACE,
    UNKNOWN,
}
