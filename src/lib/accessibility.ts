/**
 * Accessibility Enhancement System
 * WCAG 2.1 AA compliant accessibility utilities and management
 */

export interface AccessibilityPreferences {
  highContrast: boolean;
  reducedMotion: boolean;
  largeText: boolean;
  screenReaderOptimizations: boolean;
  keyboardNavigation: boolean;
  focusIndicators: boolean;
}

export interface FocusManagementOptions {
  restoreFocus: boolean;
  trapFocus: boolean;
  skipLinks: boolean;
  announcements: boolean;
}

class AccessibilityManager {
  private preferences: AccessibilityPreferences;
  private focusStack: HTMLElement[] = [];
  private screenReader: ScreenReaderManager;
  private keyboardManager: KeyboardNavigationManager;
  private contrastManager: ContrastManager;
  private announcer: LiveRegionAnnouncer;

  constructor() {
    this.preferences = this.loadPreferences();
    this.screenReader = new ScreenReaderManager();
    this.keyboardManager = new KeyboardNavigationManager();
    this.contrastManager = new ContrastManager();
    this.announcer = new LiveRegionAnnouncer();
    
    this.initialize();
  }

  private initialize() {
    this.applyPreferences();
    this.setupMediaQueryListeners();
    this.createLiveRegions();
    this.enhanceSemantics();
  }

  private loadPreferences(): AccessibilityPreferences {
    const defaults: AccessibilityPreferences = {
      highContrast: false,
      reducedMotion: false,
      largeText: false,
      screenReaderOptimizations: false,
      keyboardNavigation: true,
      focusIndicators: true
    };

    if (typeof window === 'undefined') return defaults;

    // Check system preferences
    if (window.matchMedia) {
      defaults.reducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
      defaults.highContrast = window.matchMedia('(prefers-contrast: high)').matches;
      defaults.largeText = window.matchMedia('(prefers-text-size: large)').matches;
    }

    // Load user preferences from localStorage
    const saved = localStorage.getItem('accessibility-preferences');
    if (saved) {
      try {
        return { ...defaults, ...JSON.parse(saved) };
      } catch (e) {
        console.warn('Failed to parse accessibility preferences');
      }
    }

    return defaults;
  }

  private savePreferences() {
    if (typeof window !== 'undefined') {
      localStorage.setItem('accessibility-preferences', JSON.stringify(this.preferences));
    }
  }

  private applyPreferences() {
    if (typeof document === 'undefined') return;

    const root = document.documentElement;

    // Apply CSS custom properties for accessibility
    root.style.setProperty('--a11y-high-contrast', this.preferences.highContrast ? '1' : '0');
    root.style.setProperty('--a11y-large-text', this.preferences.largeText ? '1' : '0');
    root.style.setProperty('--a11y-reduced-motion', this.preferences.reducedMotion ? '1' : '0');

    // Apply CSS classes
    root.classList.toggle('high-contrast', this.preferences.highContrast);
    root.classList.toggle('large-text', this.preferences.largeText);
    root.classList.toggle('reduced-motion', this.preferences.reducedMotion);
    root.classList.toggle('screen-reader-optimized', this.preferences.screenReaderOptimizations);
    root.classList.toggle('keyboard-navigation', this.preferences.keyboardNavigation);
    root.classList.toggle('focus-indicators', this.preferences.focusIndicators);
  }

  private setupMediaQueryListeners() {
    if (typeof window === 'undefined' || !window.matchMedia) return;

    // Listen for system preference changes
    const reducedMotionQuery = window.matchMedia('(prefers-reduced-motion: reduce)');
    reducedMotionQuery.addEventListener('change', (e) => {
      this.updatePreference('reducedMotion', e.matches);
    });

    const highContrastQuery = window.matchMedia('(prefers-contrast: high)');
    highContrastQuery.addEventListener('change', (e) => {
      this.updatePreference('highContrast', e.matches);
    });

    const largeTextQuery = window.matchMedia('(prefers-text-size: large)');
    largeTextQuery.addEventListener('change', (e) => {
      this.updatePreference('largeText', e.matches);
    });
  }

