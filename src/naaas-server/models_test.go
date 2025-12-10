package main

import (
	"encoding/json"
	"testing"
)

func TestTenantCreation(t *testing.T) {
	// Setup: Create a tenant with all required data
	var processID uint32 = 1234
	tenant := NewTenant(
		"test-id",
		"test-tenant",
		3001,
		&processID,
		"/path/to/unikernel",
	)

	// Assert: All fields are set correctly
	if tenant.ID != "test-id" {
		t.Errorf("Expected ID 'test-id', got '%s'", tenant.ID)
	}
	if tenant.Name != "test-tenant" {
		t.Errorf("Expected name 'test-tenant', got '%s'", tenant.Name)
	}
	if tenant.Port != 3001 {
		t.Errorf("Expected port 3001, got %d", tenant.Port)
	}
	if tenant.ProcessID == nil || *tenant.ProcessID != 1234 {
		t.Errorf("Expected process ID 1234, got %v", tenant.ProcessID)
	}
	if tenant.UnikernelPath != "/path/to/unikernel" {
		t.Errorf("Expected path '/path/to/unikernel', got '%s'", tenant.UnikernelPath)
	}
	if tenant.Status != "running" {
		t.Errorf("Expected status 'running', got '%s'", tenant.Status)
	}
}

func TestTenantIsActive(t *testing.T) {
	// Test active tenant
	var processID uint32 = 1234
	activeTenant := NewTenant("test-id", "test", 3001, &processID, "/path")
	if !activeTenant.IsActive() {
		t.Error("Tenant with process ID and running status should be active")
	}

	// Test inactive tenant (no process ID)
	inactiveTenant := NewTenant("test-id", "test", 3001, nil, "/path")
	if inactiveTenant.IsActive() {
		t.Error("Tenant without process ID should not be active")
	}
}

func TestTenantSerialization(t *testing.T) {
	// Setup: Create a tenant
	var processID uint32 = 1234
	originalTenant := NewTenant(
		"test-id",
		"test-tenant",
		3001,
		&processID,
		"/path/to/unikernel",
	)

	// Action: Serialize to JSON and back
	jsonBytes, err := json.Marshal(originalTenant)
	if err != nil {
		t.Fatalf("Failed to marshal tenant: %v", err)
	}

	var deserializedTenant Tenant
	err = json.Unmarshal(jsonBytes, &deserializedTenant)
	if err != nil {
		t.Fatalf("Failed to unmarshal tenant: %v", err)
	}

	// Assert: Serialization roundtrip preserves all data
	if deserializedTenant.ID != originalTenant.ID {
		t.Errorf("ID mismatch after serialization")
	}
	if deserializedTenant.Name != originalTenant.Name {
		t.Errorf("Name mismatch after serialization")
	}
	if deserializedTenant.Port != originalTenant.Port {
		t.Errorf("Port mismatch after serialization")
	}
}

func TestDeployRequestValidation(t *testing.T) {
	// Test valid request
	validRequest := &DeployRequest{
		Name:          "valid-tenant",
		UnikernelPath: "/valid/path",
		Port:          nil,
		UpstreamURL:   nil,
		AppConfig:     nil,
	}
	if err := validRequest.Validate(); err != nil {
		t.Errorf("Valid request should pass validation, got error: %v", err)
	}

	// Test empty name
	emptyNameRequest := &DeployRequest{
		Name:          "   ",
		UnikernelPath: "/valid/path",
	}
	if err := emptyNameRequest.Validate(); err == nil {
		t.Error("Empty name should fail validation")
	}

	// Test empty path
	emptyPathRequest := &DeployRequest{
		Name:          "valid-name",
		UnikernelPath: "",
	}
	if err := emptyPathRequest.Validate(); err == nil {
		t.Error("Empty unikernel path should fail validation")
	}

	// Test invalid port
	var invalidPort uint16 = 80
	invalidPortRequest := &DeployRequest{
		Name:          "valid-name",
		UnikernelPath: "/valid/path",
		Port:          &invalidPort,
	}
	if err := invalidPortRequest.Validate(); err == nil {
		t.Error("Port below 1024 should fail validation")
	}
}

func TestDeployRequestGetPort(t *testing.T) {
	// Test with specified port
	var specifiedPort uint16 = 4000
	requestWithPort := &DeployRequest{
		Name:          "test",
		UnikernelPath: "/path",
		Port:          &specifiedPort,
	}
	if requestWithPort.GetPort() != 4000 {
		t.Errorf("Expected port 4000, got %d", requestWithPort.GetPort())
	}

	// Test with default port
	requestWithoutPort := &DeployRequest{
		Name:          "test",
		UnikernelPath: "/path",
		Port:          nil,
	}
	if requestWithoutPort.GetPort() != 3001 {
		t.Errorf("Expected default port 3001, got %d", requestWithoutPort.GetPort())
	}
}

func TestDeployRequestSerialization(t *testing.T) {
	// Test serialization with all fields
	var port uint16 = 5000
	upstream := "http://upstream:2368"
	config := `{"name":"Test App"}`
	
	originalRequest := &DeployRequest{
		Name:          "test-app",
		UnikernelPath: "/test/path",
		Port:          &port,
		UpstreamURL:   &upstream,
		AppConfig:     &config,
	}

	// Serialize and deserialize
	jsonBytes, err := json.Marshal(originalRequest)
	if err != nil {
		t.Fatalf("Failed to marshal request: %v", err)
	}

	var deserializedRequest DeployRequest
	err = json.Unmarshal(jsonBytes, &deserializedRequest)
	if err != nil {
		t.Fatalf("Failed to unmarshal request: %v", err)
	}

	// Assert all fields match
	if deserializedRequest.Name != originalRequest.Name {
		t.Error("Name mismatch after serialization")
	}
	if deserializedRequest.UnikernelPath != originalRequest.UnikernelPath {
		t.Error("UnikernelPath mismatch after serialization")
	}
	if deserializedRequest.Port == nil || *deserializedRequest.Port != *originalRequest.Port {
		t.Error("Port mismatch after serialization")
	}
	if deserializedRequest.UpstreamURL == nil || *deserializedRequest.UpstreamURL != *originalRequest.UpstreamURL {
		t.Error("UpstreamURL mismatch after serialization")
	}
	if deserializedRequest.AppConfig == nil || *deserializedRequest.AppConfig != *originalRequest.AppConfig {
		t.Error("AppConfig mismatch after serialization")
	}
}