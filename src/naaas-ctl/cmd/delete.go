package cmd

import (
	"fmt"

	"github.com/go-resty/resty/v2"
	"github.com/spf13/cobra"
)

// DeleteCmd represents the delete command (exported for testing)
var DeleteCmd = &cobra.Command{
	Use:   "delete [tenant-id]",
	Short: "Delete a tenant",
	Long:  `Delete a specific tenant by its ID, stopping the associated unikernel process.`,
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		tenantID := args[0]

		// Create HTTP client
		client := resty.New()
		url := fmt.Sprintf("%s/tenants/%s", GetServerURL(), tenantID)

		// Send delete request
		resp, err := client.R().
			SetHeader("Accept", "application/json").
			Delete(url)

		if err != nil {
			return fmt.Errorf("failed to send delete request: %w", err)
		}

		if resp.IsSuccess() {
			fmt.Printf("Tenant '%s' deleted successfully\n", tenantID)
		} else if resp.StatusCode() == 404 {
			return fmt.Errorf("tenant not found: %s", tenantID)
		} else {
			return fmt.Errorf("delete failed: %s", resp.String())
		}

		return nil
	},
}

func init() {
	RootCmd.AddCommand(DeleteCmd)
}