mod member_add;
mod member_list;
mod member_remove;
mod member_update;

pub use member_add::{MemberAddRequest, MemberAddResponse};
pub use member_list::{MemberListRequest, MemberListResponse};
pub use member_remove::{MemberRemoveRequest, MemberRemoveResponse};
pub use member_update::{MemberUpdateRequest, MemberUpdateResponse};

use std::future::Future;

use crate::Result;
use crate::proto::etcdserverpb;

pub trait ClusterOp {
    fn member_add<R>(&self, req: R) -> impl Future<Output = Result<MemberAddResponse>>
    where
        R: Into<MemberAddRequest> + Send;

    fn member_remove<R>(&self, req: R) -> impl Future<Output = Result<MemberRemoveResponse>>
    where
        R: Into<MemberRemoveRequest> + Send;

    fn member_update<R>(&self, req: R) -> impl Future<Output = Result<MemberUpdateResponse>>
    where
        R: Into<MemberUpdateRequest> + Send;

    fn member_list(&self) -> impl Future<Output = Result<MemberListResponse>>;
}

#[derive(Debug, Clone)]
pub struct Member {
    pub id: u64,
    pub name: String,
    pub peer_urls: Vec<String>,
    pub client_urls: Vec<String>,
    pub is_learner: bool,
}

impl From<etcdserverpb::Member> for Member {
    fn from(proto: etcdserverpb::Member) -> Self {
        Self {
            id: proto.id,
            name: proto.name,
            peer_urls: proto.peer_ur_ls,
            client_urls: proto.client_ur_ls,
            is_learner: proto.is_learner,
        }
    }
}

impl From<Member> for etcdserverpb::Member {
    fn from(value: Member) -> Self {
        etcdserverpb::Member {
            id: value.id,
            name: value.name,
            peer_ur_ls: value.peer_urls,
            client_ur_ls: value.client_urls,
            is_learner: value.is_learner,
        }
    }
}
