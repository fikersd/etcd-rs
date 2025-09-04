use crate::ResponseHeader;
use crate::proto::etcdserverpb;

#[derive(Debug, Default, Clone)]
pub struct AuthEnableRequest {
    proto: etcdserverpb::AuthEnableRequest,
}

impl From<AuthEnableRequest> for etcdserverpb::AuthEnableRequest {
    fn from(req: AuthEnableRequest) -> Self {
        req.proto
    }
}

#[derive(Debug, Clone)]
pub struct AuthEnableResponse {
    pub header: ResponseHeader,
}

impl From<etcdserverpb::AuthEnableResponse> for AuthEnableResponse {
    fn from(proto: etcdserverpb::AuthEnableResponse) -> Self {
        Self {
            header: From::from(proto.header.expect("must fetch header")),
        }
    }
}
