pub mod blocks;
pub mod databases;
pub mod pages;
pub mod params;
pub mod search;
pub mod users;

pub use blocks::*;
pub use databases::*;
pub use pages::*;
pub use search::*;
pub use users::*;

/// define Endpoint Struct and `new` method
macro_rules! build_endpoint {
    ($name: ident) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            parent: std::sync::Arc<crate::client::BaseClient>,
        }

        impl $name {
            pub(crate) fn new(client: &std::sync::Arc<crate::client::BaseClient>) -> Self {
                Self {
                    parent: std::sync::Arc::clone(client),
                }
            }
        }
    };
}

pub(super) use build_endpoint;
