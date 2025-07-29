use std::time::Duration;

use zenobuf_core::qos::{Durability, History, QosProfile, Reliability};

#[test]
fn test_qos_profile_default() {
    let qos = QosProfile::default();

    assert_eq!(qos.reliability, Reliability::Reliable);
    assert_eq!(qos.durability, Durability::Volatile);
    assert_eq!(qos.history, History::KeepLast);
    assert_eq!(qos.depth, 10);
    assert_eq!(qos.deadline, None);
    assert_eq!(qos.lifespan, None);
}

#[test]
fn test_qos_profile_builder() {
    let qos = QosProfile::default()
        .reliability(Reliability::BestEffort)
        .durability(Durability::TransientLocal)
        .history(History::KeepAll)
        .depth(20)
        .deadline(Duration::from_secs(1))
        .lifespan(Duration::from_secs(10));

    assert_eq!(qos.reliability, Reliability::BestEffort);
    assert_eq!(qos.durability, Durability::TransientLocal);
    assert_eq!(qos.history, History::KeepAll);
    assert_eq!(qos.depth, 20);
    assert_eq!(qos.deadline, Some(Duration::from_secs(1)));
    assert_eq!(qos.lifespan, Some(Duration::from_secs(10)));
}

#[test]
fn test_qos_profile_sensor_data() {
    let qos = QosProfile::sensor_data();

    assert_eq!(qos.reliability, Reliability::BestEffort);
    assert_eq!(qos.durability, Durability::Volatile);
    assert_eq!(qos.history, History::KeepLast);
    assert_eq!(qos.depth, 5);
    assert_eq!(qos.deadline, None);
    assert_eq!(qos.lifespan, None);
}

#[test]
fn test_qos_profile_parameters() {
    let qos = QosProfile::parameters();

    assert_eq!(qos.reliability, Reliability::Reliable);
    assert_eq!(qos.durability, Durability::TransientLocal);
    assert_eq!(qos.history, History::KeepLast);
    assert_eq!(qos.depth, 1);
    assert_eq!(qos.deadline, None);
    assert_eq!(qos.lifespan, None);
}

#[test]
fn test_qos_profile_services() {
    let qos = QosProfile::services();

    assert_eq!(qos.reliability, Reliability::Reliable);
    assert_eq!(qos.durability, Durability::Volatile);
    assert_eq!(qos.history, History::KeepLast);
    assert_eq!(qos.depth, 10);
    assert_eq!(qos.deadline, Some(Duration::from_secs(1)));
    assert_eq!(qos.lifespan, None);
}

#[test]
fn test_qos_profile_clone() {
    let qos = QosProfile::default()
        .reliability(Reliability::BestEffort)
        .durability(Durability::TransientLocal)
        .history(History::KeepAll)
        .depth(20)
        .deadline(Duration::from_secs(1))
        .lifespan(Duration::from_secs(10));

    let qos_clone = qos.clone();

    assert_eq!(qos_clone.reliability, Reliability::BestEffort);
    assert_eq!(qos_clone.durability, Durability::TransientLocal);
    assert_eq!(qos_clone.history, History::KeepAll);
    assert_eq!(qos_clone.depth, 20);
    assert_eq!(qos_clone.deadline, Some(Duration::from_secs(1)));
    assert_eq!(qos_clone.lifespan, Some(Duration::from_secs(10)));
}

#[test]
fn test_qos_profile_debug() {
    let qos = QosProfile::default();
    let debug_str = format!("{qos:?}");

    assert!(debug_str.contains("QosProfile"));
    assert!(debug_str.contains("reliability"));
    assert!(debug_str.contains("durability"));
    assert!(debug_str.contains("history"));
    assert!(debug_str.contains("depth"));
}
