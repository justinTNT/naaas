import { defineConfig } from 'vitest/config'

export default defineConfig({
  test: {
    globals: true,
    environment: 'node',
    timeout: 10000, // 10 second timeout for integration tests
    setupFiles: ['./helpers/setup.ts']
  }
})