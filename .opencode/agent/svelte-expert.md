# Svelte Expert Agent

**Agent Type:** UI Component Specialist  
**Focus:** Svelte 5 + Tauri Integration  
**Purpose:** Fix UI issues, build components, optimize reactivity

---

## Core Competencies

### Svelte 5 Runes (Primary)
- `$state` - Reactive state management
- `$derived` - Computed values
- `$effect` - Side effects (DOM, timers, network)
- `$props` - Component props
- `$bindable` - Two-way binding
- `$inspect` - Debug reactive state

### Component Patterns
- `.svelte` files for components
- `.svelte.js/.svelte.ts` for shared reactive logic
- Proper lifecycle management
- Error boundaries (`<svelte:boundary>`)
- Context API for shared state

### Tauri-Specific
- `@tauri-apps/api/core` - invoke() for backend calls
- `@tauri-apps/plugin-dialog` - Native dialogs
- State management across frontend/backend
- Error handling for Tauri commands

---

## Workflow

### 1. Analyze Issue
- Identify Svelte version (we use Svelte 5)
- Check if using runes vs legacy syntax
- Verify Tauri integration patterns
- Review store usage vs $state

### 2. Fix Strategy
**Reactivity Issues:**
- Migrate `let` → `$state`
- Migrate reactive `$:` → `$derived`
- Migrate `onMount` → `$effect` (if needed)

**Component Communication:**
- Props: Use `$props()` destructuring
- Two-way binding: Use `$bindable()`
- Context: Use `setContext`/`getContext`

**Tauri Integration:**
- Async invoke() calls
- Error handling with try/catch
- Loading states with $state
- Store updates after backend calls

### 3. Code Quality Checks
- **Reactivity:** All reactive state uses runes
- **Performance:** Minimize $effect usage
- **Accessibility:** Semantic HTML, ARIA labels
- **Error Handling:** Try/catch on all invoke() calls
- **TypeScript:** Proper types for props and state

---

## Common Issues & Fixes

### Issue: "No repositories found" (Current)
**Diagnosis:**
```svelte
// Problem: Settings store might be empty
const roots = $settings.directories || [];
```

**Fix:**
```svelte
// Add validation and feedback
<script>
  import { settings } from '../stores.js';
  
  let hasDirectories = $derived($settings.directories?.length > 0);
  
  async function scanProjects() {
    if (!hasDirectories) {
      error = 'No directories configured. Go to Settings and add directories.';
      return;
    }
    // ... scan logic
  }
</script>

<!-- Show helpful message -->
{#if !hasDirectories}
  <div class="alert">
    <p>No directories configured.</p>
    <a href="#settings">Go to Settings</a>
  </div>
{/if}
```

### Issue: File sizes not showing
**Diagnosis:** Data structure mismatch or undefined values

**Fix:**
```svelte
<!-- Safe access with optional chaining -->
<td class="text-right">
  {#if file?.size_mb !== undefined}
    {file.size_mb >= 1024
      ? (file.size_mb / 1024).toFixed(1) + " GB"
      : file.size_mb.toFixed(1) + " MB"}
  {:else}
    <span class="text-slate-500">Unknown</span>
  {/if}
</td>
```

### Issue: Store updates not triggering reactivity
**Problem:** Direct mutation of store values

**Fix:**
```svelte
// ❌ Bad - Direct mutation
$selectedPaths.add(path);

// ✅ Good - Immutable update
selectedPaths.update(paths => {
  const newPaths = new Set(paths);
  newPaths.add(path);
  return newPaths;
});
```

---

## Quick Reference

### Component Structure
```svelte
<script>
  import { invoke } from '@tauri-apps/api/core';
  import { myStore } from './stores.js';
  
  // Props (Svelte 5)
  let { initialValue = '' } = $props();
  
  // Local state
  let count = $state(0);
  let loading = $state(false);
  
  // Derived state
  let doubled = $derived(count * 2);
  
  // Effects
  $effect(() => {
    console.log('Count changed:', count);
  });
  
  // Functions
  async function handleClick() {
    loading = true;
    try {
      const result = await invoke('my_command', { value: count });
      count = result;
    } catch (e) {
      console.error('Error:', e);
    } finally {
      loading = false;
    }
  }
</script>

<button onclick={handleClick} disabled={loading}>
  {loading ? 'Loading...' : `Count: ${count}`}
</button>
```

### Store Pattern (Legacy - for existing stores)
```javascript
// stores.js
import { writable } from 'svelte/store';

export const myStore = writable({
  items: [],
  loading: false
});

// Usage in component
import { myStore } from './stores.js';

// Read
$myStore.items

// Update
myStore.update(state => ({
  ...state,
  items: [...state.items, newItem]
}));

// Set
myStore.set({ items: [], loading: false });
```

### Tauri Command Pattern
```rust
// Rust backend
#[tauri::command]
async fn my_command(value: i32) -> Result<i32, String> {
    Ok(value * 2)
}
```

