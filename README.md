# Disk Bloat Scanner

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.89+-000000.svg)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-2.0+-24c8db.svg)](https://tauri.app/)
[![Svelte](https://img.shields.io/badge/Svelte-4+-ff3e00.svg)](https://svelte.dev/)

A powerful, cross-platform desktop application for scanning and cleaning disk bloat. Built with Rust and Tauri for performance, featuring a modern Svelte UI.

![Disk Bloat Scanner Screenshot](screenshots/main-interface.png)

## ✨ Features

- **Disk Analysis**: Real-time disk usage monitoring across all mounts
- **Bloat Detection**: Intelligent scanning for large directories and files
- **Duplicate Finder**: SHA256-based duplicate file detection
- **Safe Cleanup**: Secure file removal with trash integration
- **Modern UI**: Responsive Svelte interface with TailwindCSS
- **Cross-Platform**: Native binaries for macOS, Windows, and Linux
- **Privacy-Focused**: Local processing, no data collection

## 🚀 Quick Start

### Prerequisites
- [Rust 1.89+](https://rustup.rs/)
- [Node.js 18+](https://nodejs.org/)
- For macOS: Xcode Command Line Tools
- For Windows: Visual Studio Build Tools
- For Linux: WebKitGTK and appindicator

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/disk-bloat-scanner.git
cd disk-bloat-scanner

# Install dependencies
npm install

# Run in development mode
npm run dev
# or build for production
npm run build
```

### Download Releases
Download the latest release from [GitHub Releases](https://github.com/yourusername/disk-bloat-scanner/releases).

## 📖 Usage

1. **Launch the app** and grant necessary permissions
2. **View disk info** to see current usage across drives
3. **Scan for bloat** by selecting a root directory and minimum size
4. **Find duplicates** to identify identical files
5. **Clean up** selected items safely (moves to trash)

### Settings
- Adjust minimum file size thresholds
- Configure scan depth and exclusions
- Toggle symlink following

![Settings Panel](screenshots/settings-panel.png)

## 🏗️ Architecture

```
┌─────────────────┐    IPC    ┌─────────────────┐
│   Svelte UI     │◄────────►│   Rust Backend   │
│                 │           │                 │
│ • Components    │           │ • Scan Engine   │
│ • State Mgmt    │           │ • File Ops      │
│ • Responsive    │           │ • Async Tasks   │
└─────────────────┘           └─────────────────┘
         │                               │
         └───────────────────────────────┘
              Tauri Runtime
```

## 🧪 Development

```bash
# Install Tauri CLI
cargo install tauri-cli

# Run with hot reload
cargo tauri dev

# Build release
cargo tauri build
```

### Project Structure
```
disk-bloat-scanner/
├── src-tauri/          # Rust backend
│   ├── src/
│   └── tauri.conf.json
├── ui/                 # Svelte frontend
│   ├── src/
│   └── vite.config.js
├── docs/               # Documentation
└── package.json
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit changes: `git commit -m 'Add amazing feature'`
4. Push to branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

### Development Setup
- Follow the installation steps above
- Run tests: `cargo test` and `npm test`
- Use the BD issue tracker for task management

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Tauri](https://tauri.app/) for the amazing desktop framework
- [Svelte](https://svelte.dev/) for the reactive UI framework
- [TailwindCSS](https://tailwindcss.com/) for utility-first styling
- [Beads](https://github.com/steveyegge/beads) for issue tracking

## 📞 Support

- 📧 Email: support@diskbloatscanner.com
- 🐛 Issues: [GitHub Issues](https://github.com/yourusername/disk-bloat-scanner/issues)
- 📖 Docs: [Full Documentation](https://diskbloatscanner.com/docs)

---

**Made with ❤️ using Rust and Tauri**</content>
</xai:function_call">Create a comprehensive README.md file for the repository