  private createLiveRegions() {
    if (typeof document === 'undefined') return;

    // Create polite live region for non-urgent announcements
    const politeRegion = document.createElement('div');
    politeRegion.id = 'live-region-polite';
    politeRegion.setAttribute('aria-live', 'polite');
    politeRegion.setAttribute('aria-atomic', 'true');
    politeRegion.className = 'sr-only';
    document.body.appendChild(politeRegion);

    // Create assertive live region for urgent announcements
    const assertiveRegion = document.createElement('div');
    assertiveRegion.id = 'live-region-assertive';
    assertiveRegion.setAttribute('aria-live', 'assertive');
    assertiveRegion.setAttribute('aria-atomic', 'true');
    assertiveRegion.className = 'sr-only';
    document.body.appendChild(assertiveRegion);
  }

  private enhanceSemantics() {
    if (typeof document === 'undefined') return;

    // Add semantic enhancements
    this.enhanceButtons();
    this.enhanceLinks();
    this.enhanceForm();
    this.enhanceNavigation();
  }

  private enhanceButtons() {
    const buttons = document.querySelectorAll('button, [role="button"]');
    buttons.forEach(button => {
      if (!button.hasAttribute('aria-label') && !button.textContent?.trim()) {
        console.warn('Button without accessible name detected:', button);
      }
    });
  }

  private enhanceLinks() {
    const links = document.querySelectorAll('a');
    links.forEach(link => {
      if (link.target === '_blank' && !link.getAttribute('aria-label')?.includes('opens in new')) {
        const currentLabel = link.getAttribute('aria-label') || link.textContent || '';
        link.setAttribute('aria-label', `${currentLabel} (opens in new window)`);
      }
    });
  }

  private enhanceForm() {
    const inputs = document.querySelectorAll('input, select, textarea');
    inputs.forEach(input => {
      if (!input.hasAttribute('aria-label') && !input.hasAttribute('aria-labelledby')) {
        const label = document.querySelector(`label[for="${input.id}"]`);
        if (!label && input.id) {
          console.warn('Form control without associated label:', input);
        }
      }
    });
  }

  private enhanceNavigation() {
    const nav = document.querySelector('nav');
    if (nav && !nav.hasAttribute('aria-label')) {
      nav.setAttribute('aria-label', 'Main navigation');
    }
  }

  public updatePreference<K extends keyof AccessibilityPreferences>(
    key: K,
    value: AccessibilityPreferences[K]
  ) {
    this.preferences[key] = value;
    this.applyPreferences();
    this.savePreferences();
    
    // Announce preference change
    this.announcer.announce(`${key} ${value ? 'enabled' : 'disabled'}`, 'polite');
  }

  public getPreferences(): AccessibilityPreferences {
    return { ...this.preferences };
  }

  public pushFocus(element: HTMLElement) {
    const currentFocus = document.activeElement as HTMLElement;
    if (currentFocus) {
      this.focusStack.push(currentFocus);
    }
    element.focus();
  }

  public popFocus() {
    const element = this.focusStack.pop();
    if (element) {
      element.focus();
    }
  }

  public trapFocus(container: HTMLElement, options: Partial<FocusManagementOptions> = {}) {
    return this.keyboardManager.trapFocus(container, options);
  }

  public announce(message: string, priority: 'polite' | 'assertive' = 'polite') {
    this.announcer.announce(message, priority);
  }

  public createSkipLink(targetId: string, text: string) {
    return this.keyboardManager.createSkipLink(targetId, text);
  }
}

class ScreenReaderManager {
  public optimizeForScreenReader(element: HTMLElement) {
    // Add screen reader specific optimizations
    element.setAttribute('aria-describedby', this.createDescription(element));
  }

