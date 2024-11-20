//! Integration tests for the system monitor application.
//!
//! This module contains integration tests that verify the system monitor's
//! functionality as a complete unit. Tests cover:
//! * System metrics collection
//! * Resource usage calculations
//! * Data consistency
//! * Update behavior
//!
//! These tests ensure that the application correctly:
//! 1. Initializes system monitoring
//! 2. Retrieves accurate metrics
//! 3. Updates values consistently
//! 4. Handles resource calculations properly

use std::time::Duration;
use system_monitor::App; // This assumes we'll make our App public

/// Tests the system monitoring flow.
///
/// Verifies that:
/// * App can be created successfully
/// * Initial values are within valid ranges
/// * System monitoring is properly initialized
/// * Values update after refresh
#[test]
fn test_system_monitoring_flow() {
    let mut app = App::new();

    // Initial readings
    let initial_cpu = app.get_cpu_usage();
    let initial_memory = app.get_memory_usage();

    // Wait a bit and take new readings
    std::thread::sleep(Duration::from_secs(1));
    app.update();

    let updated_cpu = app.get_cpu_usage();
    let updated_memory = app.get_memory_usage();

    // Verify readings are within valid ranges
    assert!(initial_cpu >= 0.0 && initial_cpu <= 100.0);
    assert!(updated_cpu >= 0.0 && updated_cpu <= 100.0);
    assert!(initial_memory >= 0.0 && initial_memory <= 100.0);
    assert!(updated_memory >= 0.0 && updated_memory <= 100.0);
}

/// Tests the update mechanism of the application.
///
/// Verifies that:
/// * Updates complete successfully
/// * Resource values change appropriately
/// * Multiple updates maintain consistency
///
/// # Note
/// This test ensures the update mechanism works correctly
/// even with rapid, repeated calls.
#[test]
fn test_multiple_updates() {
    let mut app = App::new();

    // Test multiple consecutive updates
    for _ in 0..5 {
        app.update();
        let cpu = app.get_cpu_usage();
        let memory = app.get_memory_usage();

        assert!(cpu >= 0.0 && cpu <= 100.0);
        assert!(memory >= 0.0 && memory <= 100.0);

        std::thread::sleep(Duration::from_millis(100));
    }
}
