use std::sync::Arc;

use crate::client::BaseClient;

pub struct BlocksChildrenEndpoint {
    parent: Arc<BaseClient>,
}

impl BlocksChildrenEndpoint {
    pub(crate) fn new(client: &Arc<BaseClient>) -> Self {
        Self {
            parent: Arc::clone(client),
        }
    }
}
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
