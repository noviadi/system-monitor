//! Terminal-based system resource monitor application.
//!
//! This module provides the terminal user interface and application runtime for
//! the system resource monitor. It uses the `tui` library for rendering and
//! `crossterm` for terminal manipulation.
//!
//! # Features
//! * Real-time system metrics visualization
//! * CPU and memory usage gauges
//! * Cross-platform terminal UI
//!
//! # Controls
//! * Press 'q' to quit the application
//!
//! # Layout
//! The interface is divided into three sections:
//! 1. Title bar (2 units high)
//! 2. CPU usage gauge (50% of remaining space)
//! 3. Memory usage gauge (50% of remaining space)

use std::error::Error;
use std::io;
use std::time::Duration;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame, Terminal,
};

// Import App from our library
use system_monitor::App;

/// Runs the application's main event loop.
///
/// Handles terminal events and updates the UI at regular intervals.
/// The loop continues until the user presses 'q' to quit.
///
/// # Arguments
/// * `terminal` - Mutable reference to the terminal backend
/// * `app` - Mutable reference to the application state
///
/// # Returns
/// * `io::Result<()>` - Success if the application exits normally
///
/// # Example
/// ```no_run
/// use tui::Terminal;
/// use tui::backend::CrosstermBackend;
/// use system_monitor::App;
///
/// let backend = CrosstermBackend::new(std::io::stdout());
/// let mut terminal = Terminal::new(backend).unwrap();
/// let mut app = App::new();
/// run_app(&mut terminal, &mut app).unwrap();
/// ```
fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }

        app.update();
        std::thread::sleep(Duration::from_millis(250));
    }
}

/// Renders the user interface.
///
/// Creates a vertical layout with three sections:
/// * Title section (2 units high)
/// * CPU usage gauge (50% of remaining space)
/// * Memory usage gauge (50% of remaining space)
///
/// # Arguments
/// * `f` - Frame used for rendering
/// * `app` - Mutable reference to application state
///
/// # Type Parameters
/// * `B` - Backend implementing the `Backend` trait
fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),   // Fixed height for title
            Constraint::Ratio(1, 2), // Half of remaining space
            Constraint::Ratio(1, 2), // Half of remaining space
        ])
        .split(f.size());

    // Title
    let title = Paragraph::new(Spans::from(vec![
        Span::styled(
            "System Monitor",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" (Press 'q' to quit)"),
    ]));
    f.render_widget(title, chunks[0]);

    // CPU Usage Gauge
    let cpu_usage = app.get_cpu_usage();
    let cpu_gauge = Gauge::default()
        .block(Block::default().title("CPU Usage").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Cyan))
        .percent(cpu_usage as u16);
    f.render_widget(cpu_gauge, chunks[1]);

    // Memory Usage Gauge
    let memory_usage = app.get_memory_usage();
    let memory_gauge = Gauge::default()
        .block(Block::default().title("Memory Usage").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Magenta))
        .percent(memory_usage as u16);
    f.render_widget(memory_gauge, chunks[2]);
}

