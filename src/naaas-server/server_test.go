package main

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestHealthHandler(t *testing.T) {
	server := NewServer()
	
	req, err := http.NewRequest("GET", "/health", nil)
	if err != nil {
		t.Fatal(err)
	}
	
	rr := httptest.NewRecorder()
	handler := http.HandlerFunc(server.healthHandler)
	handler.ServeHTTP(rr, req)
	
	if status := rr.Code; status != http.StatusOK {
		t.Errorf("Expected status code %d, got %d", http.StatusOK, status)
	}
	
	expected := `{"status": "healthy", "service": "naaas-server"}`
	if rr.Body.String() != expected {
		t.Errorf("Expected body %s, got %s", expected, rr.Body.String())
	}
}

func TestListHandlerEmpty(t *testing.T) {
	server := NewServer()
	
	req, err := http.NewRequest("GET", "/tenants", nil)
	if err != nil {
		t.Fatal(err)
	}
	
	rr := httptest.NewRecorder()
	handler := http.HandlerFunc(server.listHandler)
	handler.ServeHTTP(rr, req)
	
	if status := rr.Code; status != http.StatusOK {
		t.Errorf("Expected status code %d, got %d", http.StatusOK, status)
	}
	
	var tenants []Tenant
	if err := json.Unmarshal(rr.Body.Bytes(), &tenants); err != nil {
		t.Errorf("Failed to unmarshal response: %v", err)
	}
	
	if len(tenants) != 0 {
		t.Errorf("Expected empty tenant list, got %d tenants", len(tenants))
	}
}

func TestDeployHandlerValidation(t *testing.T) {
	server := NewServer()
	
	// Test invalid request (empty name)
	invalidRequest := DeployRequest{
		Name:          "",
		UnikernelPath: "/path/to/binary",
	}
	
	reqBody, _ := json.Marshal(invalidRequest)
	req, err := http.NewRequest("POST", "/deploy", bytes.NewBuffer(reqBody))
	if err != nil {
		t.Fatal(err)
	}
	req.Header.Set("Content-Type", "application/json")
	
	rr := httptest.NewRecorder()
	handler := http.HandlerFunc(server.deployHandler)
	handler.ServeHTTP(rr, req)
	
	if status := rr.Code; status != http.StatusBadRequest {
		t.Errorf("Expected status code %d, got %d", http.StatusBadRequest, status)
	}
	
	if !bytes.Contains(rr.Body.Bytes(), []byte("name cannot be empty")) {
		t.Errorf("Expected error message about empty name, got: %s", rr.Body.String())
	}
}

func TestDeleteHandlerNotFound(t *testing.T) {
	server := NewServer()
	
	req, err := http.NewRequest("DELETE", "/tenants/nonexistent-id", nil)
	if err != nil {
		t.Fatal(err)
	}
	
	rr := httptest.NewRecorder()
	handler := http.HandlerFunc(server.deleteHandler)
	handler.ServeHTTP(rr, req)
	
	if status := rr.Code; status != http.StatusNotFound {
		t.Errorf("Expected status code %d, got %d", http.StatusNotFound, status)
	}
	
	if !bytes.Contains(rr.Body.Bytes(), []byte("Tenant not found")) {
		t.Errorf("Expected 'Tenant not found' message, got: %s", rr.Body.String())
	}
}

func TestTenantStore(t *testing.T) {
	store := NewTenantStore()
	
	// Test empty store
	tenants := store.List()
	if len(tenants) != 0 {
		t.Errorf("Expected empty store, got %d tenants", len(tenants))
	}
	
	// Test adding tenant
	var processID uint32 = 1234
	tenant := NewTenant("test-id", "test-tenant", 3001, &processID, "/test/path")
	store.Add(tenant)
	
	// Test retrieving tenant
	retrieved, exists := store.Get("test-id")
	if !exists {
		t.Error("Expected tenant to exist in store")
	}
	if retrieved.ID != "test-id" {
		t.Errorf("Expected tenant ID 'test-id', got '%s'", retrieved.ID)
	}
	
	// Test listing tenants
	tenants = store.List()
	if len(tenants) != 1 {
		t.Errorf("Expected 1 tenant in store, got %d", len(tenants))
	}
	
	// Test deleting tenant
	deleted, exists := store.Delete("test-id")
	if !exists {
		t.Error("Expected tenant to exist for deletion")
	}
	if deleted.ID != "test-id" {
		t.Errorf("Expected deleted tenant ID 'test-id', got '%s'", deleted.ID)
	}
	
	// Test store is empty after deletion
	tenants = store.List()
	if len(tenants) != 0 {
		t.Errorf("Expected empty store after deletion, got %d tenants", len(tenants))
	}
}

func TestMethodNotAllowed(t *testing.T) {
	server := NewServer()
	
	// Test wrong method on health endpoint
	req, err := http.NewRequest("POST", "/health", nil)
	if err != nil {
		t.Fatal(err)
	}
	
	rr := httptest.NewRecorder()
	handler := http.HandlerFunc(server.healthHandler)
	handler.ServeHTTP(rr, req)
	
	if status := rr.Code; status != http.StatusMethodNotAllowed {
		t.Errorf("Expected status code %d, got %d", http.StatusMethodNotAllowed, status)
	}
}

func TestJSONContentType(t *testing.T) {
	server := NewServer()
	
	// Test health endpoint sets JSON content type
	req, err := http.NewRequest("GET", "/health", nil)
	if err != nil {
		t.Fatal(err)
	}
	
	rr := httptest.NewRecorder()
	handler := http.HandlerFunc(server.healthHandler)
	handler.ServeHTTP(rr, req)
	
	contentType := rr.Header().Get("Content-Type")
	expected := "application/json"
	if contentType != expected {
		t.Errorf("Expected Content-Type %s, got %s", expected, contentType)
	}
}