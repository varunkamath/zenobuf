use zenobuf_core::parameter::Parameter;

#[test]
fn test_parameter_new_string() {
    let param = Parameter::new("string_param", "hello".to_string()).unwrap();

    assert_eq!(param.name(), "string_param");
    assert_eq!(param.get_value::<String>().unwrap(), "hello".to_string());
}

#[test]
fn test_parameter_new_int() {
    let param = Parameter::new("int_param", 42).unwrap();

    assert_eq!(param.name(), "int_param");
    assert_eq!(param.get_value::<i32>().unwrap(), 42);
}

#[test]
fn test_parameter_new_float() {
    let param = Parameter::new("float_param", 3.14).unwrap();

    assert_eq!(param.name(), "float_param");
    assert_eq!(param.get_value::<f64>().unwrap(), 3.14);
}

#[test]
fn test_parameter_new_bool() {
    let param = Parameter::new("bool_param", true).unwrap();

    assert_eq!(param.name(), "bool_param");
    assert_eq!(param.get_value::<bool>().unwrap(), true);
}

#[test]
fn test_parameter_new_array() {
    let param = Parameter::new("array_param", vec![1, 2, 3]).unwrap();

    assert_eq!(param.name(), "array_param");
    assert_eq!(param.get_value::<Vec<i32>>().unwrap(), vec![1, 2, 3]);
}

#[test]
fn test_parameter_set() {
    let param = Parameter::new("param", 42).unwrap();

    // Change the value
    param.set_value(43).unwrap();

    assert_eq!(param.get_value::<i32>().unwrap(), 43);
}

#[test]
fn test_parameter_set_different_type() {
    let param = Parameter::new("param", 42).unwrap();

    // Change the value to a different type
    param.set_value("hello".to_string()).unwrap();

    assert_eq!(param.get_value::<String>().unwrap(), "hello".to_string());
}

#[test]
fn test_parameter_get_wrong_type() {
    let param = Parameter::new("param", 42).unwrap();

    // Try to get the value as a different type
    let result = param.get_value::<String>();

    assert!(result.is_err());
}

#[test]
fn test_parameter_serialization() {
    // Test with string parameter
    let param = Parameter::new("string_param", "hello".to_string()).unwrap();
    let serialized = serde_json::to_string(&"hello".to_string()).unwrap();
    assert_eq!(
        serde_json::to_string(&param.get_value::<String>().unwrap()).unwrap(),
        serialized
    );

    // Test with int parameter
    let param = Parameter::new("int_param", 42).unwrap();
    let serialized = serde_json::to_string(&42).unwrap();
    assert_eq!(
        serde_json::to_string(&param.get_value::<i32>().unwrap()).unwrap(),
        serialized
    );

    // Test with float parameter
    let param = Parameter::new("float_param", 3.14).unwrap();
    let serialized = serde_json::to_string(&3.14).unwrap();
    assert_eq!(
        serde_json::to_string(&param.get_value::<f64>().unwrap()).unwrap(),
        serialized
    );

    // Test with bool parameter
    let param = Parameter::new("bool_param", true).unwrap();
    let serialized = serde_json::to_string(&true).unwrap();
    assert_eq!(
        serde_json::to_string(&param.get_value::<bool>().unwrap()).unwrap(),
        serialized
    );

    // Test with array parameter
    let param = Parameter::new("array_param", vec![1, 2, 3]).unwrap();
    let serialized = serde_json::to_string(&vec![1, 2, 3]).unwrap();
    assert_eq!(
        serde_json::to_string(&param.get_value::<Vec<i32>>().unwrap()).unwrap(),
        serialized
    );
}
