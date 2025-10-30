<script lang="ts">
  import { darkMode, showSuccess, showWarning, showInfo, showShortcutsHelp } from '../../stores.js';

  function testToasts() {
    showSuccess('Theme toggle working!');
    setTimeout(() => showInfo('Toast system working!'), 500);
    setTimeout(() => showWarning('All features implemented!'), 1000);
  }

  function testKeyboardShortcuts() {
    showShortcutsHelp.set(true);
  }

  function testThemeToggle() {
    darkMode.toggle();
    showSuccess(`Switched to ${$darkMode ? 'dark' : 'light'} mode`);
  }
</script>

<div class="p-6 space-y-4">
  <h2 class="text-2xl font-bold">UI Quick Wins Test Panel</h2>
  
  <div class="space-y-2">
    <h3 class="text-lg font-semibold">Theme System</h3>
    <p class="text-sm text-gray-600 dark:text-gray-400">
      Current theme: {$darkMode ? 'Dark' : 'Light'} Mode
    </p>
    <button 
      class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors"
      on:click={testThemeToggle}
    >
      Toggle Theme
    </button>
  </div>

  <div class="space-y-2">
    <h3 class="text-lg font-semibold">Toast Notifications</h3>
    <button 
      class="px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600 transition-colors"
      on:click={testToasts}
    >
      Test Toast Messages
    </button>
  </div>

  <div class="space-y-2">
    <h3 class="text-lg font-semibold">Keyboard Shortcuts</h3>
    <p class="text-sm text-gray-600 dark:text-gray-400">
      Try: D (Dashboard), P (Projects), S (Settings), G (Git), C (Clear), ? (Help)
    </p>
    <button 
      class="px-4 py-2 bg-purple-500 text-white rounded hover:bg-purple-600 transition-colors"
      on:click={testKeyboardShortcuts}
    >
      Show Keyboard Help
    </button>
  </div>

  <div class="mt-6 p-4 bg-gray-100 dark:bg-gray-800 rounded">
    <h4 class="font-semibold mb-2">Implementation Status:</h4>
    <ul class="text-sm space-y-1">
      <li>✅ Theme toggle system (dark/light mode)</li>
      <li>✅ Toast notification system (4 types)</li>
      <li>✅ Keyboard shortcuts (d, p, s, c, g, ?)</li>
      <li>✅ Help overlay with all shortcuts</li>
      <li>✅ Theme persistence in localStorage</li>
      <li>✅ Responsive design compatibility</li>
    </ul>
  </div>
</div>