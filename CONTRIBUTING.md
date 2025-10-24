# Contributing to Disk Bloat Scanner

Thank you for your interest in contributing to Disk Bloat Scanner! This document provides guidelines and instructions for contributing to this Tempext Genesis showcase project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Development Workflow](#development-workflow)
- [Testing](#testing)
- [Code Style](#code-style)
- [Commit Messages](#commit-messages)
- [Pull Request Process](#pull-request-process)
- [Architecture Guidelines](#architecture-guidelines)

## Code of Conduct

Be respectful, constructive, and professional. We're building quality software together.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/disk-bloat-scanner.git
   cd disk-bloat-scanner
   ```
3. **Add upstream remote**:
   ```bash
   git remote add upstream https://github.com/ORIGINAL_OWNER/disk-bloat-scanner.git
   ```

## Development Setup

### Prerequisites

- **Rust**: 1.77.2 or later
- **Node.js**: 18.0.0 or later
- **npm**: 9.0.0 or later
- **Platform**: macOS (primary), Windows/Linux (experimental)

### Installation

```bash
# Install dependencies
npm install

# Install Rust dependencies (automatic on first build)
cd src-tauri && cargo build
```

### Running in Development

```bash
# Start development server
npm run tauri:dev

# This runs both:
# - Frontend dev server (Vite) on port 3001
# - Tauri backend in dev mode
```

## Project Structure

```
disk-bloat-scanner/
├── .github/               # GitHub templates and workflows
│   ├── ISSUE_TEMPLATE/   # Issue templates
│   └── workflows/        # CI/CD workflows
├── docs/                 # Documentation
│   ├── design/          # Design specifications
│   └── schedule/        # Development schedules
├── src/                 # Frontend (Svelte)
│   ├── lib/
│   │   ├── components/ # Svelte components
│   │   └── stores.js   # State management
│   ├── test/           # Frontend tests
│   ├── App.svelte      # Main component
│   ├── app.css         # Tailwind CSS
│   └── main.js         # Entry point
├── src-tauri/           # Backend (Rust)
│   ├── src/
│   │   ├── lib.rs      # Core logic
│   │   └── main.rs     # Tauri entry
│   ├── tests/          # Integration tests
│   └── Cargo.toml      # Rust dependencies
├── public/              # Static assets
├── .gitignore          # Git ignore rules
├── LICENSE             # Apache 2.0 license
├── README.md           # Project readme
├── CONTRIBUTING.md     # This file
├── TESTING.md          # Testing guide
└── package.json        # NPM dependencies
```

## Development Workflow

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/your-bug-fix
```

### 2. Make Changes

- Write clean, documented code
- Follow existing patterns and conventions
- Add tests for new features
- Update documentation as needed

### 3. Test Your Changes

```bash
# Run frontend tests
npm test

# Run Rust tests
npm run tauri:test

# Run lint checks
npm run tauri:clippy

# Run all tests
npm run test:all
```

### 4. Format Code

```bash
# Format Rust code
cargo fmt

# Format frontend code
npx prettier --write "src/**/*.{js,svelte,css,html}"
```

### 5. Commit Changes

Follow our commit message guidelines (see below).

### 6. Push and Create PR

```bash
git push origin feature/your-feature-name
```

Then create a Pull Request on GitHub.

## Testing

### Rust Tests

```bash
# Run all Rust tests
cd src-tauri && cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_scan_bloat
```

### Frontend Tests

```bash
# Run once
npm test

# Watch mode
npm run test:watch

# Coverage
npm run test:coverage
```

### Integration Tests

Integration tests are in `src-tauri/tests/`. They test full workflows with temporary file systems.

## Code Style

### Rust

- Follow standard Rust naming conventions
- Use `Result<T, E>` for error handling
- Avoid `unwrap()` and `panic!()` in production code
- Document public APIs with `///` comments
- Run `cargo clippy` before committing

**Enforced Lints:**
- `pedantic`, `nursery`, `perf` - Code quality
- `unwrap_used`, `expect_used` - Avoid unsafe unwrapping
- `panic`, `todo`, `unimplemented` - Flag incomplete code

### Frontend (Svelte/JavaScript)

- Use ES6+ syntax
- Prefer `const` over `let`
- Use descriptive variable names
- Document complex logic with comments
- Run `prettier` before committing

### Tailwind CSS

- Use Tailwind utility classes
- Define custom colors in `@theme` block
- Avoid inline styles when possible
- Keep component classes organized

## Commit Messages

Follow the Conventional Commits specification:

```
type(scope): subject

body

footer
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code refactoring
- `test`: Adding/updating tests
- `chore`: Maintenance tasks

### Examples

```
feat(scanner): add support for .git directory scanning

- Implement git-aware scanning logic
- Add tests for git repository detection
- Update documentation

Closes #123
```

```
fix(ui): correct delete confirmation dialog behavior

The native dialog API was returning false unexpectedly.
Replaced browser confirm() with Tauri ask() API.

Fixes #456
```

## Pull Request Process

### Before Submitting

1. ✅ All tests pass (`npm run test:all`)
2. ✅ Code is formatted (`cargo fmt`, `prettier`)
3. ✅ Clippy passes (`npm run tauri:clippy`)
4. ✅ Documentation updated
5. ✅ Commit messages follow guidelines
6. ✅ Branch is up to date with main

### PR Template

```markdown
## Description
[Describe your changes]

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
[Describe testing performed]

## Screenshots
[If applicable]

## Checklist
- [ ] Tests pass
- [ ] Code formatted
- [ ] Documentation updated
- [ ] Reviewed my own code
```

### Review Process

- PRs require at least one approval
- Address review comments promptly
- Keep PRs focused and reasonably sized
- Rebase if requested

## Architecture Guidelines

This project follows Tempext Genesis principles:

### Frontend (Svelte + Tailwind + Vite)

- **Component-based**: Each component has single responsibility
- **State management**: Use Svelte stores for shared state
- **Styling**: Tailwind CSS 4 with theme customization
- **Build**: Vite for fast dev server and builds

### Backend (Rust + Tauri)

- **Commands**: Async Tauri commands for IPC
- **Error handling**: Use `Result<T, String>` for commands
- **Concurrency**: Use Rayon for parallel operations
- **Safety**: No unwrap/panic in production code

### Communication

- **IPC**: Tauri's `invoke()` for frontend → backend
- **Events**: (Future) Tauri events for backend → frontend streaming

### Testing

- **Unit tests**: Test individual functions
- **Integration tests**: Test full workflows
- **Component tests**: Test UI components
- **Coverage goal**: 80% for backend, 60% for frontend

## Questions?

- **Issues**: Use GitHub Issues for bugs and features
- **Discussions**: Use GitHub Discussions for questions
- **Documentation**: Check README.md and TESTING.md

## License

By contributing, you agree that your contributions will be licensed under the Apache 2.0 License.

---

**Thank you for contributing to Disk Bloat Scanner!**

This is a Tempext Genesis showcase project demonstrating best practices for:
- Tauri desktop app development
- Rust/Svelte integration
- Modern frontend tooling (Vite, Tailwind CSS 4)
- Comprehensive testing
- Open source collaboration
