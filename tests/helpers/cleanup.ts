// Test cleanup utilities
import { exec } from 'child_process'
import { promisify } from 'util'

const execAsync = promisify(exec)

export class TestCleanup {
  private spawnedProcesses: number[] = []
  private createdTenants: string[] = []

  /**
   * Track a process that was spawned during testing
   */
  trackProcess(pid: number): void {
    this.spawnedProcesses.push(pid)
  }

  /**
   * Track a tenant that was created during testing
   */
  trackTenant(tenantId: string): void {
    this.createdTenants.push(tenantId)
  }

  /**
   * Kill all tracked processes
   */
  async killTrackedProcesses(): Promise<void> {
    for (const pid of this.spawnedProcesses) {
      try {
        // Try graceful termination first
        await execAsync(`kill -TERM ${pid}`)
        
        // Wait a moment for graceful shutdown
        await new Promise(resolve => setTimeout(resolve, 1000))
        
        // Force kill if still running
        await execAsync(`kill -KILL ${pid}`)
      } catch {
        // Process might already be dead, ignore errors
      }
    }
    this.spawnedProcesses = []
  }

  /**
   * Clean up all tracked tenants via API
   */
  async cleanupTenants(serverUrl: string): Promise<void> {
    const axios = await import('axios').then(m => m.default)
    
    for (const tenantId of this.createdTenants) {
      try {
        await axios.delete(`${serverUrl}/tenants/${tenantId}`)
      } catch {
        // Tenant might already be deleted, ignore errors
      }
    }
    this.createdTenants = []
  }

  /**
   * Kill any processes listening on the given port
   */
  async killProcessOnPort(port: number): Promise<void> {
    try {
      // Find processes using the port
      const { stdout } = await execAsync(`lsof -t -i:${port}`)
      const pids = stdout.trim().split('\n').filter(pid => pid)
      
      // Kill each process
      for (const pid of pids) {
        await execAsync(`kill -TERM ${pid}`)
      }
      
      // Wait for cleanup
      await new Promise(resolve => setTimeout(resolve, 1000))
    } catch {
      // No processes on port or other error, ignore
    }
  }

  /**
   * Comprehensive cleanup - call this in test teardown
   */
  async cleanupAll(serverUrl?: string): Promise<void> {
    await this.killTrackedProcesses()
    
    if (serverUrl) {
      await this.cleanupTenants(serverUrl)
    }
    
    // Clean up common test ports
    await this.killProcessOnPort(8080) // Server port
    await this.killProcessOnPort(3001) // Default tenant port
    await this.killProcessOnPort(3002) // Second tenant port
  }
}