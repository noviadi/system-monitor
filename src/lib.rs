//! System resource monitoring library for cross-platform system metrics.
//!
//! This library provides functionality for monitoring system resources including:
//! * CPU usage tracking
//! * Memory usage monitoring
//! * Real-time metrics updates
//!
//! # Architecture
//! The library is designed with a modular architecture that separates:
//! * State management (`App` struct)
//! * System metrics collection
//! * Data type definitions
//!
//! # Usage
//! ```no_run
//! use system_monitor::App;
//!
//! let mut app = App::new();
//! app.update(); // Update system metrics
//! let cpu = app.get_cpu_usage();
//! let memory = app.get_memory_usage();
//! ```
//!
//! # Platform Support
//! Currently supports:
//! * Windows
//! * Linux
//! * macOS

use sysinfo::{CpuExt, System, SystemExt};

/// Main application state for system monitoring.
///
/// Manages the state of system resource metrics and provides methods
/// for updating and accessing these metrics in real-time.
///
/// # Fields
/// * `system` - System information provider from sysinfo
///
/// # Example
/// ```no_run
/// use system_monitor::App;
///
/// let mut app = App::new();
/// app.update();
/// println!("CPU Usage: {}%", app.get_cpu_usage());
/// println!("Memory Usage: {}%", app.get_memory_usage());
/// ```
#[derive(Debug)]
pub struct App {
    system: System,
}

impl App {
    /// Creates a new App instance with initialized system monitoring.
    ///
    /// Initializes the system information provider and sets initial
    /// usage values to 0.
    ///
    /// # Returns
    /// * `App` - A new App instance ready for monitoring
    ///
    /// # Example
    /// ```no_run
    /// use system_monitor::App;
    ///
    /// let app = App::new();
    /// ```
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        Self { system }
    }

    /// Updates system metrics with current values.
    ///
    /// Refreshes both CPU and memory usage metrics by querying
    /// the system information provider.
    ///
    /// # Implementation Note
    /// Currently returns actual system metrics.
    ///
    /// # Example
    /// ```no_run
    /// use system_monitor::App;
    ///
    /// let mut app = App::new();
    /// app.update();
    /// // Metrics are now updated with current system values
    /// ```
    pub fn update(&mut self) {
        self.system.refresh_all();
    }

    /// Returns the current CPU usage percentage.
    ///
    /// # Returns
    /// * `f32` - CPU usage as a percentage between 0.0 and 100.0
    ///
    /// # Example
    /// ```no_run
    /// use system_monitor::App;
    ///
    /// let mut app = App::new();
    /// app.update();
    /// let cpu_usage = app.get_cpu_usage();
    /// assert!(cpu_usage >= 0.0 && cpu_usage <= 100.0);
    /// ```
    pub fn get_cpu_usage(&mut self) -> f32 {
        self.system.refresh_cpu();
        self.system
            .cpus()
            .iter()
            .map(|cpu| cpu.cpu_usage())
            .sum::<f32>()
            / self.system.cpus().len() as f32
    }

    /// Returns the current memory usage percentage.
    ///
    /// # Returns
    /// * `f32` - Memory usage as a percentage between 0.0 and 100.0
    ///
    /// # Example
    /// ```no_run
    /// use system_monitor::App;
    ///
    /// let mut app = App::new();
    /// app.update();
    /// let memory_usage = app.get_memory_usage();
    /// assert!(memory_usage >= 0.0 && memory_usage <= 100.0);
    /// ```
    pub fn get_memory_usage(&mut self) -> f32 {
        let total_memory = self.system.total_memory() as f32;
        let used_memory = self.system.used_memory() as f32;
        (used_memory / total_memory) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    // Helper function for test setup
    fn create_app() -> App {
        let app = App::new();
        // Give the system a moment to get initial readings
        thread::sleep(Duration::from_millis(100));
        app
    }

    mod initialization {
        use super::*;

        #[test]
        fn test_new_app_creation() {
            let app = create_app();
            assert!(
                app.system.cpus().len() > 0,
                "Should detect at least one CPU"
            );
        }
    }

    mod cpu_monitoring {
        use super::*;

        #[test]
        fn test_cpu_usage_range() {
            let mut app = create_app();
            let usage = app.get_cpu_usage();
            assert!(
                usage >= 0.0 && usage <= 100.0,
                "CPU usage should be between 0% and 100%, got {}%",
                usage
            );
        }

        #[test]
        fn test_cpu_updates() {
            let mut app = create_app();
            let initial = app.get_cpu_usage();
            thread::sleep(Duration::from_millis(100));
            app.update();
            let updated = app.get_cpu_usage();

            // Verify both readings are valid
            assert!(
                initial >= 0.0 && initial <= 100.0,
                "Initial CPU usage should be valid, got {}%",
                initial
            );
            assert!(
                updated >= 0.0 && updated <= 100.0,
                "Updated CPU usage should be valid, got {}%",
                updated
            );

            // Values might be different due to actual CPU usage changes
            println!("CPU usage changed from {}% to {}%", initial, updated);
        }
    }

    mod memory_monitoring {
        use super::*;

        #[test]
        fn test_memory_usage_range() {
            let mut app = create_app();
            let usage = app.get_memory_usage();
            assert!(
                usage >= 0.0 && usage <= 100.0,
                "Memory usage should be between 0% and 100%, got {}%",
                usage
            );
        }

        #[test]
        fn test_memory_updates() {
            let mut app = create_app();
            let initial = app.get_memory_usage();
            thread::sleep(Duration::from_millis(100));
            app.update();
            let updated = app.get_memory_usage();

            // Verify both readings are valid
            assert!(
                initial >= 0.0 && initial <= 100.0,
                "Initial memory usage should be valid, got {}%",
                initial
            );
            assert!(
                updated >= 0.0 && updated <= 100.0,
                "Updated memory usage should be valid, got {}%",
                updated
            );

            // Memory usage should be relatively stable in this short time
            let difference = (updated - initial).abs();
            assert!(
                difference < 10.0,
                "Memory usage shouldn't change drastically in 100ms. Changed by {}%",
                difference
            );

            println!("Memory usage changed from {}% to {}%", initial, updated);
        }
    }
}
