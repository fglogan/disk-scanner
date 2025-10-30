import '@testing-library/jest-dom';
import { vi } from 'vitest';

// Mock Tauri API for testing
global.window.__TAURI__ = {
  core: {
    invoke: vi.fn(),
  },
};

// Mock Tauri plugins
global.window.__TAURI_PLUGIN_DIALOG__ = {
  open: vi.fn(),
  save: vi.fn(),
  message: vi.fn(),
  ask: vi.fn(),
  confirm: vi.fn(),
};
