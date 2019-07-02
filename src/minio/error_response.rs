/* **** SAMPLE ERROR RESPONSE ****
<?xml version="1.0" encoding="UTF-8"?>
<Error>
   <code>AccessDenied</code>
   <message>Access Denied</message>
   <bucket_name>bucket_name</bucket_name>
   <key>object_name</key>
   <RequestId>F19772218238A85A</RequestId>
   <HostId>GuWkjyviSiGHizehqpmsD1ndz5NClSP19DOT+s2mv7gXGQ8/X1lhbDGiIJEXpGFD</HostId>
</Error>
*/

use crate::minio::s3_error::S3_ERROR_RESPONSE_MAP;
use crate::minio::types::ObjectInfo;
use crate::minio::utils::xml_decoder;
use futures::stream::Stream;
use futures::Future;
use hyper::{Body, Response, StatusCode};
use std::error::Error;

// ErrorResponse - Is the typed error returned by all API operations.
// ErrorResponse struct should be comparable since it is compared inside
// golang http API (https://github.com/golang/go/issues/29768)
struct ErrorResponse {
    pub xml_name: Option<String>,
    // xml.Name// `xml:"Error" json:"-"`
    pub code: Option<String>,
    pub message: Option<String>,
    pub bucket_name: Option<String>,
    pub key: Option<String>,
    pub request_id: Option<String>,
    //`xml:"RequestId"`
    pub host_id: Option<String>, // `xml:"HostId"`

    // Region where the bucket is located. This header is returned
    // only in HEAD bucket and ListObjects response.
    pub region: Option<String>,

    // Underlying HTTP status code for the returned error
    pub status_code: u16, // `xml:"-" json:"-"`
}

// ErrInvalidArgument - Invalid argument response.
pub fn ErrInvalidArgument(message: &str) -> ErrorResponse {
    return ErrorResponse {
        xml_name: None,
        status_code: StatusCode::BAD_REQUEST,
        code: Some("InvalidArgument".to_string()),
        message: Some(message.to_string()),
        request_id: Some("minio".to_string()),
        ..Default::default()
    };
}

const REPORT_ISSUE: &str = "Please report this issue at https://github.com/minio/minio-go/issues.";

// httpRespToErrorResponse returns a new encoded ErrorResponse
// structure as error.
pub fn http_resp_to_error_response(
    resp: &Response<Body>,
    bucket_name: String,
    object_name: String,
) -> impl Future<Item = (), Error = Error> {
    if resp.body().is_empty() {
        let msg = "Response is empty. " + REPORT_ISSUE;
        return ErrInvalidArgument(msg);
    }

    let mut err_resp = ErrorResponse {
        status_code: resp.status(),
        ..Default::default()
    };

    Box::new(
        resp.into_body()
            .concat2() // Concatenate all chunks in the body
            .from_err()
            .and_then(move |entire_body| {
                let payload: String = match String::from_utf8(entire_body.to_vec()) {
                    Ok(str) => str,
                    Err(err) => panic!("Couldn't convert buffer to string: {}", err),
                };

                match xml_decoder(payload, &mut err_resp) {
                    Ok(_) => (),
                    Err(e) => {
                        // Xml decoding failed with no body, fall back to HTTP headers.
                        let err = match resp.status() {
                            StatusCode::NOT_FOUND => {
                                if object_name == "" {
                                    err_resp = ErrorResponse {
                                        status_code: resp.status(),
                                        code: Some("NoSuchBucket".to_string()),
                                        message: Some(
                                            "The specified bucket does not exist.".to_string(),
                                        ),
                                        bucket_name: Some(bucket_name.clone()),
                                        ..Default::default()
                                    }
                                } else {
                                    err_resp = ErrorResponse {
                                        status_code: Some(resp.status_code),
                                        code: Some("NoSuchkey".to_string()),
                                        message: Some(
                                            "The specified key does not exist.".to_string(),
                                        ),
                                        bucket_name: Some(bucket_name.clone()),
                                        key: Some(object_name.clone()),
                                        ..Default::default()
                                    }
                                }
                            }
                            StatusCode::FORBIDDEN => {
                                err_resp = ErrorResponse {
                                    status_code: Some(resp.status_code),
                                    code: Some("AccessDenied".to_string()),
                                    message: Some("Access Denied.".to_string()),
                                    bucket_name: Some(bucket_name.clone()),
                                    key: Some(object_name.clone()),
                                    ..Default::default()
                                }
                            }
                            StatusCode::CONFLICT => {
                                err_resp = ErrorResponse {
                                    status_code: Some(resp.status_code),
                                    code: Some("Conflict".to_string()),
                                    message: Some("Bucket not empty.".to_string()),
                                    bucket_name: Some(bucket_name.clone()),
                                    ..Default::default()
                                }
                            }
                            StatusCode::PRECONDITION_FAILED => {
                                err_resp = ErrorResponse {
                                    status_code: Some(resp.status_code),
                                    code: Some("PreconditionFailed".to_string()),
                                    message: Some(S3_ERROR_RESPONSE_MAP.get("PreconditionFailed")),
                                    bucket_name: Some(bucket_name.clone()),
                                    key: Some(object_name.clone()),
                                    ..Default::default()
                                }
                            }
                            _ => {
                                err_resp = ErrorResponse {
                                    status_code: Some(resp.status_code),
                                    code: Some(resp.Status),
                                    message: Some(resp.Status),
                                    bucket_name: Some(bucket_name.clone()),
                                    ..Default::default()
                                }
                            }
                        };
                    }
                };

                // Save hostID, requestID and region information
                // from headers if not available through error XML.
                if err_resp.request_id.is_none() {
                    err_resp.region = resp.headers().get("x-amz-request-id").into();
                }
                if err_resp.host_id.is_none() {
                    err_resp.host_id = resp.headers().get("x-amz-id-2").into();
                }
                if err_resp.region.is_none() {
                    err_resp.region = resp.headers().get("x-amz-bucket-region").into();
                }
                if err_resp.code == "InvalidRegion" && err_resp.region.is_some() {
                    err_resp.message = Some(format!(
                        "Region does not match, expecting region ‘{}’.",
                        err_resp.region.unwrap()
                    ))
                }

                futures::future::err(err_resp);
            }),
    )
}
