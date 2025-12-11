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

**🆕 CLI Migration Lessons:**
- ✅ **Use established patterns** (Cobra for CLI, similar patterns exist for servers)
- ✅ **Organize code by domain** (cmd/ pattern scales well)
- ✅ **Export what needs testing** but keep internals private
- ✅ **Go's tooling shines** - go mod, go test, go build all just work

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

### **4. Future Adaptability & Integration**

**The Reality:** We don't know what platforms we'll need (AWS? GCP? Azure? K8s? Something new?)

**The Strategy:** Design for **pluggable adaptation** rather than specific platforms

**How to maximize adaptability:**
```go
// Design EVERYTHING as interfaces, implement AWS first:
type CertificateHandler interface {
    LoadCertificate(domain string, certFiles CertificateFiles) (*Certificate, error)
    ValidateCertificate(cert *Certificate) error
}
// Implementations: &LocalCertHandler{} (BYOD: client provides cert files)

type ProcessLauncher interface {
    LaunchProcess(config ProcessConfig) (*Process, error)
    KillProcess(id string) error
    ListProcesses() ([]*Process, error)
}
// Implementations: &EC2ProcessLauncher{}, &LocalProcessLauncher{}

type MetricsCollector interface {
    RecordMetric(name string, value float64, tags map[string]string)
    GetMetrics() (MetricsSnapshot, error)
}
// Implementations: &CloudWatchMetrics{}, &LocalMetrics{}

type ConfigStore interface {
    Get(key string) (string, error)
    Set(key, value string) error
    Delete(key string) error
}
// Implementations: &SSMConfigStore{}, &FileConfigStore{}
```

**Tactical moves for balanced adaptability:**
- ✅ **Ship simple first** - direct implementations, no premature abstraction
- ✅ **Extract interfaces when needed** - let real requirements drive design
- ✅ **Configuration-driven** - runtime selection only when you have multiple options
- ✅ **Standard protocols** - HTTP/JSON work everywhere, vendor APIs are last resort
- ✅ **Gradual abstraction** - start concrete, abstract when patterns emerge

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

**🆕 CLI Migration Lessons:**
- ✅ **Focus on core logic tests** over complex integration tests
- ✅ **Export APIs for testing** (commands, helper functions)
- ✅ **Test data structures and serialization** thoroughly
- ✅ **Use helper functions** for setup/teardown in tests
- ✅ **Fewer, focused tests** > many fragile tests

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
- ✅ **TLS certificate handling** (BYOD: accept client-provided certificates) 
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

### **Phase 1: Direct Port (No Interfaces Yet)**
1. **Set up Go module** with proper structure
2. **Port basic HTTP endpoints** (deploy, list, delete, health)
3. **Port process management** logic using `os/exec` directly
4. **Maintain API compatibility** with existing `naaas-ctl`
5. **Add comprehensive testing** (easier in Go)

**🚨 Complexity Warning:** Start simple, add interfaces later when we **need** them

**🆕 CLI Migration Insights:**
- ✅ **Start with data structures** - get Tenant, DeployRequest types right first
- ✅ **Port tests alongside code** - Go's testing makes this natural
- ✅ **Use domain-based file organization** - handlers/, models/, etc.
- ✅ **Export testing helpers** - ResetState(), GetState() functions
- ✅ **Focus on JSON compatibility** - ensure seamless CLI integration
- ✅ **Ship working code first** - don't over-engineer from day one

### **Phase 2: Refactor to Interfaces (When We Need Them)**
1. **Extract interfaces** from working direct code
2. **Create local and AWS providers** from existing implementations
3. **Add provider selection mechanism**
4. **Test both providers work with same core**

**🎯 Key insight:** Let **real needs** drive interface design, not theoretical futures

### **Phase 3: AWS Alpha Deployment**
1. **AWS EC2 deployment** - Use EC2ProcessLauncher for unikernel spawning
2. **Certificate handling** - Accept client-provided certificate files for BYOD deployment  
3. **AWS CloudWatch** - Use CloudWatchMetrics for monitoring
4. **AWS SSM** - Use SSMConfigStore for tenant configuration
5. **Create AWS AMI** with naaas-server + dashboard pre-configured