// UI-specific tests
#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the basic UI layout configuration.
    ///
    /// Verifies:
    /// * Correct number of sections (3)
    /// * Title height (2 units)
    /// * Full width usage
    /// * Proper vertical positioning
    /// * Height distribution
    #[test]
    fn test_ui_layout() {
        // Create a mock terminal size
        let size = Rect::new(0, 0, 100, 100); // Mock terminal of 100x100

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),   // Fixed height for title
                Constraint::Ratio(1, 2), // Half of remaining space
                Constraint::Ratio(1, 2), // Half of remaining space
            ])
            .split(size);

        // Test layout structure
        assert_eq!(chunks.len(), 3, "Layout should have three sections");

        // Test section heights
        assert_eq!(chunks[0].height, 2, "Title should be exactly 2 units high");

        // Test that sections fill the width
        assert_eq!(chunks[0].width, 100, "Sections should use full width");
        assert_eq!(chunks[1].width, 100, "Sections should use full width");
        assert_eq!(chunks[2].width, 100, "Sections should use full width");

        // Test vertical positioning
        assert_eq!(chunks[0].y, 0, "Title should start at top");
        assert_eq!(chunks[1].y, 2, "CPU gauge should start after title");
        assert_eq!(
            chunks[2].y,
            chunks[1].y + chunks[1].height,
            "Memory gauge should start after CPU gauge"
        );

        // Test that the layout fills the entire height
        assert_eq!(
            chunks[0].height + chunks[1].height + chunks[2].height,
            size.height,
            "Layout should fill entire height"
        );

        // Test that CPU and Memory sections are approximately equal
        assert!(
            (chunks[1].height as i32 - chunks[2].height as i32).abs() <= 2,
            "CPU and Memory sections should be approximately equal"
        );
    }

    /// Tests layout constraints across various terminal sizes.
    ///
    /// Verifies layout behavior for:
    /// * Standard terminal (80x24)
    /// * Large terminal (120x40)
    /// * Small terminal (60x20)
    /// * Tiny terminal (10x10)
    /// * Maximum reasonable size (255x255)
    ///
    /// Checks:
    /// * Minimum heights
    /// * Section proportions
    /// * Space utilization
    /// * Height differences between sections
    #[test]
    fn test_layout_constraints() {
        // Test with different terminal sizes
        let test_sizes = vec![
            (80, 24),   // Standard terminal
            (120, 40),  // Large terminal
            (60, 20),   // Small terminal
            (10, 10),   // Tiny terminal
            (255, 255), // Max reasonable terminal size
        ];

        for (width, height) in test_sizes {
            let size = Rect::new(0, 0, width, height);
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(2),   // Fixed height for title
                    Constraint::Ratio(1, 2), // Half of remaining space
                    Constraint::Ratio(1, 2), // Half of remaining space
                ])
                .split(size);

            // Print detailed debug information
            println!("\nTerminal size: {}x{}", width, height);
            println!("Title height: {}", chunks[0].height);
            println!("CPU height: {}", chunks[1].height);
            println!("Memory height: {}", chunks[2].height);
            println!(
                "Total height: {}",
                chunks[0].height + chunks[1].height + chunks[2].height
            );

            // Verify minimum heights
            assert!(
                chunks[0].height > 0,
                "Title section should have non-zero height"
            );
            assert!(
                chunks[1].height > 0,
                "CPU gauge should have non-zero height"
            );
            assert!(
                chunks[2].height > 0,
                "Memory gauge should have non-zero height"
            );

            // Verify the title is exactly 2 units high
            assert_eq!(chunks[0].height, 2, "Title should be exactly 2 units high");

            // Verify total height
            let total_height = chunks[0].height + chunks[1].height + chunks[2].height;
            assert_eq!(
                total_height, height,
                "Total height {} should match terminal height {}",
                total_height, height
            );

            // Verify the difference between sections is at most 2
            assert!(
                (chunks[1].height as i32 - chunks[2].height as i32).abs() <= 2,
                "CPU height {} and Memory height {} should differ by at most 2 units",
                chunks[1].height,
                chunks[2].height
            );

            // Verify that sections are reasonable proportions
            let remaining_height = height - chunks[0].height;
            assert!(
                chunks[1].height <= remaining_height,
                "CPU section height {} should not exceed remaining height {}",
                chunks[1].height,
                remaining_height
            );
            assert!(
                chunks[2].height <= remaining_height,
                "Memory section height {} should not exceed remaining height {}",
                chunks[2].height,
                remaining_height
            );

            // Verify that sections together use all remaining space
            assert_eq!(
                chunks[1].height + chunks[2].height,
                remaining_height,
                "CPU and Memory sections should use all remaining space"
            );
        }
    }

    /// Tests layout behavior with extremely large terminal sizes.
    ///
    /// Verifies layout stability with:
    /// * Very large terminals (1000x1000)
    /// * Maximum possible dimensions (u16::MAX)
    ///
    /// Ensures:
    /// * Layout remains functional
    /// * No integer overflow occurs
    /// * Proper section ordering
    #[test]
    fn test_large_terminal_size() {
        // Test handling of unreasonably large terminal sizes
        let large_sizes = vec![(1000, 1000), (500, 500), (u16::MAX, u16::MAX)];

        for (width, height) in large_sizes {
            let size = Rect::new(0, 0, width, height);
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(2),
                    Constraint::Ratio(1, 2),
                    Constraint::Ratio(1, 2),
                ])
                .split(size);

            println!("\nLarge terminal size: {}x{}", width, height);
            println!("Title height: {}", chunks[0].height);
            println!("CPU height: {}", chunks[1].height);
            println!("Memory height: {}", chunks[2].height);
            let total = chunks[0].height + chunks[1].height + chunks[2].height;
            println!("Total height: {} (Original height: {})", total, height);

            // For large terminals, we only verify:
            // 1. Title is still 2 units
            assert_eq!(chunks[0].height, 2, "Title should be exactly 2 units high");

            // 2. Sections are non-zero
            assert!(chunks[1].height > 0, "CPU section should be non-zero");
            assert!(chunks[2].height > 0, "Memory section should be non-zero");

            // 3. Sections are approximately equal
            assert!(
                (chunks[1].height as i32 - chunks[2].height as i32).abs() <= 2,
                "CPU and Memory sections should be approximately equal"
            );

            // 4. Layout is valid (no overflows)
            assert!(chunks[0].y < chunks[1].y);
            assert!(chunks[1].y < chunks[2].y);
        }
    }
}

/// Application entry point.
///
/// Sets up the terminal environment, creates the application state,
/// runs the main event loop, and ensures proper cleanup on exit.
///
/// # Returns
/// * `Result<(), Box<dyn Error>>` - Ok if application exits normally
///
/// # Errors
/// * Terminal initialization failures
/// * Event handling errors
/// * Terminal cleanup failures
///
/// # Example
/// ```no_run
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Application setup and execution
///     Ok(())
/// }
/// ```
fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create and run the app
    let mut app = App::new();
    let result = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    // Map io::Error to Box<dyn Error>
    result.map_err(|e| e.into())
}
