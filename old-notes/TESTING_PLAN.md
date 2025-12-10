# NAAAS Comprehensive Testing Plan

## Executive Summary

This plan addresses the **critical testing gap** in NAAAS Sprint 1 components. Currently, `naaas-server`, `naaas-ctl`, and `naaas-shim` have **zero unit tests** despite implementing core tenant lifecycle functionality.

**Goal**: Establish comprehensive test coverage that enables confident development and validates the core "launch unikernel" Sprint 1 requirement.

---

## Current State Analysis

### Codebase Structure
- **3 Rust projects**: `naaas-server` (199 LOC), `naaas-ctl` (214 LOC), `naaas-shim` (170 LOC)  
- **Total**: ~600 lines of untested production logic
- **Dependencies**: Standard HTTP/async stack (tokio, hyper, reqwest, serde)
- **Architecture**: Server API + CLI client + transparent proxy

### Critical Testing Gaps
1. **Zero unit tests** across all components
2. **No integration tests** for server ↔ CLI interaction  
3. **Missing property-based tests** for tenant lifecycle invariants
4. **No error handling validation** for network failures, invalid JSON, process management
5. **Unvalidated concurrency** - multiple simultaneous deployments could conflict

---

## Testing Strategy Overview

### Hybrid Testing Approach

#### Layer 1: Unit Tests (Rust - Internal Logic)
- **Focus**: Core Rust logic, data structures, process management
- **Coverage Target**: 80% line coverage of critical internal functions
- **Language**: Rust with **extreme readability focus**
- **Tools**: Standard Rust `#[test]`, `tokio-test` for async
- **Priority**: Readable over idiomatic - clear structure, descriptive names, minimal Rust magic

#### Layer 2: Integration Tests (TypeScript - API Workflows)  
- **Focus**: HTTP API contracts, user workflows, error handling
- **Coverage**: End-to-end scenarios (deploy → list → delete)
- **Language**: TypeScript with Jest/Vitest
- **Tools**: Axios/fetch for HTTP, JSON schema validation, mock servers
- **Priority**: Fast iteration, rich assertions, natural JSON handling

#### Layer 3: Property-Based Tests (Sidecar-Generated)
- **Focus**: Invariants and edge cases across the system
- **Coverage**: Concurrent tenant management, configuration validation
- **Language**: Clojure generates cases → TypeScript executes them
- **Tools**: Sidecar property generators + TypeScript test runner

---

## Detailed Test Plan by Component

### 1. `naaas-server` Testing

#### 1.1 Unit Tests (Rust - Readable Focus)

**API Handler Functions** - Testing core business logic:
- `handle_deploy()` 
  - ✅ Valid deployment request creates tenant with correct data
  - ✅ Invalid JSON input returns clear error message  
  - ✅ Missing unikernel path fails with helpful validation error
  - ✅ Port conflict detection and automatic allocation
  - ✅ Process spawn failure is handled gracefully

- `handle_list_tenants()`
  - ✅ Empty tenant store returns empty array (not null)
  - ✅ Multiple tenants returned in predictable order
  - ✅ All tenant fields serialize correctly to JSON

- `handle_delete_tenant()`  
  - ✅ Existing tenant removal cleans up store completely
  - ✅ Non-existent tenant returns 404 with clear message
  - ✅ Process termination succeeds when tenant deleted
  - ✅ All tenant resources cleaned up properly

**Data Structures** - Core type safety:
- `Tenant` JSON serialization roundtrip preserves all fields
- `DeployRequest` validation catches missing required fields
- `TenantStore` thread safety under concurrent access

**Rust Test Readability Guidelines**:
- **Descriptive names**: `test_deploy_with_missing_name_returns_validation_error()`
- **Clear structure**: Setup → Action → Assert with comments
- **Explicit assertions**: One concept per assertion with helpful error messages
- **Minimal Rust complexity**: Avoid advanced patterns, prefer verbose clarity

#### 1.2 Integration Tests (TypeScript - API Workflows)

**Full API Workflows** - End-to-end user scenarios:
- Deploy → List → Verify → Delete → Confirm removal
- Multiple concurrent deployments don't conflict
- Server startup/shutdown behavior is clean
- Health endpoint returns proper status

