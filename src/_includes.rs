pub mod google {
    pub mod api {
        include!("google.api.rs");
    }
    pub mod rpc {
        include!("google.rpc.rs");
    }
}
pub mod postman {
    pub mod api {
        pub mod v1 {
            include!("postman.api.v1.rs");
        }
    }
}
