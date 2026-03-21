# Project Guidelines

## Stack

- **Backend**: Rust (Tauri 2, edition 2021)
- **Frontend**: TypeScript + Vue 3 + Tailwind CSS + Vite 6
- **Communication**: Tauri commands (`invoke`) between frontend and backend

## Code Style

### Rust
- Follow `rustfmt` and `clippy` conventions
- Use `#[tauri::command]` for all IPC handlers defined in `lib.rs`
- Prefer `thiserror` for error types; propagate errors with `?`
- Keep commands thin — business logic goes in separate modules, not in command handlers

### TypeScript / Vue
- Use `<script setup lang="ts">` (Composition API) in all components
- Keep components small and single-purpose
- Use `invoke<T>()` from `@tauri-apps/api/core` for backend calls — always type the return value
- Tailwind utility classes directly in templates; avoid custom CSS unless necessary

## Project Structure

```
src/                    # Vue frontend
  components/           # Reusable UI components
  App.vue               # Root component
  main.ts               # Entry point
src-tauri/
  src/
    lib.rs              # Tauri commands and app setup
    main.rs             # Binary entry point
  tauri.conf.json       # Window and app config
  capabilities/         # Permission definitions
```

## Development

```bash
npm run tauri dev    # Start dev server + Tauri window
npm run tauri build  # Production build
```

## Key Conventions

- Tauri command names use `snake_case` in Rust, `camelCase` in JS/TS invocations
- All new Tauri commands must be registered in the `.invoke_handler()` in `lib.rs`
- Avoid `unwrap()` in command handlers — return a `Result` with a meaningful error string
- Keep the window config in `tauri.conf.json`; avoid hardcoding dimensions or titles in code