**Error Scenarios** - Real-world failure modes:
- Invalid unikernel binary path returns clear error
- Network errors during process spawn handled gracefully
- Resource exhaustion (port allocation) fails safely
- Malformed JSON requests return helpful error messages

**TypeScript Test Benefits**:
- **Natural HTTP testing**: Axios/fetch with excellent JSON handling
- **Rich assertions**: Jest matchers for HTTP status, response structure
- **Fast iteration**: No Rust compilation during test development
- **Familiar syntax**: Standard JS/TS testing patterns

#### 1.3 Property-Based Tests (Priority: Medium)

**System Invariants**:
- Tenant IDs are always unique across deployments
- Port assignments never conflict
- Process IDs accurately track spawned processes
- Tenant store consistency under concurrent operations

### 2. `naaas-ctl` Testing  

#### 2.1 Unit Tests (Rust - CLI Logic Only)

**CLI Command Parsing** - Argument validation:
- `Deploy` command parses all argument combinations correctly
- `List` command handles basic invocation without errors
- `Delete` command validates tenant ID format
- `Health` command processes server URL properly
- Server URL parsing applies correct defaults

**Note**: HTTP client functions will be tested via TypeScript integration tests for better HTTP tooling.

#### 2.2 Integration Tests (TypeScript - Full Workflows)

**CLI ↔ Server Integration** - Real user scenarios:  
- Full deployment workflow: CLI command → HTTP request → server response → CLI output
- Error propagation: Server errors displayed clearly to user
- Command output formatting provides helpful user experience
- All CLI commands work against running server

**Network Scenario Testing**:
- CLI behavior against various server response codes (200, 400, 404, 500)
- Network timeout and connection failure handling
- Server unavailable scenarios return helpful messages
- JSON response parsing handles malformed server responses

### 3. `naaas-shim` Testing

#### 3.1 Unit Tests (Priority: High)

**Proxy Logic**:
- `proxy_request()` URL construction and forwarding
- Header manipulation (hop-by-hop header removal)
- Query parameter preservation
- HTTP method preservation

**Configuration Handling**:
- `handle_config()` JSON response formation
- `AppConfig` parsing from command line arguments
- Default configuration fallback behavior

#### 3.2 Integration Tests (Priority: High)

**Transparent Proxy Behavior**:
- HTTP requests correctly forwarded to upstream
- Response data preservation from upstream
- Error handling for upstream connectivity issues
- `/config` endpoint functionality independent of upstream

**Real Upstream Integration**:
- Proxy against actual Ghost CMS instance (using existing test-sprint2.sh setup)
- Static file serving through proxy
- HTTP error code propagation

---

## Test Infrastructure Requirements

### Dependencies to Add

**Rust Unit Testing** (minimal, readable):
```toml
[dev-dependencies]
tokio-test = "0.4"     # Async test utilities
assert_matches = "1.5" # Readable pattern matching
tempfile = "3.8"       # Temporary test files
```

**TypeScript Integration Testing**:
```json
// package.json
{
  "devDependencies": {
    "vitest": "^1.0.0",        // Fast test runner
    "axios": "^1.6.0",         // HTTP client
    "@types/node": "^20.0.0",  // Node.js types
    "zod": "^3.22.0"          // JSON schema validation
  }
}
```

**Property-Based Testing** (sidecar-generated):
- Clojure generators in existing sidecar
- TypeScript execution via Vitest

### Test Data Management

**Mock Unikernels**: Create simple test binaries that simulate unikernel behavior
**Configuration Files**: JSON test fixtures for various tenant configurations  
**Test Servers**: Spawn actual server instances for integration testing
**Cleanup Utilities**: Ensure test isolation and resource cleanup

---

## Test Organization Structure

