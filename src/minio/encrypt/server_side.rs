use hyper::HeaderMap;

pub type Type = String;

// ServerSide is a form of S3 server-side-encryption.
pub trait ServerSide {
    // Type returns the server-side-encryption method.
    fn get_type() -> Type;
    // Marshal adds encryption headers to the provided HTTP headers.
    // It marks an HTTP request as server-side-encryption request
    // and inserts the required data into the headers.
    fn marshal(h: &HeaderMap);
}
