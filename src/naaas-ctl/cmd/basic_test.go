package cmd

import (
	"encoding/json"
	"testing"
)

func TestTenantSerialization(t *testing.T) {
	var processID uint32 = 12345
	tenant := &Tenant{
		ID:            "test-id",
		Name:          "test-tenant", 
		Status:        "running",
		Port:          3001,
		ProcessID:     &processID,
		UnikernelPath: "/path/to/unikernel",
	}

	jsonBytes, err := json.Marshal(tenant)
	if err != nil {
		t.Fatalf("Failed to marshal: %v", err)
	}

	var parsed Tenant
	err = json.Unmarshal(jsonBytes, &parsed)
	if err != nil {
		t.Fatalf("Failed to unmarshal: %v", err)
	}

	if parsed.ID != tenant.ID || parsed.Name != tenant.Name {
		t.Error("Serialization roundtrip failed")
	}
}

func TestGetServerURL(t *testing.T) {
	// This should return the current server URL
	url := GetServerURL()
	if url == "" {
		t.Error("Server URL should not be empty")
	}
}

func TestTenantStringMethod(t *testing.T) {
	var processID uint32 = 12345
	tenant := &Tenant{
		ID:            "test-id",
		Name:          "test-tenant",
		Status:        "running", 
		Port:          3001,
		ProcessID:     &processID,
		UnikernelPath: "/path/to/unikernel",
	}

	str := tenant.String()
	if str == "" {
		t.Error("String method should not return empty")
	}

	// Should be valid JSON
	var parsed Tenant
	err := json.Unmarshal([]byte(str), &parsed)
	if err != nil {
		t.Errorf("String method should return valid JSON: %v", err)
	}
}