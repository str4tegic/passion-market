import { defineWorkspace } from 'vitest/config'

export default defineWorkspace([
  'packages/api-client/vitest.config.ts',
  'packages/hooks/vitest.config.ts',
])
