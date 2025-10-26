# Testing Guide - Disk Bloat Scanner

## Overview

This document describes the testing infrastructure and how to run tests for both the Rust backend and Svelte frontend.

## Test Structure

```
disk-bloat-scanner/
├── src-tauri/
│   ├── src/
│   │   └── lib.rs          # Contains inline unit tests
│   └── tests/
│       └── integration_tests.rs  # Integration tests
├── src/
│   ├── lib/
│   │   └── stores.test.js  # Store unit tests
│   └── test/
│       └── setup.js        # Test environment setup
├── vitest.config.js        # Frontend test configuration
└── TESTING.md             # This file
```

## Rust Backend Tests

### Test Categories

1. **Unit Tests** - Inline in `src-tauri/src/lib.rs`
   - Data structure tests
   - Helper function tests
   - Serialization/deserialization tests

2. **Integration Tests** - In `src-tauri/tests/integration_tests.rs`
   - File system operations
   - Test directory structure creation
   - File size verification
   - Duplicate file detection

### Running Rust Tests

```bash
# Run all Rust tests
npm run tauri:test

# Or directly with cargo
cd src-tauri
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_temp_dir_creation

# Run tests in release mode (faster)
cargo test --release
```

### Cargo Clippy Configuration

Strict linting is enabled with the following categories:

- **Pedantic**: Code quality checks
- **Nursery**: Experimental but useful lints
- **Performance**: Performance optimization suggestions
- **Cargo**: Cargo.toml best practices

#### Running Clippy

```bash
# Run clippy with all warnings
npm run tauri:clippy

# Or directly
cd src-tauri
cargo clippy --all-targets --all-features -- -D warnings

# Fix clippy warnings automatically (when possible)
cargo clippy --fix
```

#### Enabled Lints

```toml
unwrap_used = "warn"        # Avoid unwrap() in production code
expect_used = "warn"        # Avoid expect() in production code
panic = "warn"              # Avoid panic!() in production code
todo = "warn"               # Flag incomplete code
unimplemented = "warn"      # Flag unimplemented features
dbg_macro = "warn"          # Remove debug macros before commit
print_stdout = "warn"       # Avoid println! in production
print_stderr = "warn"       # Avoid eprintln! in production
unsafe_code = "warn"        # Flag unsafe blocks
missing_docs = "warn"       # Require documentation
```

### Test Coverage

Current Rust test coverage includes:

- ✅ Directory structure creation
- ✅ File size validation
- ✅ Duplicate file content detection
- ✅ Bloat pattern detection
- ✅ Large file scanning
- ✅ Duplicate file scanning
- ✅ Cleanup operations (dry run & trash)
- ✅ Disk info retrieval
- ✅ System info retrieval
- ✅ Data serialization

## Frontend Tests

### Test Framework

- **Vitest**: Fast unit test framework
- **Testing Library**: DOM testing utilities
- **jsdom**: Browser environment simulation

### Running Frontend Tests

```bash
# Run all frontend tests once
npm test

# Watch mode (runs on file changes)
npm run test:watch

# Run with UI
npm run test:ui

# Generate coverage report
npm run test:coverage

# Run both frontend and backend tests
npm run test:all
```

### Test Files

- `src/lib/stores.test.js` - Tests for Svelte stores
  - diskInfo store
  - summaryStats store
  - settings store
  - bloatCategories store
  - largeFiles store
  - duplicates store
  - isScanning store
  - scanProgress store
  - selectedPaths store

### Writing New Tests

#### Rust Test Example

```rust
#[test]
fn test_my_function() {
    let result = my_function();
    assert_eq!(result, expected_value);
}

#[tokio::test]
async fn test_async_function() {
    let result = async_function().await;
    assert!(result.is_ok());
}
```

#### Frontend Test Example

```javascript
import { describe, it, expect } from 'vitest';
import { get } from 'svelte/store';
import { myStore } from './stores';

describe('My Store', () => {
  it('should have default value', () => {
    const value = get(myStore);
    expect(value).toBe(expectedValue);
  });
});
```

## Continuous Integration

### Pre-commit Checks

Before committing code, run:

```bash
# Format code
cargo fmt
npx prettier --write "src/**/*.{js,svelte}"

# Run linters
npm run tauri:clippy

# Run all tests
npm run test:all
```

### Recommended CI Pipeline

```yaml
# Example GitHub Actions workflow
- name: Test Rust
  run: |
    cd src-tauri
    cargo test --verbose
    cargo clippy --all-targets -- -D warnings

- name: Test Frontend
  run: |
    npm ci
    npm test
    npm run test:coverage
```

## Test Data

### Temporary Test Directories

Integration tests use `tempfile` crate to create isolated test environments:

- Automatically cleaned up after tests
- Safe for parallel test execution
- Simulates real directory structures

### Mock Data

Frontend tests use mocked Tauri API:

```javascript
global.window.__TAURI__ = {
  core: {
    invoke: vi.fn(),
  },
};
```

## Coverage Goals

| Component | Target | Current |
|-----------|--------|---------|
| Rust Backend | 80% | ~75% |
| Frontend Stores | 80% | ~85% |
| Frontend Components | 60% | ~20% |

## Known Test Gaps

- ⚠️ Component rendering tests needed
- ⚠️ User interaction tests needed
- ⚠️ Error handling edge cases
- ⚠️ Performance benchmarks
- ⚠️ Cross-platform test execution (Windows/Linux)

## Troubleshooting

### Rust Tests Failing

```bash
# Clean and rebuild
cd src-tauri
cargo clean
cargo test

# Check for outdated dependencies
cargo update
```

### Frontend Tests Failing

```bash
# Clear cache
rm -rf node_modules
npm install

# Update snapshots (if using)
npm test -- -u
```

### Clippy Too Strict

If clippy warnings are blocking development:

```bash
# Allow specific lint temporarily
cargo clippy -- -A clippy::unwrap_used

# Or configure in Cargo.toml
[lints.clippy]
unwrap_used = "allow"
```

## Best Practices

### Rust

1. ✅ Use `#[test]` for sync tests, `#[tokio::test]` for async
2. ✅ Use `assert!`, `assert_eq!`, `assert_ne!` for assertions
3. ✅ Use `Result<(), Box<dyn Error>>` for tests that can fail
4. ✅ Create helper functions for common test setup
5. ✅ Use descriptive test names: `test_<function>_<scenario>`

### Frontend

1. ✅ Test behavior, not implementation
2. ✅ Use `describe` blocks to group related tests
3. ✅ Reset store state in `beforeEach` hooks
4. ✅ Mock external dependencies (Tauri API)
5. ✅ Test user-facing functionality

## Resources

- [Rust Testing Documentation](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Vitest Documentation](https://vitest.dev/)
- [Testing Library](https://testing-library.com/)
- [Tauri Testing Guide](https://v2.tauri.app/develop/tests/)

---

**Last Updated**: October 24, 2025  
**Version**: 0.1.0-alpha
