package cmd

import (
	"encoding/json"
	"fmt"
	"strconv"

	"github.com/go-resty/resty/v2"
	"github.com/spf13/cobra"
)

var (
	name          string
	unikernel     string
	port          string
	upstream      string
	config        string
)

// DeployCmd represents the deploy command (exported for testing)
var DeployCmd = &cobra.Command{
	Use:   "deploy",
	Short: "Deploy a new tenant unikernel",
	Long:  `Deploy a new tenant unikernel with the specified configuration.`,
	RunE: func(cmd *cobra.Command, args []string) error {
		// Create deploy request
		request := NewDeployRequest(name, unikernel)

		// Add optional port
		if port != "" {
			if portNum, err := strconv.ParseUint(port, 10, 16); err != nil {
				return fmt.Errorf("invalid port number: %s", port)
			} else {
				request.WithPort(uint16(portNum))
			}
		}

		// Add optional upstream URL
		if upstream != "" {
			request.WithUpstreamURL(upstream)
		}

		// Add optional app config
		if config != "" {
			request.WithAppConfig(config)
		}

		// Create HTTP client
		client := resty.New()
		url := fmt.Sprintf("%s/deploy", GetServerURL())

		// Send deploy request
		resp, err := client.R().
			SetHeader("Content-Type", "application/json").
			SetBody(request).
			Post(url)

		if err != nil {
			return fmt.Errorf("failed to send deploy request: %w", err)
		}

		if resp.IsSuccess() {
			var tenant Tenant
			if err := json.Unmarshal(resp.Body(), &tenant); err != nil {
				return fmt.Errorf("failed to parse response: %w", err)
			}
			fmt.Printf("Tenant deployed successfully:\n%s\n", tenant.String())
		} else {
			return fmt.Errorf("deploy failed: %s", resp.String())
		}

		return nil
	},
}

func init() {
	RootCmd.AddCommand(DeployCmd)

	// Required flags
	DeployCmd.Flags().StringVarP(&name, "name", "n", "", "Tenant name (required)")
	DeployCmd.Flags().StringVarP(&unikernel, "unikernel", "u", "", "Path to unikernel binary (required)")
	
	// Optional flags
	DeployCmd.Flags().StringVarP(&port, "port", "p", "", "Port for the unikernel")
	DeployCmd.Flags().StringVar(&upstream, "upstream", "", "Upstream URL to proxy to (for proxy shims)")
	DeployCmd.Flags().StringVar(&config, "config", "", "App configuration JSON")

	// Mark required flags
	DeployCmd.MarkFlagRequired("name")
	DeployCmd.MarkFlagRequired("unikernel")
}

// Test helpers (exported for testing)
func GetDeployParams() (string, string, string, string, string) {
	return name, unikernel, port, upstream, config
}

func ResetDeployParams() {
	name = ""
	unikernel = ""
	port = ""
	upstream = ""
	config = ""
}