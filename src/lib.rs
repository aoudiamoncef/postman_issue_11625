//! Copyright (c) 2022 Postman
//! Postman protos module
#![warn(unused_crate_dependencies)]

use futures_util as _;
use tokio as _;
use tonic_reflection as _;
use tonic_web as _;

/// Google protos Module
pub mod google {
    /// Google API Module
    pub mod api {
        include!("google.api.rs");
    }
    /// Google RPC Module
    pub mod rpc {
        include!("google.rpc.rs");
    }
}

/// Postman protos Module
pub mod postman {
    /// Postman API Module
    pub mod api {
        /// Version 1 of the Postman protos
        pub mod v1 {
            include!("postman.api.v1.rs");
            /// Compiled file descriptor set for the Postman protos
            pub const FILE_DESCRIPTOR_SET: &[u8] = include_bytes!("api.bin");
        }
    }
}
