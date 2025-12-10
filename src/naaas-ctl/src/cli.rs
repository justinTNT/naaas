use clap::{Parser, Subcommand};

/// NAAAS Control CLI - Manage tenant unikernels
#[derive(Parser, Debug, Clone, PartialEq)]
#[command(author, version, about, long_about = None)]
#[command(name = "naaas-ctl")]
#[command(about = "NAAAS Control CLI - Manage tenant unikernels")]
pub struct Cli {
    /// NAAAS server URL
    #[arg(short, long, default_value = "http://localhost:8080")]
    pub server: String,
    
    #[command(subcommand)]
    pub command: Commands,
}

/// Available CLI commands
#[derive(Subcommand, Debug, Clone, PartialEq)]
pub enum Commands {
    /// Deploy a new tenant unikernel
    Deploy {
        /// Tenant name
        #[arg(short, long)]
        name: String,
        
        /// Path to unikernel binary
        #[arg(short, long)]
        unikernel: String,
        
        /// Port for the unikernel
        #[arg(short, long)]
        port: Option<u16>,
        
        /// Upstream URL to proxy to (for proxy shims)
        #[arg(long)]
        upstream: Option<String>,
        
        /// App configuration JSON
        #[arg(long)]
        config: Option<String>,
    },
    
    /// List all deployed tenants
    List,
    
    /// Delete a tenant
    Delete {
        /// Tenant ID
        tenant_id: String,
    },
    
    /// Check server health
    Health,
}

impl Cli {
    /// Parse CLI arguments from the command line
    pub fn parse_args() -> Self {
        Self::parse()
    }
    
    /// Parse CLI arguments from a vector of strings (useful for testing)
    pub fn parse_from_args(args: Vec<&str>) -> Result<Self, clap::Error> {
        Self::try_parse_from(args)
    }
    
