// Integration tests for NAAAS Server API
import { describe, it, expect, beforeAll, afterAll, beforeEach } from 'vitest'
import axios from 'axios'
import { TestServer } from '../helpers/test-server'
import { MockBinaries } from '../helpers/mock-binaries'
import { TestCleanup } from '../helpers/cleanup'

describe('NAAAS Server API Integration', () => {
  let testServer: TestServer
  let mockBinaries: MockBinaries
  let cleanup: TestCleanup
  let serverUrl: string

  beforeAll(async () => {
    testServer = new TestServer()
    mockBinaries = new MockBinaries()
    cleanup = new TestCleanup()
    
    // Start the server
    await testServer.start()
    await testServer.waitForReady()
    serverUrl = testServer.url
  })

  afterAll(async () => {
    await cleanup.cleanupAll(serverUrl)
    await mockBinaries.cleanup()
    await testServer.stop()
  })

  beforeEach(async () => {
    // Clean up any tenants from previous tests
    try {
      const response = await axios.get(`${serverUrl}/tenants`)
      const tenants = response.data
      
      for (const tenant of tenants) {
        await axios.delete(`${serverUrl}/tenants/${tenant.id}`)
      }
    } catch {
      // Server might not be ready yet, ignore
    }
  })

  describe('Health Endpoint', () => {
    it('should return healthy status', async () => {
      const response = await axios.get(`${serverUrl}/health`)
      
      expect(response.status).toBe(200)
      expect(response.data).toEqual({
        status: 'healthy',
        service: 'naaas-server'
      })
    })
  })

  describe('Deploy Endpoint', () => {
    it('should successfully deploy a tenant with valid request', async () => {
      // Setup: Create a mock unikernel binary
      const mockBinary = await mockBinaries.createMockUnikernel('test-tenant')
      
      const deployRequest = {
        name: 'integration-test-tenant',
        unikernel_path: mockBinary,
        port: 3001,
        upstream_url: 'http://localhost:2368',
        app_config: JSON.stringify({ name: 'Test App' })
      }

      // Action: Deploy the tenant
      const response = await axios.post(`${serverUrl}/deploy`, deployRequest)

      // Assert: Deployment succeeds with expected data
      expect(response.status).toBe(201)
      expect(response.data).toMatchObject({
        name: 'integration-test-tenant',
        port: 3001,
        status: 'running',
        unikernel_path: mockBinary
      })
      expect(response.data.id).toBeTruthy()
      expect(response.data.process_id).toBeTypeOf('number')
      
      // Track for cleanup
      cleanup.trackTenant(response.data.id)
      cleanup.trackProcess(response.data.process_id)
    })

    it('should reject deployment with invalid JSON', async () => {
      const invalidRequest = 'not valid json'

      try {
        await axios.post(`${serverUrl}/deploy`, invalidRequest, {
          headers: { 'Content-Type': 'application/json' }
        })
        expect.fail('Should have thrown an error')
      } catch (error: any) {
        expect(error.response.status).toBe(400)
      }
    })

    it('should reject deployment with missing required fields', async () => {
      const invalidRequest = {
        name: '', // Empty name should fail validation
        unikernel_path: '/valid/path'
      }

      try {
        await axios.post(`${serverUrl}/deploy`, invalidRequest)
        expect.fail('Should have thrown an error')
      } catch (error: any) {
        expect(error.response.status).toBe(400)
        expect(error.response.data.error).toContain('name cannot be empty')
      }
    })

    it('should handle process spawn failure gracefully', async () => {
      const deployRequest = {
        name: 'failing-tenant',
        unikernel_path: '/nonexistent/binary/path'
      }

      try {
        await axios.post(`${serverUrl}/deploy`, deployRequest)
        expect.fail('Should have thrown an error')
      } catch (error: any) {
        expect(error.response.status).toBe(500)
        expect(error.response.data.error).toContain('Failed to start unikernel')
      }
    })
  })

  describe('List Tenants Endpoint', () => {
    it('should return empty array when no tenants deployed', async () => {
      const response = await axios.get(`${serverUrl}/tenants`)
      
      expect(response.status).toBe(200)
      expect(response.data).toEqual([])
    })

    it('should return all deployed tenants', async () => {
      // Setup: Deploy two test tenants
      const mockBinary1 = await mockBinaries.createMockUnikernel('tenant-1')
      const mockBinary2 = await mockBinaries.createMockUnikernel('tenant-2')
      
      const tenant1Response = await axios.post(`${serverUrl}/deploy`, {
        name: 'tenant-one',
        unikernel_path: mockBinary1,
        port: 3001
      })
      
      const tenant2Response = await axios.post(`${serverUrl}/deploy`, {
        name: 'tenant-two',
        unikernel_path: mockBinary2,
        port: 3002
      })

      cleanup.trackTenant(tenant1Response.data.id)
      cleanup.trackTenant(tenant2Response.data.id)
      cleanup.trackProcess(tenant1Response.data.process_id)
      cleanup.trackProcess(tenant2Response.data.process_id)

      // Action: List all tenants
      const response = await axios.get(`${serverUrl}/tenants`)

      // Assert: Both tenants are returned
      expect(response.status).toBe(200)
      expect(response.data).toHaveLength(2)
      
      const tenantNames = response.data.map((t: any) => t.name)
      expect(tenantNames).toContain('tenant-one')
      expect(tenantNames).toContain('tenant-two')
    })
  })

  describe('Delete Tenant Endpoint', () => {
    it('should successfully delete an existing tenant', async () => {
      // Setup: Deploy a tenant to delete
      const mockBinary = await mockBinaries.createMockUnikernel('delete-test')
      
      const deployResponse = await axios.post(`${serverUrl}/deploy`, {
        name: 'tenant-to-delete',
        unikernel_path: mockBinary
      })
      
      const tenantId = deployResponse.data.id
      cleanup.trackProcess(deployResponse.data.process_id)

      // Action: Delete the tenant
      const deleteResponse = await axios.delete(`${serverUrl}/tenants/${tenantId}`)

      // Assert: Deletion succeeds
      expect(deleteResponse.status).toBe(200)
      expect(deleteResponse.data.message).toContain('deleted successfully')

      // Verify tenant is removed from list
      const listResponse = await axios.get(`${serverUrl}/tenants`)
      const remainingTenants = listResponse.data
      expect(remainingTenants.find((t: any) => t.id === tenantId)).toBeUndefined()
    })

    it('should return 404 for non-existent tenant', async () => {
      const nonExistentId = 'non-existent-tenant-id'

      try {
        await axios.delete(`${serverUrl}/tenants/${nonExistentId}`)
        expect.fail('Should have thrown an error')
      } catch (error: any) {
        expect(error.response.status).toBe(404)
        expect(error.response.data.error).toContain('not found')
      }
    })
  })

  describe('Concurrent Operations', () => {
    it('should handle multiple simultaneous deployments', async () => {
      // Setup: Create multiple mock binaries
      const mockBinaries_list = await Promise.all([
        mockBinaries.createMockUnikernel('concurrent-1'),
        mockBinaries.createMockUnikernel('concurrent-2'),
        mockBinaries.createMockUnikernel('concurrent-3')
      ])

      // Action: Deploy all tenants concurrently
      const deployPromises = mockBinaries_list.map((binaryPath, index) =>
        axios.post(`${serverUrl}/deploy`, {
          name: `concurrent-tenant-${index}`,
          unikernel_path: binaryPath,
          port: 3001 + index
        })
      )

      const responses = await Promise.all(deployPromises)

      // Assert: All deployments succeed with unique IDs and ports
      expect(responses).toHaveLength(3)
      responses.forEach((response, index) => {
        expect(response.status).toBe(201)
        expect(response.data.name).toBe(`concurrent-tenant-${index}`)
        expect(response.data.port).toBe(3001 + index)
        
        // Track for cleanup
        cleanup.trackTenant(response.data.id)
        cleanup.trackProcess(response.data.process_id)
      })

      // Verify unique IDs
      const tenantIds = responses.map(r => r.data.id)
      const uniqueIds = new Set(tenantIds)
      expect(uniqueIds.size).toBe(3) // All IDs should be unique
    })
  })
})