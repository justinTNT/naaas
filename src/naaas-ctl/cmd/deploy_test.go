package cmd

import (
	"testing"
)

func TestGetDeployParams(t *testing.T) {
	// Setup: Reset parameters
	ResetDeployParams()
	
	// Test: Get empty parameters
	name, unikernel, port, upstream, config := GetDeployParams()
	if name != "" || unikernel != "" || port != "" || upstream != "" || config != "" {
		t.Error("Reset parameters should be empty")
	}
}

func TestResetDeployParams(t *testing.T) {
	// Setup: Set some values
	name = "test"
	unikernel = "test"
	port = "test" 
	upstream = "test"
	config = "test"
	
	// Action: Reset
	ResetDeployParams()
	
	// Assert: All should be empty
	if name != "" || unikernel != "" || port != "" || upstream != "" || config != "" {
		t.Error("Parameters should be empty after reset")
	}
}

func TestNewDeployRequest(t *testing.T) {
	request := NewDeployRequest("test-tenant", "/path/to/unikernel")
	
	if request.Name != "test-tenant" {
		t.Errorf("Expected name 'test-tenant', got '%s'", request.Name)
	}
	if request.UnikernelPath != "/path/to/unikernel" {
		t.Errorf("Expected path '/path/to/unikernel', got '%s'", request.UnikernelPath)
	}
	if request.Port != nil {
		t.Error("Port should be nil")
	}
}

func TestDeployRequestBuilder(t *testing.T) {
	var port uint16 = 4000
	request := NewDeployRequest("test", "/test").
		WithPort(port).
		WithUpstreamURL("http://upstream").
		WithAppConfig(`{"name": "Test"}`)
	
	if request.Name != "test" || request.UnikernelPath != "/test" {
		t.Error("Basic fields incorrect")
	}
	if request.Port == nil || *request.Port != 4000 {
		t.Error("Port not set correctly")
	}
	if request.UpstreamURL == nil || *request.UpstreamURL != "http://upstream" {
		t.Error("Upstream URL not set correctly")
	}
	if request.AppConfig == nil || *request.AppConfig != `{"name": "Test"}` {
		t.Error("App config not set correctly")
	}
}