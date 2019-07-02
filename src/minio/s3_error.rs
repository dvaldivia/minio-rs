use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref S3_ERROR_RESPONSE_MAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("AccessDenied", "Access Denied.");
        m.insert(
            "BadDigest",
            "The Content-Md5 you specified did not match what we received.",
        );
        m.insert(
            "EntityTooSmall",
            "Your proposed upload is smaller than the minimum allowed object size.",
        );
        m.insert(
            "EntityTooLarge",
            "Your proposed upload exceeds the maximum allowed object size.",
        );
        m.insert(
            "IncompleteBody",
            "You did not provide the number of bytes specified by the Content-Length HTTP header.",
        );
        m.insert(
            "InternalError",
            "We encountered an internal error, please try again.",
        );
        m.insert(
            "InvalidAccessKeyId",
            "The access key ID you provided does not exist in our records.",
        );
        m.insert("InvalidBucketName", "The specified bucket is not valid.");
        m.insert(
            "InvalidDigest",
            "The Content-Md5 you specified is not valid.",
        );
        m.insert("InvalidRange", "The requested range is not satisfiable");
        m.insert("MalformedXML", "The XML you provided was not well-formed or did not validate against our published schema.");
        m.insert(
            "MissingContentLength",
            "You must provide the Content-Length HTTP header.",
        );
        m.insert(
            "MissingContentMD5",
            "Missing required header for this request: Content-Md5.",
        );
        m.insert("MissingRequestBodyError", "Request body is empty.");
        m.insert("NoSuchBucket", "The specified bucket does not exist.");
        m.insert("NoSuchBucketPolicy", "The bucket policy does not exist");
        m.insert("NoSuchKey", "The specified key does not exist.");
        m.insert("NoSuchUpload", "The specified multipart upload does not exist. The upload ID may be invalid, or the upload may have been aborted or completed.");
        m.insert(
            "NotImplemented",
            "A header you provided implies functionality that is not implemented",
        );
        m.insert(
            "PreconditionFailed",
            "At least one of the pre-conditions you specified did not hold",
        );
        m.insert(
            "RequestTimeTooSkewed",
            "The difference between the request time and the server's time is too large.",
        );
        m.insert("SignatureDoesNotMatch", "The request signature we calculated does not match the signature you provided. Check your key and signing method.");
        m.insert(
            "MethodNotAllowed",
            "The specified method is not allowed against this resource.",
        );
        m.insert(
            "InvalidPart",
            "One or more of the specified parts could not be found.",
        );
        m.insert("InvalidPartOrder", "The list of parts was not in ascending order. The parts list must be specified in order by part number.");
        m.insert(
            "InvalidObjectState",
            "The operation is not valid for the current state of the object.",
        );
        m.insert(
            "AuthorizationHeaderMalformed",
            "The authorization header is malformed; the region is wrong.",
        );
        m.insert(
            "MalformedPOSTRequest",
            "The body of your POST request is not well-formed multipart/form-data.",
        );
        m.insert(
            "BucketNotEmpty",
            "The bucket you tried to delete is not empty",
        );
        m.insert(
            "AllAccessDisabled",
            "All access to this bucket has been disabled.",
        );
        m.insert("MalformedPolicy", "Policy has invalid resource.");
        m.insert("MissingFields", "Missing fields in request.");
        m.insert("AuthorizationQueryParametersError", "Error parsing the X-Amz-Credential parameter; the Credential is mal-formed; expecting \"<YOUR-AKID>/YYYYMMDD/REGION/SERVICE/aws4_request\".");
        m.insert("MalformedDate", "Invalid date format header, expected to be in ISO8601, RFC1123 or RFC1123Z time format.");
        m.insert(
            "BucketAlreadyOwnedByYou",
            "Your previous request to create the named bucket succeeded and you already own it.",
        );
        m.insert(
            "InvalidDuration",
            "Duration provided in the request is invalid.",
        );
        m.insert(
            "XAmzContentSHA256Mismatch",
            "The provided 'x-amz-content-sha256' header does not match what was computed.",
        );
        m
    };
}
