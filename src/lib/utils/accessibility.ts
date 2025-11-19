// Accessibility utilities for WCAG AA compliance

export interface AriaProps {
  label?: string;
  labelledby?: string;
  describedby?: string;
  expanded?: boolean;
  selected?: boolean;
  checked?: boolean;
  hidden?: boolean;
  disabled?: boolean;
  required?: boolean;
  invalid?: boolean;
  live?: 'polite' | 'assertive' | 'off';
  role?: string;
}

// Generate ARIA attributes from props
export function ariaProps(props: AriaProps): Record<string, string | boolean | undefined> {
  const attrs: Record<string, string | boolean | undefined> = {};
  
  if (props.label) attrs['aria-label'] = props.label;
  if (props.labelledby) attrs['aria-labelledby'] = props.labelledby;
  if (props.describedby) attrs['aria-describedby'] = props.describedby;
  if (props.expanded !== undefined) attrs['aria-expanded'] = props.expanded;
  if (props.selected !== undefined) attrs['aria-selected'] = props.selected;
  if (props.checked !== undefined) attrs['aria-checked'] = props.checked;
  if (props.hidden !== undefined) attrs['aria-hidden'] = props.hidden;
  if (props.disabled !== undefined) attrs['aria-disabled'] = props.disabled;
  if (props.required !== undefined) attrs['aria-required'] = props.required;
  if (props.invalid !== undefined) attrs['aria-invalid'] = props.invalid;
  if (props.live) attrs['aria-live'] = props.live;
  if (props.role) attrs['role'] = props.role;
  
  return attrs;
}

// Keyboard navigation helpers
export const KEYS = {
  ENTER: 'Enter',
  SPACE: ' ',
  ESCAPE: 'Escape',
  TAB: 'Tab',
  ARROW_UP: 'ArrowUp',
  ARROW_DOWN: 'ArrowDown',
  ARROW_LEFT: 'ArrowLeft',
  ARROW_RIGHT: 'ArrowRight',
  HOME: 'Home',
  END: 'End',
  PAGE_UP: 'PageUp',
  PAGE_DOWN: 'PageDown'
} as const;

// Check if key event matches
export function isKey(event: KeyboardEvent, key: string): boolean {
  return event.key === key;
}

// Trap focus within an element
export function trapFocus(element: HTMLElement) {
  const focusableElements = element.querySelectorAll<HTMLElement>(
    'a[href], button, textarea, input[type="text"], input[type="radio"], input[type="checkbox"], select, [tabindex]:not([tabindex="-1"])'
  );
  
  const firstFocusable = focusableElements[0];
  const lastFocusable = focusableElements[focusableElements.length - 1];
  
  function handleKeydown(e: KeyboardEvent) {
    if (e.key !== 'Tab') return;
    
    if (e.shiftKey) {
      if (document.activeElement === firstFocusable) {
        e.preventDefault();
        lastFocusable.focus();
      }
    } else {
      if (document.activeElement === lastFocusable) {
        e.preventDefault();
        firstFocusable.focus();
      }
    }
  }
  
  element.addEventListener('keydown', handleKeydown);
  firstFocusable?.focus();
  
  return {
    destroy() {
      element.removeEventListener('keydown', handleKeydown);
    }
  };
}

// Announce to screen readers
export function announce(message: string, priority: 'polite' | 'assertive' = 'polite') {
  const announcer = document.createElement('div');
  announcer.setAttribute('aria-live', priority);
  announcer.setAttribute('aria-atomic', 'true');
  announcer.style.position = 'absolute';
  announcer.style.left = '-10000px';
  announcer.style.width = '1px';
  announcer.style.height = '1px';
  announcer.style.overflow = 'hidden';
  
  document.body.appendChild(announcer);
  announcer.textContent = message;
  
  setTimeout(() => {
    document.body.removeChild(announcer);
  }, 1000);
}

// Focus management
export function manageFocus() {
  let previousFocus: HTMLElement | null = null;
  
  return {
    save() {
      previousFocus = document.activeElement as HTMLElement;
    },
    restore() {
      if (previousFocus && previousFocus.focus) {
        previousFocus.focus();
      }
    }
  };
}

// Skip links for keyboard navigation
export function createSkipLink(targetId: string, text = 'Skip to main content'): HTMLAnchorElement {
  const link = document.createElement('a');
  link.href = `#${targetId}`;
  link.className = 'skip-link';
  link.textContent = text;
  link.addEventListener('click', (e) => {
    e.preventDefault();
    const target = document.getElementById(targetId);
    if (target) {
      target.focus();
      target.scrollIntoView();
    }
  });
  return link;
}