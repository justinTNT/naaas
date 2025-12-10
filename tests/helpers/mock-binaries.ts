// Mock binary utilities for testing unikernel deployment
import { promises as fs } from 'fs'
import { join } from 'path'
import { tmpdir } from 'os'

export class MockBinaries {
  private tempDir: string
  private mockBinaries: string[] = []

  constructor() {
    this.tempDir = tmpdir()
  }

  /**
   * Create a mock unikernel binary that behaves predictably for testing
   */
  async createMockUnikernel(name: string, behavior: 'success' | 'fail' | 'hang' = 'success'): Promise<string> {
    const binaryPath = join(this.tempDir, `mock-${name}`)
    
    // Create a shell script that simulates unikernel behavior
    let script = '#!/bin/bash\n'
    
    switch (behavior) {
      case 'success':
        script += 'echo "Mock unikernel starting..."\n'
        script += 'while true; do\n'
        script += '  sleep 1\n'
        script += 'done\n'
        break
      
      case 'fail':
        script += 'echo "Mock unikernel failed to start" >&2\n'
        script += 'exit 1\n'
        break
      
      case 'hang':
        script += '# Simulate a hanging process\n'
        script += 'sleep 3600\n'  // Sleep for an hour
        break
    }

    await fs.writeFile(binaryPath, script)
    await fs.chmod(binaryPath, '755')
    
    this.mockBinaries.push(binaryPath)
    return binaryPath
  }

  /**
   * Create a mock proxy shim binary for Sprint 2 testing
   */
  async createMockProxyShim(name: string): Promise<string> {
    const binaryPath = join(this.tempDir, `mock-proxy-${name}`)
    
    const script = `#!/bin/bash
echo "Mock proxy shim starting..."
PORT=3000
UPSTREAM=""
CONFIG=""

# Parse arguments
while [[ $# -gt 0 ]]; do
  case $1 in
    --port)
      PORT="$2"
      shift 2
      ;;
    --upstream)
      UPSTREAM="$2"
      shift 2
      ;;
    --config)
      CONFIG="$2"
      shift 2
      ;;
    *)
      shift
      ;;
  esac
done

echo "Mock proxy listening on port $PORT"
echo "Upstream: $UPSTREAM"
echo "Config: $CONFIG"

# Simulate running proxy
while true; do
  sleep 1
done
`

    await fs.writeFile(binaryPath, script)
    await fs.chmod(binaryPath, '755')
    
    this.mockBinaries.push(binaryPath)
    return binaryPath
  }

  /**
   * Get the path to a real binary for testing (like the actual naaas-shim)
   */
  async getRealBinary(component: 'server' | 'ctl' | 'shim'): Promise<string> {
    const basePath = '/Users/jtnt/Play/naaasaas/src'
    const componentPath = join(basePath, `naaas-${component}`)
    
    return join(componentPath, 'target', 'debug', `naaas-${component}`)
  }

  /**
   * Check if a binary exists and is executable
   */
  async isBinaryReady(binaryPath: string): Promise<boolean> {
    try {
      const stats = await fs.stat(binaryPath)
      return stats.isFile() && (stats.mode & 0o111) !== 0
    } catch {
      return false
    }
  }

  /**
   * Clean up all created mock binaries
   */
  async cleanup(): Promise<void> {
    for (const binaryPath of this.mockBinaries) {
      try {
        await fs.unlink(binaryPath)
      } catch {
        // Ignore errors during cleanup
      }
    }
    this.mockBinaries = []
  }
}