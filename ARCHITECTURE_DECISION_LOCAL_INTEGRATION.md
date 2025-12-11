# Architecture Decision: Local Integration Testing Strategy

## Context

During Sprint 2.5 planning, we identified that building unikernels locally on macOS for testing purposes introduces significant complexity with minimal validation value:

- **macOS lacks Xen support** (required for many unikernel platforms)
- **Local QEMU ≠ AWS Firecracker** (different hypervisors, different behavior)
- **Complex toolchain setup** for limited testing value
- **AWS is our primary target** - we should test in the real environment

## Decision

**We will test shim ↔ application integration locally using regular processes, bypassing unikernel complexity entirely.**

### Local Development Flow

```bash
# Terminal 1: Express app (represents tenant application)
cd src/welcome-app
PORT=3001 node server.js

# Terminal 2: Rust shim (represents NAAAS proxy layer)
cd src/naaas-shim  
cargo run -- --port 3000 --upstream http://localhost:3001

# Terminal 3: Integration testing
curl http://localhost:3000/  # Tests shim → express proxy
```

### AWS Deployment Flow

```bash
# Same components, but shim becomes Firecracker unikernel
Internet → ALB → naaas-shim (Firecracker) → Express app (EC2 process)
```

## Architecture Clarity

**Component Responsibilities:**

1. **naaas-shim (Rust)**
   - **Local**: Regular process for integration testing
   - **AWS**: Compiled to Firecracker unikernel 
   - **Purpose**: HTTP proxy with NAAAS routing/auth logic

2. **Tenant Applications (Express/Ghost/etc.)**
   - **Local**: Regular process behind shim
   - **AWS**: Regular EC2 process behind shim
   - **Purpose**: Application logic, content serving

3. **NAAAS Platform (Go server + CLI)**
   - **Purpose**: Deploy/manage shim + application pairs

## Benefits

### Development Velocity
- ✅ **No local unikernel toolchain complexity**
- ✅ **Fast iteration** - cargo run for shim changes
- ✅ **Familiar debugging** - standard Rust/Node.js tools
- ✅ **Immediate feedback** - see integration issues instantly

### Validation Quality
- ✅ **Tests actual integration logic** (shim ↔ app communication)
- ✅ **Validates proxy behavior** (headers, routing, error handling)
- ✅ **Proves tenant isolation model** (multiple apps behind same shim)
- ✅ **AWS testing validates production reality** (real Firecracker behavior)

### Cost Efficiency
- ✅ **Local testing is free** - no AWS costs for development
- ✅ **AWS staging is cheap** - minimal Firecracker instance costs
- ✅ **No wasted time** on local unikernel toolchain complexity

## Implementation Details

### Express App Enhancements
The welcome Express app will demonstrate unikernel-specific benefits:

```javascript
// Show tenant-specific infrastructure
- Hostname (proves dedicated instance)
- Environment variables (shows tenant configuration)
- Background color (visual proof of per-tenant customization)
- Request headers (validates shim proxy behavior)
```

### Shim Configuration
```bash
# Basic proxy configuration
naaas-shim --port 3000 --upstream http://localhost:3001

# With tenant-specific config
naaas-shim --port 3000 --upstream http://localhost:3001 \
  --config '{"name":"alice-blog","primary_color":"#4299e1"}'
```

### Integration Test Scenarios
1. **Basic proxy** - shim forwards requests to Express app
2. **Header forwarding** - Express app sees original request headers
3. **Error handling** - shim handles upstream failures gracefully
4. **Configuration** - tenant-specific config affects app behavior
5. **Multiple tenants** - same Express app, different configurations

## Alternatives Considered

### Rejected: Local Unikernel Building
- **Complexity**: NanoVMs ops, Unikraft toolchain setup
- **Platform mismatch**: macOS QEMU ≠ AWS Firecracker
- **Limited value**: Doesn't test AWS integration reality
- **Time investment**: Days of toolchain work for minimal validation

### Rejected: Skip Integration Testing
- **Risk**: Deploy to AWS without validating shim ↔ app communication
- **Debugging complexity**: Hard to isolate issues in AWS environment
- **Cost**: More expensive to debug integration issues in cloud

## Success Metrics

### Local Integration Testing
- ✅ Shim successfully proxies requests to Express app
- ✅ Express app receives correct headers/environment
- ✅ Multiple tenant configurations work simultaneously  
- ✅ Error scenarios handled gracefully
- ✅ Fast development iteration (< 5 seconds to test changes)

### AWS Deployment Validation
- ✅ Shim compiles to working Firecracker unikernel
- ✅ Same Express app works behind Firecracker shim
- ✅ End-to-end tenant deployment via Lambda → EC2
- ✅ Multiple tenants isolated properly

## Timeline Impact

**Before Decision**: Sprint 2.5 estimated at 5-7 days
- Day 1-2: macOS unikernel toolchain setup
- Day 3-4: Local unikernel building/testing
- Day 5-7: AWS deployment and integration

**After Decision**: Sprint 2.5 estimated at 2-3 days  
- Day 1: Local shim ↔ express integration testing
- Day 2: AWS Firecracker deployment
- Day 3: End-to-end validation

**Result**: 50%+ faster Sprint 2.5 completion with higher confidence in AWS integration.

## Future Implications

This decision establishes a pattern for all future NAAAS development:

1. **Test integration locally** using regular processes
2. **Deploy to AWS staging** for production environment validation  
3. **Avoid local infrastructure simulation** when real environment is available
4. **Focus development time** on business logic, not toolchain complexity

This approach scales to future features like multi-region deployment, auto-scaling, and complex tenant configurations.

---

**Decision Status**: ✅ **Approved**  
**Implementation**: Sprint 2.5  
**Review Date**: After Sprint 3 completion