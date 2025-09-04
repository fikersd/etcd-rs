use crate::ResponseHeader;
use crate::proto::etcdserverpb;

#[derive(Debug, Default, Clone)]
pub struct AuthRoleListRequest {
    proto: etcdserverpb::AuthRoleListRequest,
}

impl From<AuthRoleListRequest> for etcdserverpb::AuthRoleListRequest {
    fn from(req: AuthRoleListRequest) -> Self {
        req.proto
    }
}

#[derive(Debug, Clone)]
pub struct AuthRoleListResponse {
    pub header: ResponseHeader,
    pub roles: Vec<String>,
}

impl From<etcdserverpb::AuthRoleListResponse> for AuthRoleListResponse {
    fn from(proto: etcdserverpb::AuthRoleListResponse) -> Self {
        Self {
            header: From::from(proto.header.expect("must fetch header")),
            roles: proto.roles,
        }
    }
}
