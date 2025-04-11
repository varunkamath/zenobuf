//! Example applications for the Zenobuf framework

// Include the generated Protocol Buffer code
pub mod proto {
    pub mod geometry {
        include!(concat!(env!("OUT_DIR"), "/zenobuf.examples.geometry.rs"));

        // Implement the Message trait for the generated types
        impl zenobuf_core::Message for Point {
            fn type_name() -> &'static str {
                "zenobuf.examples.geometry.Point"
            }
        }

        impl zenobuf_core::Message for Quaternion {
            fn type_name() -> &'static str {
                "zenobuf.examples.geometry.Quaternion"
            }
        }

        impl zenobuf_core::Message for Pose {
            fn type_name() -> &'static str {
                "zenobuf.examples.geometry.Pose"
            }
        }
    }

    pub mod service {
        include!(concat!(env!("OUT_DIR"), "/zenobuf.examples.service.rs"));

        // Implement the Message trait for the generated types
        impl zenobuf_core::Message for AddTwoIntsRequest {
            fn type_name() -> &'static str {
                "zenobuf.examples.service.AddTwoIntsRequest"
            }
        }

        impl zenobuf_core::Message for AddTwoIntsResponse {
            fn type_name() -> &'static str {
                "zenobuf.examples.service.AddTwoIntsResponse"
            }
        }
    }
}
