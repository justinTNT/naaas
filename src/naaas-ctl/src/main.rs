use anyhow::Result;

// Import our modularized components
use naaas_ctl::{Cli, Commands, NaaasClient, DeployRequest};

async fn deploy_tenant(
    client: &NaaasClient,
    name: String,
    unikernel_path: String,
    port: Option<u16>,
    upstream_url: Option<String>,
    app_config: Option<String>,
) -> Result<()> {
    let mut deploy_req = DeployRequest::new(name.clone(), unikernel_path);
    
    if let Some(port) = port {
        deploy_req = deploy_req.with_port(port);
    }
    if let Some(upstream) = upstream_url {
        deploy_req = deploy_req.with_upstream_url(upstream);
    }
    if let Some(config) = app_config {
        deploy_req = deploy_req.with_app_config(config);
    }
    
    println!("🚀 Deploying tenant '{}'...", name);
    
    match client.deploy_tenant(deploy_req).await {
        Ok(tenant) => {
            println!("✅ Tenant deployed successfully!");
            println!("   ID: {}", tenant.id);
            println!("   Name: {}", tenant.name);
            println!("   Port: {}", tenant.port);
            println!("   Status: {}", tenant.status);
            if let Some(pid) = tenant.process_id {
                println!("   Process ID: {}", pid);
            }
        }
        Err(e) => {
            println!("❌ Deployment failed: {}", e);
        }
    }
    
    Ok(())
}

async fn list_tenants(client: &NaaasClient) -> Result<()> {
    match client.list_tenants().await {
        Ok(tenants) => {
            if tenants.is_empty() {
                println!("No tenants deployed.");
                return Ok(());
            }
            
            println!("📋 Deployed Tenants:");
            println!();
            for tenant in tenants {
                println!("🏷️  {}", tenant.name);
                println!("   ID: {}", tenant.id);
                println!("   Status: {}", tenant.status);
                println!("   Port: {}", tenant.port);
                println!("   Unikernel: {}", tenant.unikernel_path);
                if let Some(pid) = tenant.process_id {
                    println!("   Process ID: {}", pid);
                }
                println!();
            }
        }
        Err(e) => {
            println!("❌ Failed to list tenants: {}", e);
        }
    }
    
    Ok(())
}

async fn delete_tenant(client: &NaaasClient, tenant_id: String) -> Result<()> {
    println!("🗑️  Deleting tenant '{}'...", tenant_id);
    
    match client.delete_tenant(&tenant_id).await {
        Ok(_) => {
            println!("✅ Tenant deleted successfully!");
        }
        Err(e) => {
            println!("❌ Failed to delete tenant: {}", e);
        }
    }
    
    Ok(())
}

async fn check_health(client: &NaaasClient) -> Result<()> {
    match client.check_health().await {
        Ok(_) => {
            println!("✅ NAAAS Server is healthy");
        }
        Err(e) => {
            println!("❌ NAAAS Server health check failed: {}", e);
        }
    }
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse_args();
    let client = NaaasClient::new(cli.server.clone());
    
    match cli.command {
        Commands::Deploy { name, unikernel, port, upstream, config } => {
            deploy_tenant(&client, name, unikernel, port, upstream, config).await?;
        }
        Commands::List => {
            list_tenants(&client).await?;
        }
        Commands::Delete { tenant_id } => {
            delete_tenant(&client, tenant_id).await?;
        }
        Commands::Health => {
            check_health(&client).await?;
        }
    }
    
    Ok(())
}