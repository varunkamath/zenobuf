use std::time::{Duration, SystemTime, UNIX_EPOCH};

use zenobuf_core::time::{Time, ZenobufDuration};
use zenobuf_core::util::{duration_to_string, string_to_duration};

#[test]
fn test_time_now() {
    let time = Time::now();
    let system_time = SystemTime::now();
    let system_duration = system_time
        .duration_since(UNIX_EPOCH)
        .expect("System time before Unix epoch");

    // The times should be very close (within 1 second)
    let time_duration = time.to_duration();
    let diff = if time_duration > system_duration {
        time_duration - system_duration
    } else {
        system_duration - time_duration
    };

    assert!(diff < Duration::from_secs(1));
}

#[test]
fn test_time_from_duration() {
    let duration = Duration::from_secs(1234567890);
    let time = Time::from_duration(duration);

    assert_eq!(time.sec, 1234567890);
    assert_eq!(time.nsec, 0);
}

#[test]
fn test_time_to_duration() {
    let time = Time::new(1234567890, 123456789);
    let duration = time.to_duration();

    assert_eq!(duration.as_secs(), 1234567890);
    assert_eq!(duration.subsec_nanos(), 123456789);
}

#[test]
fn test_time_normalize() {
    let time = Time::new(1234567890, 1234567890); // nsec > 1_000_000_000

    assert_eq!(time.sec, 1234567891);
    assert_eq!(time.nsec, 234567890);
}

#[test]
fn test_time_add() {
    let time = Time::new(1234567890, 500000000);
    let duration = Duration::from_millis(1500);
    let new_time = time.add(duration);

    assert_eq!(new_time.sec, 1234567892);
    assert_eq!(new_time.nsec, 0);
}

#[test]
fn test_time_sub() {
    let time = Time::new(1234567890, 500000000);
    let duration = Duration::from_millis(1500);
    let new_time = time.sub(duration);

    assert_eq!(new_time.sec, 1234567889);
    assert_eq!(new_time.nsec, 0);
}

#[test]
fn test_time_from_system_time() {
    let system_time = UNIX_EPOCH + Duration::from_secs(1234567890);
    let time = Time::from(system_time);

    assert_eq!(time.sec, 1234567890);
    assert_eq!(time.nsec, 0);
}

#[test]
fn test_zenobuf_duration_new() {
    let duration = ZenobufDuration::new(1234, 567890000);

    assert_eq!(duration.sec, 1234);
    assert_eq!(duration.nsec, 567890000);
}

#[test]
fn test_zenobuf_duration_from_std() {
    let std_duration = Duration::from_secs(1234) + Duration::from_nanos(567890000);
    let duration = ZenobufDuration::from_std(std_duration);

    assert_eq!(duration.sec, 1234);
    assert_eq!(duration.nsec, 567890000);
}

#[test]
fn test_zenobuf_duration_to_std() {
    let duration = ZenobufDuration::new(1234, 567890000);
    let std_duration = duration.to_std();

    assert_eq!(std_duration.as_secs(), 1234);
    assert_eq!(std_duration.subsec_nanos(), 567890000);
}

#[test]
fn test_zenobuf_duration_normalize() {
    let duration = ZenobufDuration::new(1234, 1567890000); // nsec > 1_000_000_000

    assert_eq!(duration.sec, 1235);
    assert_eq!(duration.nsec, 567890000);

    let duration = ZenobufDuration::new(1234, -500000000); // nsec < 0

    assert_eq!(duration.sec, 1233);
    assert_eq!(duration.nsec, 500000000);
}

#[test]
fn test_duration_to_string() {
    let duration = Duration::from_secs(3661) + Duration::from_millis(42);
    let string = duration_to_string(duration);

    assert_eq!(string, "1h 1m 1s 42ms");

    let duration = Duration::from_secs(61) + Duration::from_millis(42);
    let string = duration_to_string(duration);

    assert_eq!(string, "1m 1s 42ms");

    let duration = Duration::from_secs(1) + Duration::from_millis(42);
    let string = duration_to_string(duration);

    assert_eq!(string, "1s 42ms");

    let duration = Duration::from_millis(42);
    let string = duration_to_string(duration);

    assert_eq!(string, "42ms");
}

#[test]
fn test_string_to_duration() {
    let string = "1h 1m 1s 42ms";
    let duration = string_to_duration(string).unwrap();

    assert_eq!(duration.as_secs(), 3661);
    assert_eq!(duration.subsec_millis(), 42);

    let string = "1m 1s 42ms";
    let duration = string_to_duration(string).unwrap();

    assert_eq!(duration.as_secs(), 61);
    assert_eq!(duration.subsec_millis(), 42);

    let string = "1s 42ms";
    let duration = string_to_duration(string).unwrap();

    assert_eq!(duration.as_secs(), 1);
    assert_eq!(duration.subsec_millis(), 42);

    let string = "42ms";
    let duration = string_to_duration(string).unwrap();

    assert_eq!(duration.as_secs(), 0);
    assert_eq!(duration.subsec_millis(), 42);

    // Test with different units
    let string = "1hour 30minutes 45seconds";
    let duration = string_to_duration(string).unwrap();

    assert_eq!(duration.as_secs(), 5445);
    assert_eq!(duration.subsec_millis(), 0);

    // Test with invalid unit
    let string = "42invalid";
    let duration = string_to_duration(string).unwrap();

    assert_eq!(duration.as_secs(), 0);
    assert_eq!(duration.subsec_millis(), 0);
}
