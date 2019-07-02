use crate::minio::encrypt::server_side::ServerSide;
use std::collections::HashMap;

// GetObjectOptions are used to specify additional headers or options
// during GET requests.
pub struct GetObjectOptions<T>
where
    T: ServerSide,
{
    headers: HashMap<String, String>,
    pub server_side_encryption: T,
}

// StatObjectOptions are used to specify additional headers or options
// during GET info/stat requests.
pub enum StatObjectOptions<T>
where
    T: ServerSide,
{
    GetObjectOptions(GetObjectOptions<T>),
}
