# Project Initialization Specification

**Spec ID:** PROJ-INIT-001  
**Sprint:** 1  
**Component:** Project Scaffolding  
**Status:** DRAFT

---

## Requirements

### Input Configuration
```typescript
interface ProjectConfig {
  name: string;              // alphanumeric + hyphens
  template: 'tauri' | 'svelte' | 'vite-react' | 'full-stack';
  backend: 'rust' | 'node';
  database: 'none' | 'sqlite' | 'postgres' | 'mongodb';
  styling: 'tailwind' | 'vanilla-css';
  packageManager: 'npm' | 'yarn' | 'pnpm';
}
```

### Output Structure
- Fully functional Tauri 2.8.5 project
- Rust 2024 Edition backend
- Svelte 5 + TypeScript frontend
- All tooling pre-configured
- Ready for first `cargo build` and `npm run tauri:dev`

### Critical Files Generated
1. Cargo.toml - Rust configuration
2. package.json - Node configuration
3. tauri.conf.json - Tauri settings with CSP enabled
4. vite.config.ts - Frontend build config
5. tsconfig.json - TypeScript strict mode
6. All required directories and boilerplate

### Dependencies & Prerequisites
- Node 18+ installed
- Rust toolchain with 2024 Edition
- cargo-tauri CLI available
- Git initialized

### Validation
- Project name follows Rust package naming rules
- All required tools available
- Sufficient disk space
- Valid configuration values

### Success Criteria
- All files created successfully
- Initial build succeeds: `cargo build`
- Frontend builds: `npm run build`
- No TypeScript errors
- All tooling initialized

---

**Version:** 1.0  
**Created:** October 27, 2025
