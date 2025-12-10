// Integration tests for NAAAS CLI workflows
import { describe, it, expect, beforeAll, afterAll, beforeEach } from 'vitest'
import { spawn } from 'child_process'
import { promisify } from 'util'
import { TestServer } from '../helpers/test-server'
import { MockBinaries } from '../helpers/mock-binaries'
import { TestCleanup } from '../helpers/cleanup'

const execCmd = promisify(require('child_process').exec)

describe('NAAAS CLI Workflow Integration', () => {
  let testServer: TestServer
  let mockBinaries: MockBinaries
  let cleanup: TestCleanup
  let serverUrl: string
  let cliPath: string

  beforeAll(async () => {
    testServer = new TestServer()
    mockBinaries = new MockBinaries()
    cleanup = new TestCleanup()
    
    // Start the server
    await testServer.start()
    await testServer.waitForReady()
    serverUrl = testServer.url
    
    // Build the CLI if needed
    cliPath = '/Users/jtnt/Play/naaasaas/src/naaas-ctl/target/debug/naaas-ctl'
    
    // Ensure CLI is built
    await execCmd('cargo build', { 
      cwd: '/Users/jtnt/Play/naaasaas/src/naaas-ctl' 
    })
  })

  afterAll(async () => {
    await cleanup.cleanupAll(serverUrl)
    await mockBinaries.cleanup()
    await testServer.stop()
  })

  beforeEach(async () => {
    // Clean up any tenants from previous tests
    await cleanup.cleanupAll(serverUrl)
  })

  /**
   * Helper function to run CLI commands
   */
  async function runCli(args: string[]): Promise<{ stdout: string; stderr: string; exitCode: number }> {
    return new Promise((resolve) => {
      const child = spawn(cliPath, ['--server', serverUrl, ...args], {
        stdio: 'pipe'
      })

      let stdout = ''
      let stderr = ''

      child.stdout?.on('data', (data) => {
        stdout += data.toString()
      })

      child.stderr?.on('data', (data) => {
        stderr += data.toString()
      })

      child.on('close', (exitCode) => {
        resolve({ stdout, stderr, exitCode: exitCode || 0 })
      })
    })
  }

  describe('Health Command', () => {
    it('should report server as healthy', async () => {
      const result = await runCli(['health'])

      expect(result.exitCode).toBe(0)
      expect(result.stdout).toContain('✅ NAAAS Server is healthy')
      expect(result.stderr).toBe('')
    })

    it('should handle server unavailable gracefully', async () => {
      // Use a non-existent server URL
      const child = spawn(cliPath, ['--server', 'http://localhost:9999', 'health'], {
        stdio: 'pipe'
      })

      let stdout = ''
      let stderr = ''

      child.stdout?.on('data', (data) => {
        stdout += data.toString()
      })

      child.stderr?.on('data', (data) => {
        stderr += data.toString()
      })

      const result = await new Promise<{ stdout: string; stderr: string; exitCode: number }>((resolve) => {
        child.on('close', (exitCode) => {
          resolve({ stdout, stderr, exitCode: exitCode || 0 })
        })
      })

      expect(result.stdout).toContain('❌ NAAAS Server health check failed')
    })
  })

  describe('Deploy Command', () => {
    it('should successfully deploy a tenant', async () => {
      // Setup: Create mock unikernel binary
      const mockBinary = await mockBinaries.createMockUnikernel('cli-deploy-test')

      // Action: Deploy via CLI
      const result = await runCli([
        'deploy',
        '--name', 'cli-test-tenant',
        '--unikernel', mockBinary,
        '--port', '3001'
      ])

      // Assert: Deployment succeeds
      expect(result.exitCode).toBe(0)
      expect(result.stdout).toContain('🚀 Deploying tenant \'cli-test-tenant\'')
      expect(result.stdout).toContain('✅ Tenant deployed successfully!')
      expect(result.stdout).toContain('Name: cli-test-tenant')
      expect(result.stdout).toContain('Port: 3001')

      // Extract tenant ID for cleanup
      const idMatch = result.stdout.match(/ID: ([a-f0-9-]+)/)
      if (idMatch) {
        cleanup.trackTenant(idMatch[1])
      }

      // Extract process ID for cleanup  
      const pidMatch = result.stdout.match(/Process ID: (\d+)/)
      if (pidMatch) {
        cleanup.trackProcess(parseInt(pidMatch[1]))
      }
    })

    it('should deploy with all optional parameters', async () => {
      const mockBinary = await mockBinaries.createMockProxyShim('full-deploy-test')

      const result = await runCli([
        'deploy',
        '--name', 'full-featured-tenant',
        '--unikernel', mockBinary,
        '--port', '3003',
        '--upstream', 'http://localhost:2368',
        '--config', JSON.stringify({ name: 'Full Test', theme: 'blue' })
      ])

      expect(result.exitCode).toBe(0)
      expect(result.stdout).toContain('✅ Tenant deployed successfully!')
      expect(result.stdout).toContain('Name: full-featured-tenant')
      expect(result.stdout).toContain('Port: 3003')

      // Extract IDs for cleanup
      const idMatch = result.stdout.match(/ID: ([a-f0-9-]+)/)
      const pidMatch = result.stdout.match(/Process ID: (\d+)/)
      if (idMatch) cleanup.trackTenant(idMatch[1])
      if (pidMatch) cleanup.trackProcess(parseInt(pidMatch[1]))
    })

    it('should handle deployment failure gracefully', async () => {
      const result = await runCli([
        'deploy',
        '--name', 'failing-tenant',
        '--unikernel', '/nonexistent/binary'
      ])

      expect(result.stdout).toContain('🚀 Deploying tenant \'failing-tenant\'')
      expect(result.stdout).toContain('❌ Deployment failed')
    })

    it('should validate required arguments', async () => {
      // Missing required --name argument
      const result = await runCli([
        'deploy',
        '--unikernel', '/some/path'
      ])

      expect(result.exitCode).not.toBe(0)
      expect(result.stderr).toContain('required')
    })
  })

  describe('List Command', () => {
    it('should show no tenants when none are deployed', async () => {
      const result = await runCli(['list'])

      expect(result.exitCode).toBe(0)
      expect(result.stdout).toContain('No tenants deployed.')
    })

    it('should display deployed tenants correctly', async () => {
      // Setup: Deploy a couple of tenants
      const mockBinary1 = await mockBinaries.createMockUnikernel('list-test-1')
      const mockBinary2 = await mockBinaries.createMockUnikernel('list-test-2')

      // Deploy first tenant
      const deploy1 = await runCli([
        'deploy',
        '--name', 'list-tenant-1',
        '--unikernel', mockBinary1,
        '--port', '3001'
      ])
      expect(deploy1.exitCode).toBe(0)

      // Deploy second tenant
      const deploy2 = await runCli([
        'deploy',
        '--name', 'list-tenant-2', 
        '--unikernel', mockBinary2,
        '--port', '3002'
      ])
      expect(deploy2.exitCode).toBe(0)

      // Extract tenant IDs for cleanup
      const id1Match = deploy1.stdout.match(/ID: ([a-f0-9-]+)/)
      const id2Match = deploy2.stdout.match(/ID: ([a-f0-9-]+)/)
      const pid1Match = deploy1.stdout.match(/Process ID: (\d+)/)
      const pid2Match = deploy2.stdout.match(/Process ID: (\d+)/)

      if (id1Match) cleanup.trackTenant(id1Match[1])
      if (id2Match) cleanup.trackTenant(id2Match[1])
      if (pid1Match) cleanup.trackProcess(parseInt(pid1Match[1]))
      if (pid2Match) cleanup.trackProcess(parseInt(pid2Match[1]))

      // Action: List tenants
      const result = await runCli(['list'])

      // Assert: Both tenants are displayed
      expect(result.exitCode).toBe(0)
      expect(result.stdout).toContain('📋 Deployed Tenants:')
      expect(result.stdout).toContain('🏷️  list-tenant-1')
      expect(result.stdout).toContain('🏷️  list-tenant-2')
      expect(result.stdout).toContain('Port: 3001')
      expect(result.stdout).toContain('Port: 3002')
    })
  })

  describe('Delete Command', () => {
    it('should successfully delete an existing tenant', async () => {
      // Setup: Deploy a tenant to delete
      const mockBinary = await mockBinaries.createMockUnikernel('delete-test')
      
      const deployResult = await runCli([
        'deploy',
        '--name', 'tenant-to-delete',
        '--unikernel', mockBinary
      ])
      expect(deployResult.exitCode).toBe(0)

      // Extract tenant ID
      const idMatch = deployResult.stdout.match(/ID: ([a-f0-9-]+)/)
      expect(idMatch).toBeTruthy()
      const tenantId = idMatch![1]

      // Action: Delete the tenant
      const deleteResult = await runCli(['delete', tenantId])

      // Assert: Deletion succeeds
      expect(deleteResult.exitCode).toBe(0)
      expect(deleteResult.stdout).toContain(`🗑️  Deleting tenant '${tenantId}'`)
      expect(deleteResult.stdout).toContain('✅ Tenant deleted successfully!')

      // Verify tenant no longer appears in list
      const listResult = await runCli(['list'])
      expect(listResult.stdout).not.toContain(tenantId)
    })

    it('should handle deletion of non-existent tenant', async () => {
      const nonExistentId = 'non-existent-tenant-id'

      const result = await runCli(['delete', nonExistentId])

      expect(result.stdout).toContain(`🗑️  Deleting tenant '${nonExistentId}'`)
      expect(result.stdout).toContain('❌ Failed to delete tenant')
      expect(result.stdout).toContain('not found')
    })
  })

  describe('End-to-End Workflow', () => {
    it('should complete full tenant lifecycle: deploy → list → delete', async () => {
      // Create mock binary
      const mockBinary = await mockBinaries.createMockUnikernel('e2e-test')

      // Step 1: Deploy
      const deployResult = await runCli([
        'deploy',
        '--name', 'e2e-tenant',
        '--unikernel', mockBinary,
        '--port', '3005'
      ])
      expect(deployResult.exitCode).toBe(0)
      expect(deployResult.stdout).toContain('✅ Tenant deployed successfully!')

      // Extract tenant ID
      const idMatch = deployResult.stdout.match(/ID: ([a-f0-9-]+)/)
      const tenantId = idMatch![1]

      // Step 2: List and verify
      const listResult = await runCli(['list'])
      expect(listResult.exitCode).toBe(0)
      expect(listResult.stdout).toContain('e2e-tenant')
      expect(listResult.stdout).toContain(tenantId)

      // Step 3: Delete
      const deleteResult = await runCli(['delete', tenantId])
      expect(deleteResult.exitCode).toBe(0)
      expect(deleteResult.stdout).toContain('✅ Tenant deleted successfully!')

      // Step 4: Verify deletion
      const finalListResult = await runCli(['list'])
      expect(finalListResult.stdout).toContain('No tenants deployed.')
    })
  })
})