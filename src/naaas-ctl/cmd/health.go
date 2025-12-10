package cmd

import (
	"fmt"

	"github.com/go-resty/resty/v2"
	"github.com/spf13/cobra"
)

// HealthCmd represents the health command (exported for testing)
var HealthCmd = &cobra.Command{
	Use:   "health",
	Short: "Check server health",
	Long:  `Check the health status of the NAAAS server to ensure it's running and responding properly.`,
	RunE: func(cmd *cobra.Command, args []string) error {
		// Create HTTP client
		client := resty.New()
		url := fmt.Sprintf("%s/health", GetServerURL())

		// Send health check request
		resp, err := client.R().
			SetHeader("Accept", "application/json").
			Get(url)

		if err != nil {
			return fmt.Errorf("failed to connect to server: %w", err)
		}

		if resp.IsSuccess() {
			fmt.Printf("Server is healthy (HTTP %d)\n", resp.StatusCode())
			fmt.Printf("Server URL: %s\n", GetServerURL())
		} else {
			return fmt.Errorf("health check failed: HTTP %d - %s", resp.StatusCode(), resp.String())
		}

		return nil
	},
}

func init() {
	RootCmd.AddCommand(HealthCmd)
}