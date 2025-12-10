# NAAAS Server: Go Rewrite Plan

## 🎯 **Strategic Decision**

**Decision:** Rewrite `naaas-server` from Rust to Go  
**Reasoning:** The server is a **fleet manager/orchestrator**, not a data processor. It handles process lifecycle, configuration, and APIs - classic Go strengths.

**Architecture Clarity:**
- **Control Plane (Go):** `naaas-server` - orchestration, APIs, management
- **Data Plane (Rust):** `naaas-shim` - performance-critical HTTP proxy
- **CLI (Go):** `naaas-ctl` - single binary distribution
- **Dashboard (Separate):** Frontend webapp consuming server APIs

---

## 🚀 **Go Benefits: Maximization Strategy**

### **1. Development Velocity**

**The Benefit:** Go's simplicity = faster iteration on control plane features

**How to maximize:**
```go
// Use Go's strengths deliberately:
- Standard `net/http` (not heavy frameworks initially)
- `encoding/json` for APIs
- `os/exec` for process management
- `context` for request cancellation
- Built-in testing with `go test`
```

**Tactical moves:**
- ✅ Start with stdlib, add frameworks later when needed
- ✅ Leverage `go generate` for repetitive code
- ✅ Use Go's excellent tooling (`go fmt`, `go vet`, `go mod`)

### **2. Single Binary Deployment**

**The Benefit:** Zero dependency headaches, trivial Docker images, easy AMI packaging

**How to maximize:**
```bash
# Make distribution dead simple:
CGO_ENABLED=0 go build -ldflags="-s -w" -o naaas-server

# Result: ~10MB static binary that runs anywhere
```

**Tactical moves:**
- ✅ Design for configuration via environment variables
- ✅ Embed static assets with `go:embed` if needed
- ✅ Cross-compile for all target platforms from day one

### **3. Process Orchestration Excellence**

**The Benefit:** `os/exec` is battle-tested, goroutines perfect for managing concurrent processes

**How to maximize:**
```go
// Design for robust process management:
type TenantManager struct {
    processes map[string]*exec.Cmd
    ctx       context.Context
    cancel    context.CancelFunc
}

// Built-in supervision, health checking, graceful shutdown
```

**Tactical moves:**
- ✅ Use `context.Context` for proper cancellation
- ✅ Build process supervision from day one
- ✅ Design for graceful shutdown of all tenants

### **4. Cloud & Infrastructure Integration**

**The Benefit:** First-class cloud SDKs, excellent HTTP libraries, great webhook/API support

**How to maximize:**
```go
// Position for future cloud features:
- aws-sdk-go-v2 for AWS Certificate Manager
- Prometheus client for metrics
- Standard interfaces for cloud providers
```

**Tactical moves:**
- ✅ Design APIs to be cloud-agnostic from start
- ✅ Use interfaces for external services (easy testing/mocking)
- ✅ Build metrics collection early (Prometheus compatible)

### **5. API-First Architecture**

**The Benefit:** `net/http` is mature, JSON handling trivial, great middleware ecosystem

**How to maximize:**
```go
// Design for API evolution:
type APIv1 struct{}  // Current
type APIv2 struct{}  // Future

// Structured responses from day one:
type Response struct {
    Data   interface{} `json:"data"`
    Error  *string     `json:"error,omitempty"`
    Meta   *Meta       `json:"meta,omitempty"`
}
```

**Tactical moves:**
- ✅ Version your APIs from the start (`/api/v1/...`)
- ✅ Use structured error responses
- ✅ Design for OpenAPI documentation

### **6. Testing & Reliability**

**The Benefit:** Built-in testing framework, great table-driven patterns, easy integration testing

**How to maximize:**
```go
// Leverage Go's testing culture:
func TestTenantDeployment(t *testing.T) {
    tests := []struct {
        name string
        req  DeployRequest
        want Result
    }{
        // Table-driven tests
    }
    // Parallel execution, benchmarks, etc.
}
```

**Tactical moves:**
- ✅ Write tests that spawn real processes (Go makes this easy)
- ✅ Use `httptest` for API testing
- ✅ Build testing utilities for common operations

### **7. Observability**

**The Benefit:** `log/slog` for structured logging, `net/http/pprof` for profiling

**How to maximize:**
```go
// Built-in observability:
import (
    "log/slog"
    _ "net/http/pprof"
)

// Structured logs, health endpoints, metrics from day one
```

---

## 🏗️ **Implementation Strategy**

### **Current Server Functionality to Port:**
```
✅ HTTP API (4 endpoints: deploy, list, delete, health)
✅ In-memory tenant registry (HashMap → map[string]*Tenant)
✅ Process spawning (Command::new() → exec.Command)
✅ Basic validation & error handling
✅ Upstream URL and app config support
```

### **Near Future Extensions (Sprints 3-4):**
- ✅ **Dashboard backend** (serving the web UI APIs)
- ✅ **TLS certificate management** 
- ✅ **Rate limiting configuration**
- ✅ **Log aggregation** (collecting from shims)
- ✅ **Tenant configuration storage**

### **Alpha Vision Features:**
- ✅ **Multi-instance orchestrator** (manage many Ghost instances)
- ✅ **Configuration hub** (for all "enterprise" features)
- ✅ **Monitoring dashboard backend** (sparklines, telemetry)
- ✅ **AMI packaging coordinator** (deployment automation)

---

## 📋 **Migration Plan**

### **Phase 1: Core API Port**
1. **Set up Go module** with proper structure
2. **Port basic HTTP endpoints** (deploy, list, delete, health)
3. **Port process management** logic
4. **Maintain API compatibility** with existing `naaas-ctl`
5. **Add comprehensive testing** (easier in Go)

### **Phase 2: Enhanced Features**
1. **Add structured logging** with `log/slog`
2. **Implement metrics collection** (Prometheus)
3. **Add configuration management**
4. **Build process supervision** and health checking

### **Phase 3: Future Integrations**
1. **Dashboard API endpoints**
2. **Cloud provider interfaces**
3. **Certificate management**
4. **Multi-region support**

---

## 🎯 **Strategic Positioning**

**Design the Go server to be:**
- ✅ **The single source of truth** for tenant state
- ✅ **The integration hub** for all external services  
- ✅ **The control plane** that makes complex operations simple
- ✅ **API-first** so other tools can build on it

**This positions perfectly for:**
- Dashboard integrations
- Cloud provider integrations  
- Monitoring and alerting
- Multi-region deployments
- Kubernetes integration (when ready)

---

## 🧠 **Key Insight**

The server is fundamentally about **coordination, not computation**. It's switching control signals and managing process lifecycle - exactly what Go excels at. The "switching gates" intuition was architecturally correct, just at the orchestration level rather than packet level.

**Confidence Level:** 95% - This aligns Go's strengths perfectly with the server's actual responsibilities as a unikernel fleet manager.