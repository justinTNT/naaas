# Sprint 2.5: Minimal Unikernel for Development

## Goal
Build a minimal Express.js unikernel that serves as a development artifact for CLI and infrastructure testing.

## Why Sprint 2.5?
- **Sprint 3 blocker**: Can't develop Lambda deployment without a real unikernel to deploy
- **Demo requirement**: CDK demo needs actual unikernel binary, not mock files
- **Architecture validation**: Need to test the full deployment flow with real artifacts
- **Development velocity**: Fast iteration on CLI requires quick-building test unikernel

## Deliverables

### 1. Welcome Express App
**File**: `src/welcome-app/server.js`

**Requirements:**
- Simple Express.js HTTP server
- Displays hostname (proves it's running on correct EC2 instance)
- Shows request headers (validates NAAAS routing/proxy behavior)
- Shows environment variables (for configuration testing)
- Single-file application for minimal build complexity

**Example Output:**
```html
<h1>Welcome ip-10-0-1-234!</h1>
<p>NAAAS is working 🎉</p>
<p>Port: 3000</p>
<p>Uptime: 15 seconds</p>
<p>Headers: {"x-tenant-id":"blog-123", "host":"test.naaas.com"}</p>
<p>ENV: {"NODE_ENV":"production", "NAAAS_TENANT":"blog-123"}</p>
```

### 2. Unikernel Build Toolchain
**Goal**: Convert Node.js app → bootable unikernel binary

**Tool Options:**
- **NanoVMs ops**: `ops build server.js -o welcome.unikernel`
- **Unikraft**: More complex but potentially better performance
- **OSv**: Alternative unikernel platform

**Decision Criteria:**
- Build speed (for development iteration)
- Documentation quality
- AWS Firecracker compatibility
- Binary size

**Target Output:**
- `welcome.unikernel` - Bootable binary (~10-50MB)
- Boot time: <1 second locally
- HTTP server responding on configured port

### 3. Local Testing Infrastructure
**Goal**: Validate unikernel works before AWS deployment

**Testing Stack:**
- **QEMU/KVM**: Local unikernel execution
- **Networking**: Port forwarding to test HTTP access
- **Automation**: Script to build → run → test → kill

**Test Script:**
```bash
#!/bin/bash
# build-and-test.sh
ops build server.js -o welcome.unikernel
ops run welcome.unikernel -p 3000 &
sleep 5
curl http://localhost:3000
kill %1
```

### 4. Integration Artifacts
**Goal**: Package for CLI development use

**Outputs:**
- `welcome.unikernel` - Binary for CLI testing
- `build-welcome.sh` - Repeatable build script
- `test-welcome.sh` - Local validation script
- `README-unikernel.md` - Build process documentation

## Success Criteria

### Technical Validation
1. **Build Success**: `ops build` completes without errors
2. **Boot Success**: Unikernel starts and serves HTTP within 5 seconds
3. **HTTP Response**: `curl localhost:3000` returns expected HTML
4. **Hostname Display**: Shows actual system hostname in response
5. **Header Forwarding**: Displays request headers correctly

### Development Integration
1. **CLI Testing**: Can use binary with existing `naaas-ctl deploy --binary`
2. **Fast Iteration**: Build time <30 seconds for development changes
3. **Consistent Builds**: Same input produces identical unikernel binary
4. **Documentation**: Clear instructions for rebuilding/modifying

## Architecture Decisions

### Build Tool Selection
**Recommendation**: Start with **NanoVMs ops**
- **Pros**: Simple CLI, good documentation, Node.js support
- **Cons**: Commercial licensing for production use
- **Rationale**: Get development artifact quickly, evaluate alternatives later

### Application Framework
**Decision**: **Express.js** (not raw Node.js HTTP)
- **Pros**: Familiar, well-documented, middleware ecosystem
- **Cons**: Slightly larger binary size
- **Rationale**: Development speed over size optimization

### Testing Strategy
**Decision**: **Local-first** testing before AWS
- **Pros**: Faster iteration, no AWS costs during development
- **Cons**: Not testing AWS Firecracker specifically
- **Rationale**: Catch basic issues early, AWS testing in Sprint 3

## Integration with Sprint 3

### Enables Sprint 3 Development
- **Lambda deployment**: Real binary to deploy via EC2 API
- **ALB testing**: Actual HTTP server to proxy to
- **End-to-end validation**: Complete flow from CLI → running service
- **Demo credibility**: Real unikernel in CDK demo, not mock

### Feeds into Future Sprints
- **Template for Ghost unikernel**: Same build process, different app
- **Build automation**: CI/CD pipeline for unikernel generation
- **Performance baseline**: Measure boot time, memory usage, throughput

## Timeline Estimate
**Duration**: 3-5 days
- Day 1: Express app + NanoVMs setup
- Day 2: Build pipeline + local testing
- Day 3: Integration testing + documentation
- Days 4-5: Buffer for toolchain issues

## Risk Mitigation

### Toolchain Complexity
- **Risk**: NanoVMs ops setup is complex or buggy
- **Mitigation**: Have Unikraft as backup option
- **Fallback**: Use Docker container as "fake unikernel" if needed

### Performance Issues
- **Risk**: Unikernel boot time too slow for development
- **Mitigation**: Focus on correctness first, optimize later
- **Fallback**: Use regular binary deployment if unikernels impractical

### AWS Compatibility
- **Risk**: Local-built unikernel doesn't work on AWS Firecracker
- **Mitigation**: Test early in Sprint 3, adjust build flags if needed
- **Fallback**: Use EC2 with standard VM deployment temporarily

## Deliverable Checklist

**Code Artifacts:**
- [ ] `src/welcome-app/server.js` - Express application
- [ ] `src/welcome-app/package.json` - Dependencies and build config
- [ ] `welcome.unikernel` - Built unikernel binary
- [ ] `build-welcome.sh` - Build automation script
- [ ] `test-welcome.sh` - Local testing script

**Documentation:**
- [ ] `README-unikernel.md` - Setup and build instructions
- [ ] `UNIKERNEL_TESTING.md` - Testing procedures and troubleshooting
- [ ] Tool evaluation notes (ops vs unikraft vs alternatives)

**Validation:**
- [ ] Unikernel boots successfully locally
- [ ] HTTP server responds correctly
- [ ] Headers and hostname display properly
- [ ] Build process is repeatable
- [ ] Integration with existing CLI works

---

**Sprint 2.5 sets the foundation for Sprint 3** by providing the minimal but real artifact needed for AWS deployment development and demo credibility.