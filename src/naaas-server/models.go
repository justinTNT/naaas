package main

import (
	"fmt"
	"strings"
)

// Tenant represents a deployed unikernel tenant
type Tenant struct {
	ID            string  `json:"id"`
	Name          string  `json:"name"`
	Status        string  `json:"status"`
	Port          uint16  `json:"port"`
	ProcessID     *uint32 `json:"process_id"`
	UnikernelPath string  `json:"unikernel_path"`
}

// DeployRequest represents a request to deploy a new tenant
type DeployRequest struct {
	Name          string  `json:"name"`
	UnikernelPath string  `json:"unikernel_path"`
	Port          *uint16 `json:"port,omitempty"`
	UpstreamURL   *string `json:"upstream_url,omitempty"`
	AppConfig     *string `json:"app_config,omitempty"`
}

// NewTenant creates a new tenant with the given parameters
func NewTenant(id, name string, port uint16, processID *uint32, unikernelPath string) *Tenant {
	return &Tenant{
		ID:            id,
		Name:          name,
		Status:        "running",
		Port:          port,
		ProcessID:     processID,
		UnikernelPath: unikernelPath,
	}
}

// IsActive checks if this tenant is considered active (has a process ID and running status)
func (t *Tenant) IsActive() bool {
	return t.ProcessID != nil && t.Status == "running"
}

// Validate checks that the deploy request has all required fields
func (dr *DeployRequest) Validate() error {
	if strings.TrimSpace(dr.Name) == "" {
		return fmt.Errorf("tenant name cannot be empty")
	}

	if strings.TrimSpace(dr.UnikernelPath) == "" {
		return fmt.Errorf("unikernel path cannot be empty")
	}

	// Check port range if specified
	if dr.Port != nil && *dr.Port < 1024 {
		return fmt.Errorf("port must be between 1024 and 65535")
	}

	return nil
}

// GetPort returns the port to use, either from request or default
func (dr *DeployRequest) GetPort() uint16 {
	if dr.Port != nil {
		return *dr.Port
	}
	return 3001
}