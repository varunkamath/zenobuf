use zenobuf_core::node::Node;
use zenobuf_core::transport::ZenohTransport;

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_node_creation() {
    let transport = ZenohTransport::new().await.unwrap();
    let node = Node::with_transport("test_node", transport).unwrap();

    assert_eq!(node.name(), "test_node");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_node_parameter() {
    let transport = ZenohTransport::new().await.unwrap();
    let node = Node::with_transport("test_node", transport).unwrap();

    // Set a parameter
    node.set_parameter("test_param", 42).unwrap();

    // Get the parameter
    let value = node.get_parameter::<i32>("test_param").unwrap();
    assert_eq!(value, 42);

    // Set the parameter to a new value
    node.set_parameter("test_param", 43).unwrap();
    let value = node.get_parameter::<i32>("test_param").unwrap();
    assert_eq!(value, 43);

    // Try to get a non-existent parameter
    let result = node.get_parameter::<i32>("nonexistent");
    assert!(result.is_err());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_node_spin_once() {
    let transport = ZenohTransport::new().await.unwrap();
    let node = Node::with_transport("test_node", transport).unwrap();

    // Spin once should not fail
    node.spin_once().unwrap();
}