  private createDescription(element: HTMLElement): string {
    // Generate contextual descriptions for screen readers
    const role = element.getAttribute('role') || element.tagName.toLowerCase();
    const purpose = this.inferPurpose(element);
    
    const descId = `desc-${Math.random().toString(36).substr(2, 9)}`;
    const descElement = document.createElement('div');
    descElement.id = descId;
    descElement.className = 'sr-only';
    descElement.textContent = `${role} for ${purpose}`;
    
    element.appendChild(descElement);
    return descId;
  }

  private inferPurpose(element: HTMLElement): string {
    // Smart purpose inference based on context
    const className = element.className;
    const id = element.id;
    const parent = element.parentElement;
    
    if (className.includes('scan')) return 'scanning operations';
    if (className.includes('delete')) return 'file deletion';
    if (className.includes('nav')) return 'navigation';
    if (parent?.tagName === 'FORM') return 'form input';
    
    return 'interface interaction';
  }
}

class KeyboardNavigationManager {
  private focusableSelectors = [
    'button:not([disabled])',
    'input:not([disabled])',
    'select:not([disabled])',
    'textarea:not([disabled])',
    'a[href]',
    '[tabindex]:not([tabindex="-1"])',
    '[role="button"]:not([disabled])',
    '[role="link"]'
  ].join(', ');

  public trapFocus(container: HTMLElement, options: Partial<FocusManagementOptions> = {}): () => void {
    const focusableElements = container.querySelectorAll(this.focusableSelectors) as NodeListOf<HTMLElement>;
    
    if (focusableElements.length === 0) return () => {};

    const firstElement = focusableElements[0];
    const lastElement = focusableElements[focusableElements.length - 1];

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Tab') {
        if (e.shiftKey) {
          if (document.activeElement === firstElement) {
            e.preventDefault();
            lastElement.focus();
          }
        } else {
          if (document.activeElement === lastElement) {
            e.preventDefault();
            firstElement.focus();
          }
        }
      }
    };

    container.addEventListener('keydown', handleKeyDown);
    firstElement.focus();

    return () => {
      container.removeEventListener('keydown', handleKeyDown);
    };
  }

  public createSkipLink(targetId: string, text: string): HTMLElement {
    const skipLink = document.createElement('a');
    skipLink.href = `#${targetId}`;
    skipLink.textContent = text;
    skipLink.className = 'skip-link';
    skipLink.setAttribute('aria-label', `Skip to ${text}`);
    
    // Position absolutely and hide until focused
    skipLink.style.cssText = `
      position: absolute;
      top: -40px;
      left: 6px;
      background: var(--color-primary);
      color: white;
      padding: 8px;
      border-radius: 4px;
      text-decoration: none;
      z-index: 1000;
      transition: top 0.2s;
    `;

    skipLink.addEventListener('focus', () => {
      skipLink.style.top = '6px';
    });

    skipLink.addEventListener('blur', () => {
      skipLink.style.top = '-40px';
    });

    return skipLink;
  }

  public enhanceKeyboardNavigation(element: HTMLElement) {
    // Add arrow key navigation for custom components
    element.addEventListener('keydown', this.handleArrowKeys.bind(this));
  }

  private handleArrowKeys(e: KeyboardEvent) {
    const target = e.target as HTMLElement;
    const role = target.getAttribute('role');
    
    if (role === 'listbox' || role === 'menu') {
      this.handleListNavigation(e);
    } else if (role === 'tablist') {
      this.handleTabNavigation(e);
    }
  }

  private handleListNavigation(e: KeyboardEvent) {
    const container = e.currentTarget as HTMLElement;
    const items = container.querySelectorAll('[role="option"], [role="menuitem"]') as NodeListOf<HTMLElement>;
    const currentIndex = Array.from(items).indexOf(e.target as HTMLElement);
    
    let nextIndex = currentIndex;
    
    switch (e.key) {
      case 'ArrowDown':
        e.preventDefault();
        nextIndex = (currentIndex + 1) % items.length;
        break;
      case 'ArrowUp':
        e.preventDefault();
        nextIndex = currentIndex === 0 ? items.length - 1 : currentIndex - 1;
        break;
      case 'Home':
        e.preventDefault();
        nextIndex = 0;
        break;
      case 'End':
        e.preventDefault();
        nextIndex = items.length - 1;
        break;
    }
    
    if (nextIndex !== currentIndex) {
      items[nextIndex].focus();
    }
  }

  private handleTabNavigation(e: KeyboardEvent) {
    const container = e.currentTarget as HTMLElement;
    const tabs = container.querySelectorAll('[role="tab"]') as NodeListOf<HTMLElement>;
    const currentIndex = Array.from(tabs).indexOf(e.target as HTMLElement);
    
    let nextIndex = currentIndex;
    
    switch (e.key) {
      case 'ArrowRight':
        e.preventDefault();
        nextIndex = (currentIndex + 1) % tabs.length;
        break;
      case 'ArrowLeft':
        e.preventDefault();
        nextIndex = currentIndex === 0 ? tabs.length - 1 : currentIndex - 1;
        break;
    }
    
    if (nextIndex !== currentIndex) {
      tabs[nextIndex].focus();
      tabs[nextIndex].click(); // Activate the tab
    }
  }
}

