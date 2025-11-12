# Error Handling Policy

## Overview

The Disk Bloat Scanner enforces strict error handling practices to ensure production safety and prevent panics that could crash the application or leave the system in an inconsistent state.

## Core Principles

1. **No Panics in Production**: Production code must never panic. All potential failures must be handled gracefully.
2. **Descriptive Errors**: All errors must provide clear, actionable information about what went wrong.
3. **Error Context**: Errors should include relevant context to help diagnose and fix issues.
4. **Type Safety**: Use the type system to make invalid states unrepresentable.

## Enforcement Mechanisms

### 1. Compile-Time Enforcement

The following directives in `src-tauri/src/lib.rs` prevent dangerous code from compiling:

```rust
#![forbid(clippy::unwrap_used)]
#![forbid(clippy::expect_used)]
```

### 2. Clippy Configuration

The `clippy.toml` file disallows panic-inducing methods and macros:
- `Option::unwrap()` - Use `.ok_or()` or pattern matching instead
- `Result::unwrap()` - Use the `?` operator or proper error handling
- `Option::expect()` - Use `.ok_or_else()` with a proper error
- `Result::expect()` - Use `.map_err()` and `?` for error context
- `panic!()` macro - Return errors instead
- `unimplemented!()` and `todo!()` - Complete the implementation or return an error

### 3. Error Type System

We use `thiserror` for deriving error implementations, providing:
- Automatic `Display` and `Error` trait implementations
- Source error chaining with `#[from]`
- Formatted error messages with `#[error()]`
- Transparent error wrapping

## Best Practices

### DO: Proper Error Handling

```rust
// Good: Using the ? operator
let contents = std::fs::read_to_string(&path)?;

// Good: Providing context
let file = File::open(&path)
    .map_err(|e| ScannerError::FileAccess { 
        path: path.display().to_string(), 
        source: e 
    })?;

// Good: Pattern matching for Options
match some_option {
    Some(value) => process(value),
    None => return Err(ScannerError::NotFound("item".into())),
}

// Good: Using ok_or for Option to Result conversion
let value = some_option
    .ok_or_else(|| ScannerError::InvalidConfig("missing value".into()))?;
```

### DON'T: Panic-Inducing Code

```rust
// Bad: Direct unwrap
let value = some_option.unwrap();  // FORBIDDEN

// Bad: Using expect
let file = File::open(path).expect("file should exist");  // FORBIDDEN

// Bad: Panic macro
if condition {
    panic!("This should never happen");  // FORBIDDEN
}

// Bad: Unimplemented placeholder
fn new_feature() {
    unimplemented!();  // FORBIDDEN
}
```

## Testing Exception

Test code is explicitly allowed to use `unwrap()` and `expect()` for simplicity. All test modules include:

```rust
#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic
    )]
    // Test code here
}
```

## Error Categories

Our `ScannerError` enum provides specific error variants for different failure modes:

- **Io**: File system I/O errors with OS error details
- **InvalidPath**: Path validation failures with the problematic path
- **PermissionDenied**: Access control violations
- **NotFound**: Missing resources
- **InvalidConfig**: Configuration errors
- **DeletionFailed**: Cleanup operation failures
- **InvalidFloatComparison**: NaN/Inf in numeric operations
- **LockPoisoned**: Concurrent access errors
- **Other**: Generic errors with custom messages

## Migration Guide

When encountering code with `unwrap()` or `expect()`:

1. **Identify the error case**: What could cause this to fail?
2. **Choose appropriate handling**:
   - If failure is truly impossible, document why with a comment
   - Otherwise, propagate the error with `?` or handle it explicitly
3. **Add context**: Use `.map_err()` to add helpful error context
4. **Test error paths**: Ensure error cases are covered by tests

## Verification

Run these commands to verify compliance:

```bash
# Check for forbidden patterns
cargo clippy -- -D clippy::unwrap_used -D clippy::expect_used

# Run all tests to ensure nothing broke
cargo test

# Check for any remaining unwraps in production code
rg "\.unwrap\(\)" src-tauri/src --glob '!*.rs' -t rust | grep -v test
```

## Conclusion

This policy ensures our application remains stable and provides helpful error messages when things go wrong. By following these guidelines, we create a more reliable and maintainable codebase.