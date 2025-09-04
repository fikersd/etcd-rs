use crate::ResponseHeader;
use crate::proto::etcdserverpb;

#[derive(Debug, Clone)]
pub struct AuthRoleAddRequest {
    proto: etcdserverpb::AuthRoleAddRequest,
}

impl AuthRoleAddRequest {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            proto: etcdserverpb::AuthRoleAddRequest { name: name.into() },
        }
    }
}

impl<I> From<I> for AuthRoleAddRequest
where
    I: Into<String>,
{
    fn from(name: I) -> Self {
        Self::new(name)
    }
}

impl From<AuthRoleAddRequest> for etcdserverpb::AuthRoleAddRequest {
    fn from(req: AuthRoleAddRequest) -> Self {
        req.proto
    }
}

#[derive(Debug, Clone)]
pub struct AuthRoleAddResponse {
    pub header: ResponseHeader,
}

impl From<etcdserverpb::AuthRoleAddResponse> for AuthRoleAddResponse {
    fn from(proto: etcdserverpb::AuthRoleAddResponse) -> Self {
        Self {
            header: From::from(proto.header.expect("must fetch header")),
        }
    }
}