### Directory Layout
```
src/
├── naaas-server/
│   ├── src/
│   │   ├── main.rs
│   │   ├── handlers.rs     # Extract from main.rs  
│   │   ├── models.rs       # Extract Tenant/DeployRequest
│   │   └── lib.rs          # Public interface for testing
│   └── tests/              # Rust unit tests only
│       ├── handlers_test.rs
│       ├── models_test.rs
│       └── lib.rs
├── naaas-ctl/
│   └── tests/              # Rust unit tests only
│       └── cli_test.rs
├── naaas-shim/
│   └── tests/              # Rust unit tests only
│       └── proxy_test.rs
└── tests/                  # TypeScript integration tests
    ├── package.json
    ├── vitest.config.ts
    ├── integration/
    │   ├── server-api.test.ts
    │   ├── cli-workflows.test.ts
    │   ├── shim-proxy.test.ts
    │   └── end-to-end.test.ts
    └── helpers/
        ├── test-server.ts
        ├── mock-binaries.ts
        └── cleanup.ts
```

### Test Execution Strategy

**Fast Unit Tests**: Run on every code change (`cargo test` in each component)
**Integration Tests**: Run before commits (`npm test` in `tests/` directory)  
**Property Tests**: Run nightly + before releases (sidecar → TypeScript execution)
**Cross-Component**: Full system test via `test-sprint2.sh` enhancement

**Language-Specific Benefits**:
- **Rust unit tests**: Type safety catches test bugs, direct access to internals
- **TypeScript integration**: Natural HTTP/JSON handling, fast iteration, familiar syntax
- **Hybrid approach**: Best tool for each layer, readable Rust where necessary

---

## Implementation Phases

### Phase 1: Foundation (Week 1)
1. **Refactor for testability**: Extract handlers and models into separate modules
2. **Add test dependencies**: Update all Cargo.toml files
3. **Create test infrastructure**: Mock utilities, test data, cleanup helpers

### Phase 2: Unit Test Core (Week 1-2)  
1. **naaas-server unit tests**: API handlers and data structures
2. **naaas-ctl unit tests**: CLI parsing and HTTP client functions
3. **naaas-shim unit tests**: Proxy logic and configuration handling

### Phase 3: Integration Testing (Week 2)
1. **Component integration**: Server ↔ CLI interaction tests
2. **Real backend testing**: Shim ↔ Ghost CMS integration  
3. **Error scenario coverage**: Network failures, invalid configurations

### Phase 4: Property-Based Testing (Week 3)
1. **Sidecar integration**: Use existing property test generators
2. **Invariant validation**: Concurrent tenant management properties
3. **Edge case discovery**: Automated test case generation

### Phase 5: Continuous Testing (Week 3)
1. **CI integration**: Automated test execution
2. **Coverage measurement**: Ensure 80%+ coverage maintained
3. **Performance regression**: Benchmark critical paths

---

## Success Criteria

### Quantitative Targets
- **80% line coverage** across all components  
- **Zero failing tests** in CI pipeline
- **Sub-100ms unit test suite** execution time
- **Property tests find 0 invariant violations**

### Qualitative Targets  
- **Confident refactoring**: Tests enable safe code changes
- **Sprint 1 validation**: "Launch unikernel" requirement thoroughly tested
- **Regression prevention**: New features can't break existing functionality
- **Documentation**: Tests serve as executable specifications

---

## Risk Mitigation

### Technical Risks
- **Test complexity**: Start with simple cases, build up gradually
- **Async testing**: Use `tokio-test` for deterministic async behavior
- **Process testing**: Careful cleanup to prevent resource leaks
- **Timing issues**: Avoid time-dependent assertions in tests

### Schedule Risks  
- **Parallel development**: Unit tests can be written alongside feature development
- **Incremental delivery**: Each component can be tested independently
- **Fallback plan**: Focus on server tests first as highest impact

---

## Next Steps for Review

1. **Approve overall strategy**: 3-layer testing approach and coverage targets
2. **Prioritize components**: Confirm naaas-server → naaas-ctl → naaas-shim order  
3. **Review test infrastructure**: Dependencies and directory structure
4. **Schedule coordination**: Align testing phases with Sprint 1 development
5. **Resource allocation**: Confirm dedicated time for comprehensive testing

This plan transforms NAAAS from an untested prototype into a production-ready system with confidence guarantees for the critical "launch unikernel" Sprint 1 goal.