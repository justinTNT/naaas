// Global test setup for NAAAS integration tests
import { beforeEach, afterEach } from 'vitest'

// Global setup that runs before each test
beforeEach(() => {
  // Reset any global state if needed
  // This runs before each test to ensure clean slate
})

// Global cleanup that runs after each test
afterEach(() => {
  // Clean up any resources created during tests
  // This helps prevent test pollution
})

// Export common test utilities
export const TEST_CONFIG = {
  SERVER_URL: 'http://localhost:8080',
  TEST_TIMEOUT: 5000,
  RETRY_ATTEMPTS: 3,
}