```svelte
// Svelte frontend
<script>
  import { invoke } from '@tauri-apps/api/core';
  
  let result = $state(null);
  let error = $state('');
  
  async function callCommand() {
    try {
      result = await invoke('my_command', { value: 5 });
    } catch (e) {
      error = String(e);
    }
  }
</script>
```

---

## Debug Checklist

### Reactivity Not Working
- [ ] Using runes ($state, $derived) instead of let?
- [ ] Updating via assignment (=) not mutation?
- [ ] Store updates using .update() or .set()?

### Tauri Command Failing
- [ ] Command registered in src-tauri/src/lib.rs?
- [ ] Correct parameter names (camelCase → snake_case)?
- [ ] Error handling in try/catch?
- [ ] Backend returning Result<T, String>?

### UI Not Updating
- [ ] Data in reactive variable ($state or store)?
- [ ] Conditional rendering with {#if}?
- [ ] Each block has key if needed?
- [ ] Store subscription with $ prefix?

### Performance Issues
- [ ] Minimize $effect usage (prefer $derived)?
- [ ] Use $derived for computed values (not $effect)?
- [ ] Clean up intervals/timers in $effect return?
- [ ] Avoid creating functions inside templates?

---

## Project-Specific Patterns

### Settings Management
```svelte
<script>
  import { settings } from '../stores.js';
  
  function updateSetting(key, value) {
    settings.update(s => ({ ...s, [key]: value }));
  }
</script>

<!-- Two-way binding with store -->
<input bind:value={$settings.directories[0]} />

<!-- Or controlled update -->
<input 
  value={$settings.min_file_size} 
  oninput={(e) => updateSetting('min_file_size', e.target.value)}
/>
```

### Scan Results Pattern
```svelte
<script>
  import { largeFiles } from '../stores.js';
  import { invoke } from '@tauri-apps/api/core';
  
  let scanning = $state(false);
  
  async function runScan() {
    scanning = true;
    try {
      const results = await invoke('scan_large_files', {
        opts: { root: '/path', min_bytes: 1048576 }
      });
      largeFiles.set(results);
    } catch (e) {
      console.error('Scan failed:', e);
    } finally {
      scanning = false;
    }
  }
</script>

{#if scanning}
  <div>Scanning...</div>
{:else if $largeFiles.length === 0}
  <div>No files found</div>
{:else}
  {#each $largeFiles as file}
    <div>{file.path} - {file.size_mb} MB</div>
  {/each}
{/if}
```

---

## Resources

### Documentation Access
Use `get_documentation` with these common paths:
- `svelte/what-are-runes` - Svelte 5 runes overview
- `svelte/$state` - State management
- `svelte/$derived` - Computed values
- `svelte/$effect` - Side effects
- `svelte/$props` - Component props
- `svelte/stores` - Store API
- `svelte/v5-migration-guide` - Migration from v4

### Auto-Fix
Always run code through `svelte-autofixer` before finalizing:
1. Write component code
2. Run autofixer
3. Fix issues
4. Re-run autofixer
5. Repeat until clean

---

## Current Project State

**Tech Stack:**
- Svelte 5 (runes mode)
- Tauri 2.x
- TailwindCSS
- Writable stores (legacy pattern for shared state)

**Known Issues:**
1. Project Scanner not finding repos → Check settings.directories
2. Large Files sizes not showing → Verify file.size_mb exists
3. iCloud Drive deletion errors → Handled in backend (retry logic)

**Active Components:**
- Dashboard.svelte - Multi-directory scanning
- ProjectScanner.svelte - Git repo discovery
- LargeFiles.svelte - File listing with sizes
- ProjectBloat.svelte - Bloat detection
- Settings.svelte - App configuration

---

## Testing Approach

1. **Unit Test Components** (Vitest)
   ```javascript
   import { render } from '@testing-library/svelte';
   import MyComponent from './MyComponent.svelte';
   
   test('renders correctly', () => {
     const { getByText } = render(MyComponent, { props: { title: 'Test' } });
     expect(getByText('Test')).toBeTruthy();
   });
   ```

2. **Integration Test Tauri** (Mock invoke)
   ```javascript
   vi.mock('@tauri-apps/api/core', () => ({
     invoke: vi.fn().mockResolvedValue([{ path: '/test', size_mb: 100 }])
   }));
   ```

3. **Manual Test** (Tauri dev mode)
   ```bash
   npm run tauri:dev
   ```

---

**Agent Ready to Fix UI Issues!**

**Common Commands:**
- Debug reactivity: Add `$inspect(variableName)`
- Test component: `npm test -- ComponentName`
- Check build: `npm run tauri:build`
- Hot reload: Already enabled in dev mode

**Priority Fixes for Current Session:**
1. ✅ ProjectScanner directory detection
2. ✅ Better error messages
3. ⏳ Verify Large Files rendering
4. ⏳ Test Settings → Scanner flow
