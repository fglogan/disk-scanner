import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import tailwindcss from '@tailwindcss/vite';

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    svelte(),
    tailwindcss()
  ],
  
  // Performance optimizations
  build: {
    target: 'es2022',
    minify: 'terser',
    cssMinify: true,
    sourcemap: true,
    chunkSizeWarningLimit: 1000,
    rollupOptions: {
      output: {
        manualChunks: {
          // Vendor chunk for better caching
          vendor: ['svelte'],
          // Tauri API chunk
          tauri: ['@tauri-apps/api', '@tauri-apps/plugin-dialog'],
          // UI components chunk
          ui: [
            './src/lib/components/ui/Button.svelte',
            './src/lib/components/ui/Card.svelte',
            './src/lib/components/ui/Toast.svelte'
          ]
        },
        chunkFileNames: 'assets/[name]-[hash].js',
        entryFileNames: 'assets/[name]-[hash].js',
        assetFileNames: 'assets/[name]-[hash].[ext]'
      }
    },
    terserOptions: {
      compress: {
        drop_console: true,
        drop_debugger: true
      }
    }
  },

  // Development optimizations
  optimizeDeps: {
    include: ['@tauri-apps/api', '@tauri-apps/plugin-dialog']
  },

  // Enhanced CSS processing
  css: {
    preprocessorOptions: {
      scss: {
        additionalData: '@import "src/lib/design-tokens.css";'
      }
    }
  },

  // Server configuration for development
  server: {
    hmr: {
      overlay: false
    }
  }
});