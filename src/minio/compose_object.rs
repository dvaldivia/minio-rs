use std::collections::HashMap;

use crate::minio::constants::MAX_PARTS_COUNT;
use crate::minio::encrypt::server_side::ServerSide;
use crate::minio::error_response::ErrInvalidArgument;
use crate::minio::get_options::StatObjectOptions;
use crate::minio::types::ObjectInfo;
use crate::minio::Client;
use futures::Future;
use hyper::HeaderMap;
use std::error::Error;

// DestinationInfo - type with information about the object to be
// created via server-side copy requests, using the Compose API.
struct DestinationInfo<T>
where
    T: ServerSide,
{
    bucket: String,
    object: String,
    encryption: T,

    // if no user-metadata is provided, it is copied from source
    // (when there is only once source object in the compose
    // request)
    user_metadata: HashMap<String, String>,
}

// SourceInfo - represents a source object to be copied, using
// server-side copying APIs.
struct SourceInfo<T>
where
    T: ServerSide,
{
    bucket: String,
    object: String,
    start: i64,
    end: i64,
    encryption: T,
    // Headers to send with the upload-part-copy request involving
    // this source object.
    pub headers: HeaderMap,
}

impl<T> SourceInfo<T>
where
    T: ServerSide,
{
    //    fn get_props(&self, c: &Client) -> impl Future<Item=(),Error=Error> {
    //        // Get object info - need size and etag here. Also, decryption
    //        // headers are added to the stat request if given.
    //        let obj_inf:ObjectInfo;
    //        let opts = StatObjectOptions::GetObjectOptions({ServerSideEncryption: encrypt.SSE(s.encryption)});
    //        c.stat_object( &self.bucket, &self.object, opts).then(|res| {
    //
    //
    //            //	err = ErrInvalidArgument(fmt.Sprintf("Could not stat object - %s/%s: %v", s.bucket, s.object, err))
    //            //} else {
    //            //	size = objInfo.Size
    //            //	etag = objInfo.ETag
    //            //	userMeta = make(map[string]string)
    //            //	for k, v := range objInfo.Metadata {
    //            //		if strings.HasPrefix(k, "x-amz-meta-") {
    //            //			if len(v) > 0 {
    //            //				userMeta[k] = v[0]
    //            //			}
    //            //		}
    //            //	}
    //            //}
    //            //return
    //
    //        })
    //
    //    }
}

impl Client {
    //    fn compose_object_with_progress(dst: DestinationInfo, srcs: Vec<SourceInfo>) -> ErrorResponse {
    //        if srcs.len() < 1 || srcs.len() > MAX_PARTS_COUNT {
    //            return ErrInvalidArgument(
    //                "There must be as least one and up to 10000 source objects.",
    //            );
    //        }
    //        let mut src_sizes : Vec<u64> = Vec::with_capacity(srcs.len());
    //        let mut  totalSize:u64;
    //        let mut size:u64;
    //        let mut totalParts:u64;
    //        let mut src_user_meta :HashMap<String,String> = HashMap::new();
    //        let mut etags :Vec<String> = Vec::with_capacity(srcs.len());
    //        let err: Error;
    //        for i in 0..srcs.len() {
    //            let src = srcs[i];
    //            let (size, etags[i], srcUserMeta, err) = src.getProps(c);
    //            if err != nil {
    //                return err
    //            }
    //
    //        // Error out if client side encryption is used in this source object when
    //        // more than one source objects are given.
    //        if len(srcs) > 1 && src.Headers.Get("x-amz-meta-x-amz-key") != "" {
    //            return ErrInvalidArgument(
    //                fmt.Sprintf("Client side encryption is used in source object %s/%s", src.bucket, src.object))
    //        }
    //
    //        // Check if a segment is specified, and if so, is the
    //        // segment within object bounds?
    //        if src.start != -1 {
    //            // Since range is specified,
    //            //    0 <= src.start <= src.end
    //            // so only invalid case to check is:
    //            if src.end >= size {
    //                return ErrInvalidArgument(
    //                    fmt.Sprintf("SourceInfo %d has invalid segment-to-copy [%d, %d] (size is %d)",
    //                                i, src.start, src.end, size))
    //            }
    //            size = src.end - src.start + 1
    //        }
    //
    //        // Only the last source may be less than `absMinPartSize`
    //        if size < absMinPartSize && i < len(srcs)-1 {
    //            return ErrInvalidArgument(
    //                fmt.Sprintf("SourceInfo %d is too small (%d) and it is not the last part", i, size))
    //        }
    //
    //        // Is data to copy too large?
    //        totalSize += size
    //        if totalSize > maxMultipartPutObjectSize {
    //            return ErrInvalidArgument(fmt.Sprintf("Cannot compose an object of size %d (> 5TiB)", totalSize))
    //        }
    //
    //        // record source size
    //        srcSizes[i] = size
    //
    //        // calculate parts needed for current source
    //        totalParts += partsRequired(size)
    //        // Do we need more parts than we are allowed?
    //        if totalParts > maxPartsCount {
    //            return ErrInvalidArgument(fmt.Sprintf(
    //                "Your proposed compose object requires more than %d parts", maxPartsCount))
    //        }
    //    }
    //    }
}
