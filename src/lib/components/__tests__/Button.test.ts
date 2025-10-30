import { render, screen, fireEvent } from '@testing-library/svelte';
import { describe, it, expect, vi } from 'vitest';
import Button from '../ui/Button.svelte';

describe('Button Component', () => {
  it('renders with default props', () => {
    render(Button, { 
      props: { 
        children: () => 'Click me' 
      } 
    });
    
    const button = screen.getByRole('button');
    expect(button).toBeInTheDocument();
    expect(button).toHaveTextContent('Click me');
    expect(button).toHaveClass('bg-indigo-600'); // primary variant
  });

  it('applies variant classes correctly', () => {
    render(Button, { 
      props: { 
        variant: 'danger',
        children: () => 'Delete' 
      } 
    });
    
    const button = screen.getByRole('button');
    expect(button).toHaveClass('bg-red-600');
  });

  it('handles click events', async () => {
    const handleClick = vi.fn();
    
    render(Button, { 
      props: { 
        onclick: handleClick,
        children: () => 'Click me' 
      } 
    });
    
    const button = screen.getByRole('button');
    await fireEvent.click(button);
    
    expect(handleClick).toHaveBeenCalledTimes(1);
  });

  it('shows loading state', () => {
    render(Button, { 
      props: { 
        loading: true,
        children: () => 'Submit' 
      } 
    });
    
    const button = screen.getByRole('button');
    expect(button).toHaveTextContent('Loading...');
    expect(button).toHaveAttribute('aria-busy', 'true');
  });

  it('is disabled when loading', async () => {
    const handleClick = vi.fn();
    
    render(Button, { 
      props: { 
        loading: true,
        onclick: handleClick,
        children: () => 'Submit' 
      } 
    });
    
    const button = screen.getByRole('button');
    expect(button).toBeDisabled();
    
    await fireEvent.click(button);
    expect(handleClick).not.toHaveBeenCalled();
  });

  it('handles keyboard navigation', async () => {
    const handleClick = vi.fn();
    
    render(Button, { 
      props: { 
        onclick: handleClick,
        children: () => 'Click me' 
      } 
    });
    
    const button = screen.getByRole('button');
    
    // Test Enter key
    await fireEvent.keyDown(button, { key: 'Enter' });
    expect(handleClick).toHaveBeenCalledTimes(1);
    
    // Test Space key
    await fireEvent.keyDown(button, { key: ' ' });
    expect(handleClick).toHaveBeenCalledTimes(2);
  });

  it('applies custom classes', () => {
    render(Button, { 
      props: { 
        class: 'custom-class',
        children: () => 'Custom' 
      } 
    });
    
    const button = screen.getByRole('button');
    expect(button).toHaveClass('custom-class');
  });

  it('renders with icon', () => {
    render(Button, { 
      props: { 
        icon: '🔍',
        iconPosition: 'left',
        children: () => 'Search' 
      } 
    });
    
    const button = screen.getByRole('button');
    expect(button).toHaveTextContent('🔍');
    expect(button).toHaveTextContent('Search');
  });
});