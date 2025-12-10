package main

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"os"
	"os/exec"
	"strconv"
	"strings"
	"sync"

	"github.com/google/uuid"
)

// TenantStore manages all deployed tenants
type TenantStore struct {
	mu      sync.RWMutex
	tenants map[string]*Tenant
}

// NewTenantStore creates a new tenant store
func NewTenantStore() *TenantStore {
	return &TenantStore{
		tenants: make(map[string]*Tenant),
	}
}

// Add adds a tenant to the store
func (ts *TenantStore) Add(tenant *Tenant) {
	ts.mu.Lock()
	defer ts.mu.Unlock()
	ts.tenants[tenant.ID] = tenant
}

// Get retrieves a tenant by ID
func (ts *TenantStore) Get(id string) (*Tenant, bool) {
	ts.mu.RLock()
	defer ts.mu.RUnlock()
	tenant, exists := ts.tenants[id]
	return tenant, exists
}

// List returns all tenants
func (ts *TenantStore) List() []*Tenant {
	ts.mu.RLock()
	defer ts.mu.RUnlock()
	
	tenants := make([]*Tenant, 0, len(ts.tenants))
	for _, tenant := range ts.tenants {
		tenants = append(tenants, tenant)
	}
	return tenants
}

// Delete removes a tenant from the store
func (ts *TenantStore) Delete(id string) (*Tenant, bool) {
	ts.mu.Lock()
	defer ts.mu.Unlock()
	
	tenant, exists := ts.tenants[id]
	if exists {
		delete(ts.tenants, id)
	}
	return tenant, exists
}

// Server represents the NAAAS server
type Server struct {
	store *TenantStore
}

// NewServer creates a new NAAAS server
func NewServer() *Server {
	return &Server{
		store: NewTenantStore(),
	}
}

// deployHandler handles POST /deploy
func (s *Server) deployHandler(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var request DeployRequest
	if err := json.NewDecoder(r.Body).Decode(&request); err != nil {
		log.Printf("Failed to decode deploy request: %v", err)
		http.Error(w, fmt.Sprintf(`{"error": "Invalid JSON: %s"}`, err.Error()), http.StatusBadRequest)
		return
	}

	// Validate the request
	if err := request.Validate(); err != nil {
		log.Printf("Deploy request validation failed: %v", err)
		http.Error(w, fmt.Sprintf(`{"error": "%s"}`, err.Error()), http.StatusBadRequest)
		return
	}

	// Generate unique tenant ID and assign port
	tenantID := uuid.New().String()
	port := request.GetPort()

	log.Printf("Deploying tenant: %s on port %d", request.Name, port)

	// Launch the process
	processID, err := s.launchProcess(&request, port)
	if err != nil {
		log.Printf("Failed to spawn process: %v", err)
		http.Error(w, fmt.Sprintf(`{"error": "Failed to start unikernel: %s"}`, err.Error()), http.StatusInternalServerError)
		return
	}

	// Create and store the tenant
	tenant := NewTenant(tenantID, request.Name, port, &processID, request.UnikernelPath)
	s.store.Add(tenant)

	log.Printf("Tenant %s deployed successfully with process ID %d", tenantID, processID)

	// Return the tenant data
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusCreated)
	json.NewEncoder(w).Encode(tenant)
}

// listHandler handles GET /tenants
func (s *Server) listHandler(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	tenants := s.store.List()
	
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(tenants)
}

// deleteHandler handles DELETE /tenants/{id}
func (s *Server) deleteHandler(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodDelete {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	// Extract tenant ID from path
	path := strings.TrimPrefix(r.URL.Path, "/tenants/")
	if path == "" {
		http.Error(w, `{"error": "Tenant ID required"}`, http.StatusBadRequest)
		return
	}
	tenantID := path

	// Remove tenant from store
	tenant, exists := s.store.Delete(tenantID)
	if !exists {
		http.Error(w, `{"error": "Tenant not found"}`, http.StatusNotFound)
		return
	}

	log.Printf("Stopping tenant: %s", tenantID)

	// Attempt to kill the process
	if tenant.ProcessID != nil {
		if err := s.terminateProcess(*tenant.ProcessID); err != nil {
			log.Printf("Failed to terminate process %d: %v", *tenant.ProcessID, err)
		}
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	fmt.Fprint(w, `{"message": "Tenant deleted successfully"}`)
}

// healthHandler handles GET /health
func (s *Server) healthHandler(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	fmt.Fprint(w, `{"status": "healthy", "service": "naaas-server"}`)
}

// launchProcess spawns a unikernel process
func (s *Server) launchProcess(request *DeployRequest, port uint16) (uint32, error) {
	args := []string{"--port", strconv.Itoa(int(port))}

	// Add upstream URL if provided
	if request.UpstreamURL != nil {
		args = append(args, "--upstream", *request.UpstreamURL)
	}

	// Add app config if provided
	if request.AppConfig != nil {
		args = append(args, "--config", *request.AppConfig)
	}

	// Spawn the process
	cmd := exec.Command(request.UnikernelPath, args...)
	cmd.Stdout = nil // Discard stdout for now
	cmd.Stderr = nil // Discard stderr for now
	
	if err := cmd.Start(); err != nil {
		return 0, fmt.Errorf("failed to start process: %w", err)
	}

	processID := uint32(cmd.Process.Pid)

	// Detach the process so it runs independently
	go func() {
		cmd.Wait() // Clean up when process exits
	}()

	return processID, nil
}

// terminateProcess kills a process by PID
func (s *Server) terminateProcess(pid uint32) error {
	process, err := os.FindProcess(int(pid))
	if err != nil {
		return fmt.Errorf("failed to find process: %w", err)
	}
	
	return process.Kill()
}

func main() {
	server := NewServer()

	// Set up routes
	http.HandleFunc("/deploy", server.deployHandler)
	http.HandleFunc("/tenants", server.listHandler)
	http.HandleFunc("/tenants/", server.deleteHandler)
	http.HandleFunc("/health", server.healthHandler)

	addr := ":8080"
	
	log.Printf("NAAAS Server starting on http://0.0.0.0%s", addr)
	log.Println("Endpoints:")
	log.Println("  POST /deploy      - Deploy a new tenant")
	log.Println("  GET  /tenants     - List all tenants")
	log.Println("  DELETE /tenants/{id} - Delete a tenant")
	log.Println("  GET  /health      - Health check")

	if err := http.ListenAndServe(addr, nil); err != nil {
		log.Fatalf("Server error: %v", err)
	}
}