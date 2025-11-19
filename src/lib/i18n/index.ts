// Simple i18n implementation without external dependencies
import { writable, derived, get } from 'svelte/store';
import en from './locales/en';

// Available locales
export const LOCALES = {
  EN: 'en',
  // Add more locales as needed
  // ES: 'es',
  // FR: 'fr',
} as const;

type Locale = typeof LOCALES[keyof typeof LOCALES];

// Translations type
type Translations = typeof en;

// Locale store
const locale = writable<Locale>(LOCALES.EN);

// Translations store
const translations = writable<Record<Locale, Translations>>({
  [LOCALES.EN]: en,
});

// Load locale from localStorage
export function initI18n() {
  if (typeof window !== 'undefined') {
    const saved = localStorage.getItem('locale') as Locale;
    if (saved && Object.values(LOCALES).includes(saved)) {
      locale.set(saved);
    }
  }
}

// Set locale
export function setLocale(newLocale: Locale) {
  locale.set(newLocale);
  if (typeof window !== 'undefined') {
    localStorage.setItem('locale', newLocale);
  }
}

// Get current locale
export function getLocale(): Locale {
  return get(locale);
}

// Translation function
export const t = derived(
  [locale, translations],
  ([$locale, $translations]) => {
    return (key: string, params?: Record<string, string | number>) => {
      const keys = key.split('.');
      let value: any = $translations[$locale];
      
      for (const k of keys) {
        value = value?.[k];
      }
      
      if (typeof value !== 'string') {
        console.warn(`Translation key not found: ${key}`);
        return key;
      }
      
      // Replace parameters
      if (params) {
        return value.replace(/\{(\w+)\}/g, (_, param) => {
          return String(params[param] || `{${param}}`);
        });
      }
      
      return value;
    };
  }
);

// Format functions
export const formatters = {
  number: (value: number, locale: Locale = getLocale()) => {
    return new Intl.NumberFormat(locale).format(value);
  },
  
  fileSize: (bytes: number) => {
    const units = ['B', 'KB', 'MB', 'GB', 'TB'];
    let size = bytes;
    let unitIndex = 0;
    
    while (size >= 1024 && unitIndex < units.length - 1) {
      size /= 1024;
      unitIndex++;
    }
    
    return `${size.toFixed(2)} ${units[unitIndex]}`;
  },
  
  date: (date: Date | string | number, locale: Locale = getLocale()) => {
    return new Intl.DateTimeFormat(locale, {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    }).format(new Date(date));
  },
  
  relativeTime: (date: Date | string | number, locale: Locale = getLocale()) => {
    const rtf = new Intl.RelativeTimeFormat(locale, { numeric: 'auto' });
    const diff = Date.now() - new Date(date).getTime();
    const seconds = Math.floor(diff / 1000);
    const minutes = Math.floor(seconds / 60);
    const hours = Math.floor(minutes / 60);
    const days = Math.floor(hours / 24);
    
    if (days > 0) return rtf.format(-days, 'day');
    if (hours > 0) return rtf.format(-hours, 'hour');
    if (minutes > 0) return rtf.format(-minutes, 'minute');
    return rtf.format(-seconds, 'second');
  }
};

// Export stores
export { locale };

// Initialize on import
initI18n();