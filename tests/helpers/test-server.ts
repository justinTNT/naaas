// Test server utilities for integration testing
import { spawn, ChildProcess } from 'child_process'
import axios from 'axios'

export class TestServer {
  private process?: ChildProcess
  private port: number
  private baseUrl: string

  constructor(port: number = 8080) {
    this.port = port
    this.baseUrl = `http://localhost:${port}`
  }

  /**
   * Start the NAAAS server for testing
   */
  async start(): Promise<void> {
    return new Promise((resolve, reject) => {
      // Start the server process
      this.process = spawn('cargo', ['run'], {
        cwd: '/Users/jtnt/Play/naaasaas/src/naaas-server',
        stdio: 'pipe',
        env: { ...process.env, RUST_LOG: 'debug' }
      })

      // Handle startup errors
      this.process.stderr?.on('data', (data) => {
        const output = data.toString()
        if (output.includes('error') || output.includes('Error')) {
          reject(new Error(`Server startup failed: ${output}`))
        }
      })

      // Wait for server to be ready
      this.process.stdout?.on('data', (data) => {
        const output = data.toString()
        if (output.includes('starting on')) {
          // Give server a moment to fully initialize
          setTimeout(() => resolve(), 1000)
        }
      })

      // Handle process exit
      this.process.on('exit', (code) => {
        if (code !== 0) {
          reject(new Error(`Server exited with code ${code}`))
        }
      })

      // Set a timeout for startup
      setTimeout(() => {
        reject(new Error('Server startup timeout'))
      }, 10000)
    })
  }

  /**
   * Stop the test server
   */
  async stop(): Promise<void> {
    if (this.process) {
      this.process.kill('SIGTERM')
      
      // Wait for graceful shutdown
      return new Promise((resolve) => {
        this.process?.on('exit', () => resolve())
        
        // Force kill after timeout
        setTimeout(() => {
          this.process?.kill('SIGKILL')
          resolve()
        }, 5000)
      })
    }
  }

  /**
   * Check if server is responding
   */
  async isHealthy(): Promise<boolean> {
    try {
      const response = await axios.get(`${this.baseUrl}/health`, {
        timeout: 2000
      })
      return response.status === 200
    } catch {
      return false
    }
  }

  /**
   * Wait for server to be ready
   */
  async waitForReady(maxAttempts: number = 30): Promise<void> {
    for (let i = 0; i < maxAttempts; i++) {
      if (await this.isHealthy()) {
        return
      }
      await new Promise(resolve => setTimeout(resolve, 1000))
    }
    throw new Error('Server failed to become ready')
  }

  get url(): string {
    return this.baseUrl
  }
}