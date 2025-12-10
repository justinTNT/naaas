package cmd

import (
	"encoding/json"
	"fmt"

	"github.com/go-resty/resty/v2"
	"github.com/spf13/cobra"
)

// ListCmd represents the list command (exported for testing)
var ListCmd = &cobra.Command{
	Use:   "list",
	Short: "List all deployed tenants",
	Long:  `List all deployed tenants with their current status and configuration.`,
	RunE: func(cmd *cobra.Command, args []string) error {
		// Create HTTP client
		client := resty.New()
		url := fmt.Sprintf("%s/tenants", GetServerURL())

		// Send list request
		resp, err := client.R().
			SetHeader("Accept", "application/json").
			Get(url)

		if err != nil {
			return fmt.Errorf("failed to send list request: %w", err)
		}

		if resp.IsSuccess() {
			var tenants []Tenant
			if err := json.Unmarshal(resp.Body(), &tenants); err != nil {
				return fmt.Errorf("failed to parse response: %w", err)
			}

			if len(tenants) == 0 {
				fmt.Println("No tenants deployed")
			} else {
				fmt.Printf("Found %d tenant(s):\n", len(tenants))
				for i, tenant := range tenants {
					fmt.Printf("\n--- Tenant %d ---\n%s\n", i+1, tenant.String())
				}
			}
		} else {
			return fmt.Errorf("list failed: %s", resp.String())
		}

		return nil
	},
}

func init() {
	RootCmd.AddCommand(ListCmd)
}