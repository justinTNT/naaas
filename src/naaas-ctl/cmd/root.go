package cmd

import (
	"encoding/json"
	"fmt"
	"os"

	"github.com/spf13/cobra"
)

var (
	serverURL string
)

// RootCmd represents the base command when called without any subcommands (exported for testing)
var RootCmd = &cobra.Command{
	Use:   "naaas-ctl",
	Short: "NAAAS Control CLI - Manage tenant unikernels",
	Long:  `NAAAS Control CLI - Manage tenant unikernels`,
}

// Execute adds all child commands to the root command and sets flags appropriately.
func Execute() {
	if err := RootCmd.Execute(); err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}
}

func init() {
	// Global flags
	RootCmd.PersistentFlags().StringVarP(&serverURL, "server", "s", "http://localhost:8080", "NAAAS server URL")
}

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

// NewDeployRequest creates a new deploy request with required fields
func NewDeployRequest(name, unikernelPath string) *DeployRequest {
	return &DeployRequest{
		Name:          name,
		UnikernelPath: unikernelPath,
	}
}

// WithPort sets the port for the deploy request
func (dr *DeployRequest) WithPort(port uint16) *DeployRequest {
	dr.Port = &port
	return dr
}

// WithUpstreamURL sets the upstream URL for the deploy request
func (dr *DeployRequest) WithUpstreamURL(upstream string) *DeployRequest {
	dr.UpstreamURL = &upstream
	return dr
}

// WithAppConfig sets the app config for the deploy request
func (dr *DeployRequest) WithAppConfig(config string) *DeployRequest {
	dr.AppConfig = &config
	return dr
}

// String returns a pretty-printed JSON representation
func (t *Tenant) String() string {
	data, _ := json.MarshalIndent(t, "", "  ")
	return string(data)
}

// GetServerURL returns the configured server URL
func GetServerURL() string {
	return serverURL
}