### **Phase 4: Provider Ecosystem Growth**
1. **Additional platforms** (GCP, Azure, bare metal)
2. **Additional orchestrators** (K8s, Docker Swarm, Nomad)
3. **Additional services** (different CAs, metrics systems, config stores)

**AWS-First Advantages:**
- ✅ **Immediate deployment target** - Real production environment
- ✅ **Proven ecosystem** - AWS services are battle-tested
- ✅ **AMI distribution** - Easy customer deployment
- ✅ **Future flexibility** - Interface design allows expansion beyond AWS

---

## 🎯 **Strategic Positioning**

**Design the Go server to be:**
- ✅ **The single source of truth** for tenant state
- ✅ **The integration hub** for all external services  
- ✅ **The control plane** that makes complex operations simple
- ✅ **API-first** so other tools can build on it

**This positions perfectly for:**
- **Unknown cloud providers** (via pluggable interfaces)
- **Unknown orchestrators** (Docker, K8s, Nomad, whatever comes next)
- **Unknown monitoring systems** (Prometheus, DataDog, New Relic, etc.)
- **Unknown certificate sources** (client-provided files, cloud CAs, internal enterprise CAs)
- **Unknown storage backends** (local files, databases, cloud storage)

**The key insight:** Design for **categories of integration**, not specific vendors.

---

## 🧠 **Key Insight**

The server is fundamentally about **coordination, not computation**. It's switching control signals and managing process lifecycle - exactly what Go excels at. The "switching gates" intuition was architecturally correct, just at the orchestration level rather than packet level.

**Confidence Level:** 99% - This aligns Go's strengths perfectly with the server's actual responsibilities as a unikernel fleet manager.

## 🎓 **CLI Migration Validation**

The CLI rewrite **proved our thesis correct:**

**✅ Development Speed:** CLI rewrite took ~2 hours vs days in Rust  
**✅ Code Quality:** 300 lines Go vs 800+ lines Rust for same functionality  
**✅ Testing:** 7 focused tests vs 15+ complex Rust tests  
**✅ Maintainability:** Cmd pattern makes code navigation trivial  
**✅ Deployment:** Single 4MB binary vs Rust's complex build setup  
**✅ API Compatibility:** 100% compatibility maintained seamlessly  

**Key Success Metrics:**
- **Readability:** "ridiculously readable" - immediate team feedback
- **Testing:** Core logic tests run faster and more reliably  
- **Structure:** cmd/ pattern scales beautifully for larger codebases
- **Tooling:** go build, go test, go mod "just work"

The CLI migration **de-risks the server rewrite** and validates our technical choices.

## 🔌 **Adaptability in Action**

**Example: AWS-first with adaptability built in**

Development: "Local testing"
```go
server := NewServer(&LocalProcessLauncher{}, &FileConfigStore{})
```

Alpha: "Deploy to AWS"
```go
server := NewServer(&EC2ProcessLauncher{}, &SSMConfigStore{})
```

Customer request: "We need on-premises deployment"
```go
server := NewServer(&LocalProcessLauncher{}, &FileConfigStore{})
```

Enterprise: "We use Kubernetes"
```go
server := NewServer(&K8sProcessLauncher{}, &EtcdConfigStore{})
```

**Zero core server code changes** - providers handle platform specifics.

**The secret:** Start **concrete** (os/exec, file storage), then extract **interfaces** when you actually need multiple implementations.

## ⚖️ **Complexity vs Flexibility Tradeoff**

**The tension:** Interfaces enable flexibility but add complexity

**The balanced approach:**
1. **Phase 1:** Ship direct implementation (like current Rust server, but in Go)
2. **Phase 2:** Extract interfaces when we need AWS integration  
3. **Phase 3:** Add more providers only when customers demand them

**Benefits of this approach:**
- ✅ **Fast initial delivery** - No over-engineering delays
- ✅ **Real-world driven design** - Interfaces reflect actual needs
- ✅ **Easier debugging** - Concrete code is simpler to understand
- ✅ **Future flexibility** - Can still add providers later

**Risk mitigation:** Go's interfaces are lightweight - easy to add later without breaking changes.