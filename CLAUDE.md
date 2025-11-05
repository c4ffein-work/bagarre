# Claude AI Assistant Guidelines for Bagarre Project

## Verification Requirement

**CRITICAL: Before declaring any task complete, you MUST run `make verify` and ensure it passes.**

### Verification Rules

1. **Always run `make verify`** before saying a task is done
2. **If `make verify` fails:**
   - Fix the issues causing the failure
   - Run `make verify` again
   - Only after it passes can you declare the task complete
3. **Never say "we're done" or "task complete" unless `make verify` passes**
4. **If you cannot fix the failures:**
   - Clearly explain what's failing and why
   - Ask the user for their opinion on how to proceed
   - Do NOT claim the task is complete

### What `make verify` Checks

The verify command runs all CI/CD pipeline checks:
- Code formatting (`cargo fmt --check`)
- Linting with Clippy (`cargo clippy` with warnings as errors)
- Full test suite (`cargo test --verbose`)

## Development Workflow

When making changes to the codebase:

1. Make your changes
2. Run `make fmt` to format the code
3. Run `make verify` to ensure all checks pass
4. Commit and push only if verification passes

## Available Make Commands

- `make verify` - Run all checks (REQUIRED before completion)
- `make test` - Run tests only
- `make lint` - Run clippy only
- `make fmt-check` - Check formatting only
- `make fmt` - Auto-format code
- `make build-wasm` - Build WASM binary for browser demo
- `make clean` - Clean build artifacts

## Project Structure

- `src/` - Core engine code
- `tests/` - Integration and E2E tests
- `examples/` - Example implementations
- `examples/browser/` - Browser demo with WASM
- `.github/workflows/` - CI/CD pipeline configuration
