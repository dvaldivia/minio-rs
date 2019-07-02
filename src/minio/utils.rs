use regex::Regex;
use serde_xml_rs::Error as SError;
use std::error::Error;
use std::fmt;
use std::io::Read;

use lazy_static::lazy_static;

use crate::minio::types::Err;

#[derive(Debug)]
struct UtilsError {
    details: String,
}

impl UtilsError {
    fn new(msg: &str) -> UtilsError {
        UtilsError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for UtilsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for UtilsError {
    fn description(&self) -> &str {
        &self.details
    }
}

// xml_decoder provide decoded value in xml.
pub fn xml_decoder<T>(body: String, t: &mut T) -> Result<(), Error> {
    let t: T = match serde_xml_rs::from_str(&body) {
        Ok(v) => v,
        SError(e) => Err(e),
    };
}

// CheckValidBucketName - checks if we have a valid input bucket name.
pub fn check_valid_bucket_name(bucket_name: &String) -> Result<(), Error> {
    return check_bucket_name_common(bucket_name, false);
}

// Common checker for both stricter and basic validation.
fn check_bucket_name_common(bucket_name: &String, strict: bool) -> Result<(), Error> {
    lazy_static! {
        static ref VALID_BUCKET_NAME_RE: Regex =
            Regex::new(r"^[A-Za-z0-9][A-Za-z0-9\.\-\_\:]{1,61}[A-Za-z0-9]$").unwrap();
        static ref VALID_BUCKET_NAME_STRICT_RE: Regex =
            Regex::new(r"^[a-z0-9][a-z0-9\.\-]{1,61}[a-z0-9]$").unwrap();
        static ref IP_ADDRESS_RE: Regex = Regex::new(r"^(\d+\.){3}\d+$").unwrap();
    };

    if bucket_name.trim() == "" {
        Err(UtilsError::new("Bucket name cannot be empty"))
    }
    if bucket_name.len() < 3 {
        Err(UtilsError::new(
            "Bucket name cannot be smaller than 3 characters",
        ))
    }
    if bucket_name.len() > 63 {
        Err(UtilsError::new(
            "Bucket name cannot be greater than 63 characters",
        ))
    }
    if IP_ADDRESS_RE.is_match(bucket_name) {
        Err(UtilsError::new("Bucket name cannot be an ip address"))
    }
    if bucket_name.contains("..") || bucket_name.contains(".-") || bucket_name.contains("-.") {
        Err(UtilsError::new("Bucket name contains invalid characters"))
    }
    if strict {
        if !VALID_BUCKET_NAME_STRICT_RE.is_match(bucket_name) {
            Err(UtilsError::new("Bucket name contains invalid characters"))
        }
    } else {
        if !VALID_BUCKET_NAME_RE.is_match(bucket_name) {
            Err(UtilsError::new("Bucket name contains invalid characters"))
        }
    }
}

// check_valid_object_name_prefix - checks if we have a valid input object name prefix.
//   - http://docs.aws.amazon.com/AmazonS3/latest/dev/UsingMetadata.html
pub fn check_valid_object_name_prefix(object_name: &String) -> Result<(), Error> {
    if object_name.len() > 1024 {
        Err(UtilsError::new(
            "Object name cannot be greater than 1024 characters",
        ))
    }

    match str::from_utf8(&object_name) {
        Err(_) => Err(UtilsError::new(
            "Object name with non UTF-8 strings are not supported",
        )),
        _ => (),
    }
}

// CheckValidObjectName - checks if we have a valid input object name.
//   - http://docs.aws.amazon.com/AmazonS3/latest/dev/UsingMetadata.html
pub fn check_valid_object_name(object_name: &String) -> Result<(), Error> {
    if object_name.trim() == "" {
        Err(UtilsError::new("Object name cannot be empty"))
    }
    return check_valid_object_name_prefix(object_name);
}
