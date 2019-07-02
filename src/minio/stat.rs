use crate::minio::encrypt::server_side::ServerSide;
use crate::minio::error_response::http_resp_to_error_response;
use crate::minio::get_options::StatObjectOptions;
use crate::minio::net::{Values, ValuesAccess};
use crate::minio::types::ObjectInfo;
use crate::minio::{utils, Client, S3Req};
use futures::Future;
use hyper::{Body, HeaderMap, Method, StatusCode};
use std::error::Error;

impl Client {
    // Lower level API for statObject supporting pre-conditions and range headers.
    pub fn stat_object<T: ServerSide>(
        &self,
        bucket_name: &String,
        object_name: &String,
        opts: StatObjectOptions<T>,
    ) -> impl Future<Item = ObjectInfo, Error = Error> {
        // Input validation.
        match utils::check_valid_bucket_name(bucket_name) {
            Err(e) => return Err(e),
            _ => (),
        }
        match utils::check_valid_object_name(object_name) {
            Err(e) => return Err(e),
            _ => (),
        }

        let s3_req = S3Req {
            method: Method::HEAD,
            bucket: Some(bucket_name.to_string()),
            object: None,
            headers: HeaderMap::new(),
            query: Values::new(),
            body: Body::empty(),
            ts: time::now_utc(),
        };
        self.signed_req_future(s3_req, Ok(Body::empty()))
            .then(|response_result| {
                let resp = match response_result {
                    Err(e) => return futures::future::err(e),
                    Ok(r) => r,
                };
                if resp.status() != StatusCode::OK && resp.status() != StatusCode::PARTIAL_CONTENT {
                    return futures::future::err(http_resp_to_error_response(
                        &resp,
                        &bucket_name,
                        &object_name,
                    ));
                }
            })

        //if resp != nil {
        //	if resp.StatusCode != http.StatusOK && resp.StatusCode != http.StatusPartialContent {
        //		return ObjectInfo{}, httpRespToErrorResponse(resp, bucketName, objectName)
        //	}
        //}
        //
        //// Trim off the odd double quotes from ETag in the beginning and end.
        //md5sum := strings.TrimPrefix(resp.Header.Get("ETag"), "\"")
        //md5sum = strings.TrimSuffix(md5sum, "\"")
        //
        //// Parse content length is exists
        //var size int64 = -1
        //contentLengthStr := resp.Header.Get("Content-Length")
        //if contentLengthStr != "" {
        //	size, err = strconv.ParseInt(contentLengthStr, 10, 64)
        //	if err != nil {
        //		// Content-Length is not valid
        //		return ObjectInfo{}, ErrorResponse{
        //			Code:       "InternalError",
        //			Message:    "Content-Length is invalid. " + reportIssue,
        //			BucketName: bucketName,
        //			Key:        objectName,
        //			RequestID:  resp.Header.Get("x-amz-request-id"),
        //			HostID:     resp.Header.Get("x-amz-id-2"),
        //			Region:     resp.Header.Get("x-amz-bucket-region"),
        //		}
        //	}
        //}
        //
        //// Parse Last-Modified has http time format.
        //date, err := time.Parse(http.TimeFormat, resp.Header.Get("Last-Modified"))
        //if err != nil {
        //	return ObjectInfo{}, ErrorResponse{
        //		Code:       "InternalError",
        //		Message:    "Last-Modified time format is invalid. " + reportIssue,
        //		BucketName: bucketName,
        //		Key:        objectName,
        //		RequestID:  resp.Header.Get("x-amz-request-id"),
        //		HostID:     resp.Header.Get("x-amz-id-2"),
        //		Region:     resp.Header.Get("x-amz-bucket-region"),
        //	}
        //}
        //
        //// Fetch content type if any present.
        //contentType := strings.TrimSpace(resp.Header.Get("Content-Type"))
        //if contentType == "" {
        //	contentType = "application/octet-stream"
        //}
        //
        //expiryStr := resp.Header.Get("Expires")
        //var expTime time.Time
        //if t, err := time.Parse(http.TimeFormat, expiryStr); err == nil {
        //	expTime = t.UTC()
        //}
        //// Save object metadata info.
        //return ObjectInfo{
        //	ETag:         md5sum,
        //	Key:          objectName,
        //	Size:         size,
        //	LastModified: date,
        //	ContentType:  contentType,
        //	Expires:      expTime,
        //	// Extract only the relevant header keys describing the object.
        //	// following function filters out a list of standard set of keys
        //	// which are not part of object metadata.
        //	Metadata: extractObjMetadata(resp.Header),
        //}, nil
    }
}
