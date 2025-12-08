use std::sync::Arc;

use zenobuf_core::node::Node;
use zenobuf_core::transport::ZenohTransport;

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_basic_commands() {
    // Create a transport
    let transport = ZenohTransport::new().await.unwrap();

    // Create a node
    let node = Arc::new(Node::with_transport("test_node", transport).await.unwrap());

    // Set a parameter
    node.set_parameter("test_param", 42).unwrap();

    // Get the parameter
    let value = node.get_parameter::<i32>("test_param").unwrap();
    assert_eq!(value, 42);
}
