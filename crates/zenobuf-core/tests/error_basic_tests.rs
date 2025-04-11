use zenobuf_core::error::Error;

#[test]
fn test_error_display() {
    // Test serialization error
    let error = Error::Serialization("failed to serialize".to_string());
    assert!(format!("{}", error).contains("Serialization error"));
    assert!(format!("{}", error).contains("failed to serialize"));

    // Test node already exists error
    let error = Error::NodeAlreadyExists("test_node".to_string());
    assert!(format!("{}", error).contains("Node already exists"));
    assert!(format!("{}", error).contains("test_node"));

    // Test topic already exists error
    let error = Error::TopicAlreadyExists("test_topic".to_string());
    assert!(format!("{}", error).contains("Topic already exists"));
    assert!(format!("{}", error).contains("test_topic"));

    // Test service already exists error
    let error = Error::ServiceAlreadyExists("test_service".to_string());
    assert!(format!("{}", error).contains("Service already exists"));
    assert!(format!("{}", error).contains("test_service"));

    // Test service call timeout error
    let error = Error::ServiceCallTimeout("test_service".to_string());
    assert!(format!("{}", error).contains("Service call timed out"));
    assert!(format!("{}", error).contains("test_service"));

    // Test service call failed error
    let error = Error::ServiceCallFailed("test_service".to_string());
    assert!(format!("{}", error).contains("Service call failed"));
    assert!(format!("{}", error).contains("test_service"));

    // Test parameter error
    let error = Error::Parameter("failed to set parameter".to_string());
    assert!(format!("{}", error).contains("Parameter error"));
    assert!(format!("{}", error).contains("failed to set parameter"));

    // Test node error
    let error = Error::Node("failed to create node".to_string());
    assert!(format!("{}", error).contains("Node error"));
    assert!(format!("{}", error).contains("failed to create node"));

    // Test publisher error
    let error = Error::Publisher("failed to create publisher".to_string());
    assert!(format!("{}", error).contains("Publisher error"));
    assert!(format!("{}", error).contains("failed to create publisher"));

    // Test subscriber error
    let error = Error::Subscriber("failed to create subscriber".to_string());
    assert!(format!("{}", error).contains("Subscriber error"));
    assert!(format!("{}", error).contains("failed to create subscriber"));

    // Test service error
    let error = Error::Service("failed to create service".to_string());
    assert!(format!("{}", error).contains("Service error"));
    assert!(format!("{}", error).contains("failed to create service"));

    // Test client error
    let error = Error::Client("failed to create client".to_string());
    assert!(format!("{}", error).contains("Client error"));
    assert!(format!("{}", error).contains("failed to create client"));

    // Test not supported error
    let error = Error::NotSupported("operation not supported".to_string());
    assert!(format!("{}", error).contains("Operation not supported"));
    assert!(format!("{}", error).contains("operation not supported"));

    // Test not implemented error
    let error = Error::NotImplemented("operation not implemented".to_string());
    assert!(format!("{}", error).contains("Operation not implemented"));
    assert!(format!("{}", error).contains("operation not implemented"));

    // Test other error
    let error = Error::Other("other error".to_string());
    assert!(format!("{}", error).contains("Other error"));
    assert!(format!("{}", error).contains("other error"));
}

#[test]
fn test_error_debug() {
    // Test Debug implementation
    let error = Error::Other("other error".to_string());
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("Other"));
    assert!(debug_str.contains("other error"));
}
