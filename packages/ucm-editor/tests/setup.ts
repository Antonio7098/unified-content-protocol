/**
 * Test setup for UCM Editor.
 */

import { beforeAll, afterEach } from 'vitest'

// Mock window.matchMedia for tests
beforeAll(() => {
  Object.defineProperty(window, 'matchMedia', {
    writable: true,
    value: (query: string) => ({
      matches: false,
      media: query,
      onchange: null,
      addListener: () => {},
      removeListener: () => {},
      addEventListener: () => {},
      removeEventListener: () => {},
      dispatchEvent: () => true,
    }),
  })
})

// Clean up after each test
afterEach(() => {
  // Reset any global state if needed
})
