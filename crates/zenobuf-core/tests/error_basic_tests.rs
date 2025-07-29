use zenobuf_core::error::Error;

#[test]
fn test_error_display() {
    // Test serialization error (legacy)
    let error = Error::Serialization("failed to serialize".to_string());
    assert!(format!("{error}").contains("Serialization error"));
    assert!(format!("{error}").contains("failed to serialize"));

    // Test new structured error constructors
    let error = Error::node_already_exists("test_node");
    assert_eq!(error.to_string(), "Node 'test_node' already exists");

    let error = Error::topic_already_exists("test_topic", "test_node");
    assert_eq!(
        error.to_string(),
        "Topic 'test_topic' already exists on node 'test_node'"
    );

    let error = Error::service_already_exists("test_service", "test_node");
    assert_eq!(
        error.to_string(),
        "Service 'test_service' already exists on node 'test_node'"
    );

    let error = Error::service_call_timeout("test_service", 5000);
    assert_eq!(
        error.to_string(),
        "Service call to 'test_service' timed out after 5000ms"
    );

    let error = Error::service_call_failed("test_service", "connection failed");
    assert_eq!(
        error.to_string(),
        "Service call to 'test_service' failed: connection failed"
    );

    let error = Error::parameter("test_param", "failed to set parameter");
    assert_eq!(
        error.to_string(),
        "Parameter 'test_param' error: failed to set parameter"
    );

    let error = Error::node("test_node", "failed to create node");
    assert_eq!(
        error.to_string(),
        "Node 'test_node' error: failed to create node"
    );

    let error = Error::publisher("test_topic", "failed to create publisher");
    assert_eq!(
        error.to_string(),
        "Publisher for topic 'test_topic' error: failed to create publisher"
    );

    let error = Error::subscriber("test_topic", "failed to create subscriber");
    assert_eq!(
        error.to_string(),
        "Subscriber for topic 'test_topic' error: failed to create subscriber"
    );

    let error = Error::service("test_service", "failed to create service");
    assert_eq!(
        error.to_string(),
        "Service 'test_service' error: failed to create service"
    );

    let error = Error::client("test_service", "failed to create client");
    assert_eq!(
        error.to_string(),
        "Client for service 'test_service' error: failed to create client"
    );

    let error = Error::configuration("invalid configuration");
    assert_eq!(
        error.to_string(),
        "Configuration error: invalid configuration"
    );

    let error = Error::network("connection timeout");
    assert_eq!(error.to_string(), "Network error: connection timeout");

    let error = Error::other("other error");
    assert_eq!(error.to_string(), "Error: other error");
}

#[test]
fn test_error_debug() {
    // Test Debug implementation
    let error = Error::other("other error");
    let debug_str = format!("{error:?}");
    assert!(debug_str.contains("Other"));
    assert!(debug_str.contains("other error"));
}

#[test]
fn test_error_context() {
    // Test error context helpers
    use zenobuf_core::error::ErrorContext;

    let result: Result<(), Error> = Err(Error::other("base error"));
    let with_context = result.with_context("additional context");

    assert!(with_context.is_err());
}
