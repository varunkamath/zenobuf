//! Example parameter usage for the Zenobuf framework

use std::time::Duration;

use zenobuf_core::{Node, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create a node
    let node = Node::new("parameter_example").await?;

    // Set parameters
    node.set_parameter("string_param", "hello".to_string())?;
    node.set_parameter("int_param", 42)?;
    node.set_parameter("float_param", std::f64::consts::PI)?;
    node.set_parameter("bool_param", true)?;

    // Get parameters
    let string_param: String = node.get_parameter("string_param")?;
    let int_param: i32 = node.get_parameter("int_param")?;
    let float_param: f64 = node.get_parameter("float_param")?;
    let bool_param: bool = node.get_parameter("bool_param")?;

    // Print parameters
    println!("Parameters:");
    println!("  string_param: {}", string_param);
    println!("  int_param: {}", int_param);
    println!("  float_param: {}", float_param);
    println!("  bool_param: {}", bool_param);

    // Update a parameter
    println!("Updating int_param to 100");
    node.set_parameter("int_param", 100)?;

    // Get the updated parameter
    let int_param: i32 = node.get_parameter("int_param")?;
    println!("  int_param: {}", int_param);

    // Sleep for a while
    tokio::time::sleep(Duration::from_secs(1)).await;

    Ok(())
}
