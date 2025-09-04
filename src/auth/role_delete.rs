use crate::ResponseHeader;
use crate::proto::etcdserverpb;

#[derive(Debug, Clone)]
pub struct AuthRoleDeleteRequest {
    proto: etcdserverpb::AuthRoleDeleteRequest,
}

impl AuthRoleDeleteRequest {
    pub fn new(role: impl Into<String>) -> Self {
        Self {
            proto: etcdserverpb::AuthRoleDeleteRequest { role: role.into() },
        }
    }
}

impl<I> From<I> for AuthRoleDeleteRequest
where
    I: Into<String>,
{
    fn from(role: I) -> Self {
        Self::new(role)
    }
}

impl From<AuthRoleDeleteRequest> for etcdserverpb::AuthRoleDeleteRequest {
    fn from(req: AuthRoleDeleteRequest) -> Self {
        req.proto
    }
}

#[derive(Debug, Clone)]
pub struct AuthRoleDeleteResponse {
    pub header: ResponseHeader,
}

impl From<etcdserverpb::AuthRoleDeleteResponse> for AuthRoleDeleteResponse {
    fn from(proto: etcdserverpb::AuthRoleDeleteResponse) -> Self {
        Self {
            header: From::from(proto.header.expect("must fetch header")),
        }
    }
}
