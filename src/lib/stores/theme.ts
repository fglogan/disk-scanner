import { writable, derived, type Readable } from "svelte/store";

// Theme types
export const THEMES = {
  LIGHT: 'light',
  DARK: 'dark',
  SYSTEM: 'system'
} as const;

type Theme = typeof THEMES[keyof typeof THEMES];
type AppliedTheme = 'light' | 'dark';

interface ThemeState {
  theme: Theme;
  systemPreference: AppliedTheme;
  appliedTheme: AppliedTheme;
}

interface ThemeStore extends Readable<ThemeState> {
  setTheme: (theme: Theme) => void;
  toggle: () => void;
  init: () => void;
  THEMES: typeof THEMES;
}

// Create enhanced theme store
function createThemeStore(): ThemeStore {
  const defaultTheme: Theme = THEMES.DARK;
  const { subscribe, set, update } = writable<ThemeState>({
    theme: defaultTheme,
    systemPreference: 'dark',
    appliedTheme: 'dark'
  });

  // Initialize theme
  function init() {
    if (typeof window === 'undefined') return;

    // Get saved preference
    const saved = (localStorage.getItem('theme') || defaultTheme) as Theme;
    
    // Get system preference
    const systemPreference: AppliedTheme = window.matchMedia('(prefers-color-scheme: dark)').matches 
      ? 'dark'
      : 'light';
    
    // Determine applied theme
    const appliedTheme: AppliedTheme = saved === THEMES.SYSTEM ? systemPreference : (saved as AppliedTheme);
    
    // Apply theme without transition on init
    document.documentElement.classList.add('theme-transitioning');
    document.documentElement.setAttribute('data-theme', appliedTheme);
    setTimeout(() => {
      document.documentElement.classList.remove('theme-transitioning');
    }, 50);
    
    set({
      theme: saved,
      systemPreference,
      appliedTheme
    });

    // Listen for system theme changes
    window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
      const newSystemPref: AppliedTheme = e.matches ? 'dark' : 'light';
      update(state => {
        if (state.theme === THEMES.SYSTEM) {
          applyTheme(newSystemPref);
          return { ...state, systemPreference: newSystemPref, appliedTheme: newSystemPref };
        }
        return { ...state, systemPreference: newSystemPref };
      });
    });
  }

  // Apply theme to DOM
  function applyTheme(theme: AppliedTheme) {
    if (typeof window === 'undefined') return;
    document.documentElement.setAttribute('data-theme', theme);
  }

  // Set theme
  function setTheme(theme: Theme) {
    if (!Object.values(THEMES).includes(theme)) return;

    update(state => {
      const appliedTheme: AppliedTheme = theme === THEMES.SYSTEM 
        ? state.systemPreference 
        : (theme as AppliedTheme);
      
      // Save to localStorage
      localStorage.setItem('theme', theme);
      
      // Apply to DOM with transition
      applyTheme(appliedTheme);
      
      return { ...state, theme, appliedTheme };
    });
  }

  // Toggle between light and dark
  function toggle() {
    update(state => {
      const newAppliedTheme: AppliedTheme = state.appliedTheme === 'dark' ? 'light' : 'dark';
      const newTheme: Theme = newAppliedTheme as Theme;
      
      localStorage.setItem('theme', newTheme);
      applyTheme(newAppliedTheme);
      
      return { ...state, theme: newTheme, appliedTheme: newAppliedTheme };
    });
  }

  // Initialize on mount
  if (typeof window !== 'undefined') {
    init();
  }

  return {
    subscribe,
    setTheme,
    toggle,
    init,
    THEMES
  };
}

export const themeStore = createThemeStore();

// Derived store for current applied theme
export const currentTheme: Readable<AppliedTheme> = derived(themeStore, $theme => $theme.appliedTheme);

// Derived store for isDark
export const isDarkMode: Readable<boolean> = derived(themeStore, $theme => $theme.appliedTheme === 'dark');