    /// Get the server URL, applying any defaults
    pub fn get_server_url(&self) -> &str {
        &self.server
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parses_deploy_command_with_all_arguments() {
        // Setup: CLI arguments for deploy command with all options
        let args = vec![
            "naaas-ctl",
            "--server", "http://test-server:9000",
            "deploy",
            "--name", "test-tenant",
            "--unikernel", "/path/to/unikernel",
            "--port", "4000",
            "--upstream", "http://upstream:2368",
            "--config", "{\"name\": \"Test App\"}"
        ];

        // Action: Parse the arguments
        let cli = Cli::parse_from_args(args).unwrap();

        // Assert: All fields are parsed correctly
        assert_eq!(cli.server, "http://test-server:9000");
        
        match cli.command {
            Commands::Deploy { name, unikernel, port, upstream, config } => {
                assert_eq!(name, "test-tenant");
                assert_eq!(unikernel, "/path/to/unikernel");
                assert_eq!(port, Some(4000));
                assert_eq!(upstream, Some("http://upstream:2368".to_string()));
                assert_eq!(config, Some("{\"name\": \"Test App\"}".to_string()));
            }
            _ => panic!("Expected Deploy command"),
        }
    }

    #[test]
    fn test_cli_parses_deploy_command_with_minimal_arguments() {
        // Setup: CLI arguments for deploy command with only required fields
        let args = vec![
            "naaas-ctl",
            "deploy",
            "--name", "minimal-tenant",
            "--unikernel", "/minimal/path"
        ];

        // Action: Parse the arguments
        let cli = Cli::parse_from_args(args).unwrap();

        // Assert: Required fields are set, optional fields are None
        assert_eq!(cli.server, "http://localhost:8080"); // Default server
        
        match cli.command {
            Commands::Deploy { name, unikernel, port, upstream, config } => {
                assert_eq!(name, "minimal-tenant");
                assert_eq!(unikernel, "/minimal/path");
                assert_eq!(port, None);
                assert_eq!(upstream, None);
                assert_eq!(config, None);
            }
            _ => panic!("Expected Deploy command"),
        }
    }

    #[test]
    fn test_cli_parses_list_command() {
        // Setup: CLI arguments for list command
        let args = vec!["naaas-ctl", "list"];

        // Action: Parse the arguments
        let cli = Cli::parse_from_args(args).unwrap();

        // Assert: Command is parsed as List
        assert!(matches!(cli.command, Commands::List));
        assert_eq!(cli.server, "http://localhost:8080"); // Default server
    }

    #[test]
    fn test_cli_parses_delete_command_with_tenant_id() {
        // Setup: CLI arguments for delete command
        let args = vec!["naaas-ctl", "delete", "tenant-123"];

        // Action: Parse the arguments
        let cli = Cli::parse_from_args(args).unwrap();

        // Assert: Command is parsed as Delete with correct tenant ID
        match cli.command {
            Commands::Delete { tenant_id } => {
                assert_eq!(tenant_id, "tenant-123");
            }
            _ => panic!("Expected Delete command"),
        }
    }

    #[test]
    fn test_cli_parses_health_command() {
        // Setup: CLI arguments for health command
        let args = vec!["naaas-ctl", "health"];

        // Action: Parse the arguments
        let cli = Cli::parse_from_args(args).unwrap();

        // Assert: Command is parsed as Health
        assert!(matches!(cli.command, Commands::Health));
    }

    #[test]
    fn test_cli_fails_to_parse_deploy_command_missing_required_fields() {
        // Setup: CLI arguments missing required name field
        let args = vec![
            "naaas-ctl",
            "deploy",
            "--unikernel", "/path/to/unikernel"
            // Missing --name
        ];

        // Action: Try to parse the arguments
        let result = Cli::parse_from_args(args);

        // Assert: Parsing fails due to missing required field
        assert!(result.is_err(), "Should fail when required field is missing");
    }

    #[test]
    fn test_cli_applies_custom_server_url() {
        // Setup: CLI arguments with custom server URL
        let args = vec![
            "naaas-ctl",
            "--server", "https://production-server.example.com",
            "health"
        ];

        // Action: Parse the arguments
        let cli = Cli::parse_from_args(args).unwrap();

        // Assert: Custom server URL is applied
        assert_eq!(cli.get_server_url(), "https://production-server.example.com");
    }

    #[test]
    fn test_cli_uses_default_server_url_when_not_specified() {
        // Setup: CLI arguments without server URL
        let args = vec!["naaas-ctl", "health"];

        // Action: Parse the arguments
        let cli = Cli::parse_from_args(args).unwrap();

        // Assert: Default server URL is used
        assert_eq!(cli.get_server_url(), "http://localhost:8080");
    }

    #[test]
    fn test_cli_deploy_command_with_upstream_only() {
        // Setup: Deploy command with upstream but no config
        let args = vec![
            "naaas-ctl",
            "deploy",
            "--name", "proxy-only",
            "--unikernel", "/path/to/shim",
            "--upstream", "http://backend-service:8080"
        ];

        // Action: Parse the arguments
        let cli = Cli::parse_from_args(args).unwrap();

        // Assert: Upstream is set, config is None
        match cli.command {
            Commands::Deploy { name, unikernel, port, upstream, config } => {
                assert_eq!(name, "proxy-only");
                assert_eq!(unikernel, "/path/to/shim");
                assert_eq!(port, None);
                assert_eq!(upstream, Some("http://backend-service:8080".to_string()));
                assert_eq!(config, None);
            }
            _ => panic!("Expected Deploy command"),
        }
    }

    #[test]
    fn test_cli_deploy_command_with_config_only() {
        // Setup: Deploy command with config but no upstream
        let args = vec![
            "naaas-ctl",
            "deploy",
            "--name", "config-only",
            "--unikernel", "/path/to/app",
            "--config", r#"{"theme":"dark","features":["logging"]}"#
        ];

        // Action: Parse the arguments
        let cli = Cli::parse_from_args(args).unwrap();

        // Assert: Config is set, upstream is None
        match cli.command {
            Commands::Deploy { name, unikernel, port, upstream, config } => {
                assert_eq!(name, "config-only");
                assert_eq!(unikernel, "/path/to/app");
                assert_eq!(port, None);
                assert_eq!(upstream, None);
                assert_eq!(config, Some(r#"{"theme":"dark","features":["logging"]}"#.to_string()));
            }
            _ => panic!("Expected Deploy command"),
        }
    }

    #[test]
    fn test_cli_deploy_command_with_complex_json_config() {
        // Setup: Deploy command with complex JSON configuration
        let args = vec![
            "naaas-ctl",
            "deploy",
            "--name", "complex-app",
            "--unikernel", "/complex/app",
            "--port", "5000",
            "--upstream", "https://api.complex.example.com/v2",
            "--config", r#"{"name":"Complex App","ui":{"theme":"dark","sidebar":true},"features":{"auth":true,"analytics":false}}"#
        ];

        // Action: Parse the arguments
        let cli = Cli::parse_from_args(args).unwrap();

        // Assert: Complex JSON is preserved correctly
        match cli.command {
            Commands::Deploy { name, unikernel, port, upstream, config } => {
                assert_eq!(name, "complex-app");
                assert_eq!(unikernel, "/complex/app");
                assert_eq!(port, Some(5000));
                assert_eq!(upstream, Some("https://api.complex.example.com/v2".to_string()));
                
                let expected_config = r#"{"name":"Complex App","ui":{"theme":"dark","sidebar":true},"features":{"auth":true,"analytics":false}}"#;
                assert_eq!(config, Some(expected_config.to_string()));
            }
            _ => panic!("Expected Deploy command"),
        }
    }

    #[test] 
    fn test_cli_deploy_command_with_special_characters_in_config() {
        // Setup: Deploy command with JSON containing special characters
        let args = vec![
            "naaas-ctl",
            "deploy",
            "--name", "special-chars",
            "--unikernel", "/path/to/app",
            "--config", r#"{"title":"App with \"quotes\" & symbols","description":"Testing: chars like @#$%"}"#
        ];

        // Action: Parse the arguments
        let cli = Cli::parse_from_args(args).unwrap();

        // Assert: Special characters are preserved
        match cli.command {
            Commands::Deploy { config, .. } => {
                let expected = r#"{"title":"App with \"quotes\" & symbols","description":"Testing: chars like @#$%"}"#;
                assert_eq!(config, Some(expected.to_string()));
            }
            _ => panic!("Expected Deploy command"),
        }
    }
}