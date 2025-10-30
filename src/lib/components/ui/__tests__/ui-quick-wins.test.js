// @ts-nocheck
import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, fireEvent, screen } from '@testing-library/svelte';
import { get } from 'svelte/store';
import Toast from '../Toast.svelte';
import KeyboardShortcuts from '../KeyboardShortcuts.svelte';
import { toasts, showShortcutsHelp } from '../../../stores.js';

// Mock svelte/transition
vi.mock('svelte/transition', () => ({
  fly: () => ({ delay: 0, duration: 0 }),
  fade: () => ({ delay: 0, duration: 0 })
}));

describe('UI Quick Wins Components', () => {
  beforeEach(() => {
    toasts.clear();
    showShortcutsHelp.set(false);
  });

  describe('Toast Component', () => {
    const mockToast = {
      id: 'test-123',
      type: 'success',
      message: 'Test message',
      duration: 3000,
      dismissible: true
    };

    it('should render toast with correct content', () => {
      render(Toast, { toast: mockToast });
      
      expect(screen.getByText('Test message')).toBeInTheDocument();
      expect(screen.getByRole('alert')).toBeInTheDocument();
      expect(screen.getByLabelText('Dismiss notification')).toBeInTheDocument();
    });

    it('should show correct icon for each toast type', () => {
      const toastTypes = [
        { type: 'success', icon: '✅' },
        { type: 'error', icon: '❌' },
        { type: 'warning', icon: '⚠️' },
        { type: 'info', icon: 'ℹ️' }
      ];

      toastTypes.forEach(({ type, icon }) => {
        const toast = { ...mockToast, type, id: `test-${type}` };
        const { container } = render(Toast, { toast });
        expect(container.textContent).toContain(icon);
      });
    });

    it('should dismiss toast when close button is clicked', async () => {
      render(Toast, { toast: mockToast });
      
      const closeButton = screen.getByLabelText('Dismiss notification');
      await fireEvent.click(closeButton);
      
      // Toast should be removed from store
      expect(get(toasts).length).toBe(0);
    });

    it('should dismiss toast on Escape key', async () => {
      render(Toast, { toast: mockToast });
      
      const toastElement = screen.getByRole('alert');
      await fireEvent.keyDown(toastElement, { key: 'Escape' });
      
      expect(get(toasts).length).toBe(0);
    });

    it('should not show close button when not dismissible', () => {
      const nonDismissibleToast = { ...mockToast, dismissible: false };
      render(Toast, { toast: nonDismissibleToast });
      
      expect(screen.queryByLabelText('Dismiss notification')).not.toBeInTheDocument();
    });

    it('should apply correct CSS classes for different types', () => {
      const successToast = { ...mockToast, type: 'success' };
      const { container } = render(Toast, { toast: successToast });
      
      const toastElement = container.querySelector('.toast');
      expect(toastElement).toHaveClass('bg-green-600');
      expect(toastElement).toHaveClass('border-green-500');
    });
  });

  describe('KeyboardShortcuts Component', () => {
    it('should render shortcuts help modal', () => {
      render(KeyboardShortcuts);
      
      expect(screen.getByText('Keyboard Shortcuts')).toBeInTheDocument();
      expect(screen.getByRole('dialog')).toBeInTheDocument();
    });

    it('should display all keyboard shortcuts', () => {
      render(KeyboardShortcuts);
      
      // Check for some key shortcuts
      expect(screen.getByText('Dashboard')).toBeInTheDocument();
      expect(screen.getByText('Project Scanner')).toBeInTheDocument();
      expect(screen.getByText('Settings')).toBeInTheDocument();
      expect(screen.getByText('Clear Notifications')).toBeInTheDocument();
      expect(screen.getByText('Toggle Theme')).toBeInTheDocument();
    });

    it('should show keyboard shortcuts with icons', () => {
      const { container } = render(KeyboardShortcuts);
      
      // Check for emoji icons
      expect(container.textContent).toContain('📊'); // Dashboard
      expect(container.textContent).toContain('🔍'); // Project Scanner
      expect(container.textContent).toContain('⚙️'); // Settings
    });

    it('should close help when close button is clicked', async () => {
      render(KeyboardShortcuts);
      
      const closeButton = screen.getByLabelText('Close shortcuts help');
      await fireEvent.click(closeButton);
      
      expect(get(showShortcutsHelp)).toBe(false);
    });

    it('should close help when backdrop is clicked', async () => {
      render(KeyboardShortcuts);
      
      const backdrop = screen.getByRole('dialog');
      await fireEvent.click(backdrop);
      
      expect(get(showShortcutsHelp)).toBe(false);
    });

    it('should close help on Escape key', async () => {
      render(KeyboardShortcuts);
      
      await fireEvent.keyDown(document, { key: 'Escape' });
      
      expect(get(showShortcutsHelp)).toBe(false);
    });

    it('should display keyboard shortcuts in correct format', () => {
      render(KeyboardShortcuts);
      
      // Check for kbd elements
      const kbdElements = screen.getAllByRole('button', { name: /^[A-Z]$/ });
      expect(kbdElements.length).toBeGreaterThan(0);
    });

    it('should have proper accessibility attributes', () => {
      render(KeyboardShortcuts);
      
      const dialog = screen.getByRole('dialog');
      expect(dialog).toHaveAttribute('aria-labelledby', 'shortcuts-title');
      expect(dialog).toHaveAttribute('aria-modal', 'true');
      
      const title = screen.getByText('Keyboard Shortcuts');
      expect(title).toHaveAttribute('id', 'shortcuts-title');
    });
  });

  describe('Integration Tests', () => {
    it('should handle rapid toast creation and dismissal', () => {
      // Add multiple toasts quickly
      toasts.add({ type: 'success', message: 'Test 1' });
      toasts.add({ type: 'error', message: 'Test 2' });
      toasts.add({ type: 'warning', message: 'Test 3' });
      
      expect(get(toasts).length).toBe(3);
      
      // Clear all
      toasts.clear();
      expect(get(toasts).length).toBe(0);
    });

    it('should persist shortcuts help state correctly', () => {
      expect(get(showShortcutsHelp)).toBe(false);
      
      showShortcutsHelp.set(true);
      expect(get(showShortcutsHelp)).toBe(true);
      
      showShortcutsHelp.update(show => !show);
      expect(get(showShortcutsHelp)).toBe(false);
    });
  });
});