# 🖥️ System Monitor

A blazing-fast ⚡, cross-platform system resource monitoring tool built in Rust 🦀. Monitor your CPU and memory usage with a clean, modern terminal-based interface.

## ✨ Features

- 📊 Real-time CPU usage tracking
- 💾 Real-time Memory usage tracking
- 📺 Beautiful terminal-based graphical display
- 🎮 Intuitive keyboard controls
- 🌍 Cross-platform compatibility (Windows, macOS, Linux)
- 🪶 Incredibly lightweight (< 1% CPU usage)

## 🛠️ Prerequisites

- Rust (latest stable version)
- Cargo package manager

## 🚀 Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/system-monitor.git
```

2. Navigate to the project directory:
```bash
cd system-monitor
```

3. Build for release:
```bash
cargo build --release
```

## 🎯 Usage

Launch the application with:

```bash
cargo run
```

For Linux/macOS systems, you might need elevated privileges:
```bash
# Linux 🐧
sudo cargo run

# macOS 🍎
sudo cargo run
```

### 🎮 Controls

- `q` - Quit the application
- `h` - Toggle help menu
- `p` - Pause/Resume monitoring
- `r` - Reset statistics

## 📦 Dependencies

- 📊 sysinfo: Cross-platform system information retrieval
- 🖥️ crossterm: Platform-agnostic terminal manipulation
- 🎨 tui: Terminal user interface rendering

## 🌍 Cross-Platform Support

This application works seamlessly across multiple platforms:
- 🪟 Windows (PowerShell/Command Prompt)
- 🐧 Linux (Various terminals)
- 🍎 macOS (Terminal/iTerm)

No platform-specific configuration needed! The application automatically adapts to your operating system.

## 🏗️ Building for Different Platforms

Build for your target platform using cargo:

```bash
# Windows 🪟
cargo build --target x86_64-pc-windows-msvc

# Linux 🐧
cargo build --target x86_64-unknown-linux-gnu

# macOS (Intel) 🍎
cargo build --target x86_64-apple-darwin

# macOS (Apple Silicon) 🍎
cargo build --target aarch64-apple-darwin
```

## ⚡ Performance

This lightweight system monitoring tool is optimized for minimal resource usage:
- CPU Usage: < 1% on idle
- Memory Footprint: < 10MB
- Startup Time: < 1 second

## 🤝 Contributing

Contributions are welcome! Feel free to:
- 🐛 Report bugs
- 💡 Suggest features
- 🔧 Submit pull requests

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- The Rust community for amazing crates
- This project is GenAI exercise created using [Windsurf](https://codeium.com/windsurf) and refined using [Cursor](https://www.cursor.com/)