class ContrastManager {
  private highContrastStyles: string = `
    .high-contrast {
      --color-bg: #000000 !important;
      --color-text: #ffffff !important;
      --color-primary: #ffff00 !important;
      --color-secondary: #00ffff !important;
      --color-accent: #ff00ff !important;
      --color-border: #ffffff !important;
      --color-focus: #ffff00 !important;
    }
    
    .high-contrast button {
      border: 2px solid var(--color-border) !important;
    }
    
    .high-contrast a {
      color: var(--color-primary) !important;
      text-decoration: underline !important;
    }
    
    .high-contrast :focus {
      outline: 3px solid var(--color-focus) !important;
      outline-offset: 2px !important;
    }
  `;

  public enableHighContrast() {
    this.injectStyles();
    document.documentElement.classList.add('high-contrast');
  }

  public disableHighContrast() {
    document.documentElement.classList.remove('high-contrast');
  }

  private injectStyles() {
    if (document.getElementById('high-contrast-styles')) return;
    
    const style = document.createElement('style');
    style.id = 'high-contrast-styles';
    style.textContent = this.highContrastStyles;
    document.head.appendChild(style);
  }
}

class LiveRegionAnnouncer {
  public announce(message: string, priority: 'polite' | 'assertive' = 'polite') {
    const regionId = priority === 'assertive' ? 'live-region-assertive' : 'live-region-polite';
    const region = document.getElementById(regionId);
    
    if (region) {
      // Clear and set to ensure announcement
      region.textContent = '';
      setTimeout(() => {
        region.textContent = message;
      }, 10);
      
      // Clear after announcement
      setTimeout(() => {
        region.textContent = '';
      }, 1000);
    }
  }
}

// Global accessibility manager instance
export const accessibilityManager = new AccessibilityManager();

// Accessibility directive for Svelte components
export function accessibility(node: HTMLElement, _options: Partial<FocusManagementOptions> = {}) {
  const cleanup = accessibilityManager.trapFocus(node, _options);
  
  return {
    destroy() {
      cleanup();
    }
  };
}

// ARIA enhancement utilities
export function enhanceARIA(element: HTMLElement, enhancements: Record<string, string>) {
  Object.entries(enhancements).forEach(([attr, value]) => {
    element.setAttribute(attr, value);
  });
}

// Screen reader text utility
export function createScreenReaderText(text: string): HTMLElement {
  const span = document.createElement('span');
  span.className = 'sr-only';
  span.textContent = text;
  return span;
}