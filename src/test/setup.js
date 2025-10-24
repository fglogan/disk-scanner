import '@testing-library/jest-dom';

// Mock Tauri API for testing
global.window.__TAURI__ = {
  core: {
    invoke: vi.fn(),
  },
};
