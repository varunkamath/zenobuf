//! Example applications for the Zenobuf framework

// Include the generated Protocol Buffer code
pub mod proto {
    pub mod geometry {
        include!(concat!(env!("OUT_DIR"), "/zenobuf.examples.geometry.rs"));
    }

    pub mod service {
        include!(concat!(env!("OUT_DIR"), "/zenobuf.examples.service.rs"));
    }
}
