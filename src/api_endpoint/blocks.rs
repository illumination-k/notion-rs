use super::build_endpoint;
use crate::client::BaseClient;
use std::sync::Arc;

build_endpoint!(BlocksChildrenEndpoint);

pub struct BlocksEndpoint {
    parent: Arc<BaseClient>,
    pub children: BlocksChildrenEndpoint,
}

impl BlocksEndpoint {
    pub(crate) fn new(client: &Arc<BaseClient>) -> Self {
        Self {
            parent: Arc::clone(client),
            children: BlocksChildrenEndpoint::new(client),
        }
    }
}
