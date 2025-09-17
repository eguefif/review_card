# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Tauri desktop application for creating AI-powered review cards. The app features a React frontend that communicates with a Rust backend to generate content via Anthropic's Claude API.

## Development Commands

### Frontend Development

```bash
pnpm dev          # Start development server (runs Vite on localhost:1420)
pnpm build        # Build for production (TypeScript compilation + Vite build)
pnpm preview      # Preview production build
```

### Tauri Development

```bash
pnpm tauri dev    # Start Tauri development mode with hot reload
pnpm tauri build  # Build Tauri application for distribution
```

### Dependencies

```bash
pnpm install      # Install all dependencies (frontend + Rust via Tauri)
```

## Architecture

### Frontend Structure

- **Entry point**: `src/main.tsx` renders the React app
- **Main component**: `src/App.tsx` renders the `CardCrafter` component
- **Components**:
  - `src/components/create/CardCrafter.tsx` - Main UI for creating review cards with AI
  - `src/components/reusables/` - Shared/reusable components
- **Styling**: Uses Emotion styled-components with CSS-in-JS approach

### Backend Structure (Rust/Tauri)

- **Entry point**: `src-tauri/src/main.rs` initializes Tauri app
- **Core module**: `src-tauri/src/lib.rs` sets up invoke handlers and plugins
- **AI integration**: `src-tauri/src/ai_app/prompt.rs` contains Claude API communication
- **Dependencies**: Uses `reqwest` for HTTP requests, `serde` for JSON serialization

### Key Technologies

- **Tauri 2.x**: Desktop application framework bridging Rust backend with web frontend
- **React 19**: Frontend framework with TypeScript
- **Vite**: Build tool and development server with HMR
- **Emotion**: CSS-in-JS styling library for styled-components
- **TypeScript**: Strict mode with ES2020 target and bundler module resolution
- **Rust**: Backend with `reqwest`, `serde`, and `dotenv` dependencies

### Communication Flow

1. Frontend form in `CardCrafter.tsx` captures user input
2. Tauri's `invoke()` API calls Rust backend via `prompt_ai` command
3. Rust backend (`ai_app/prompt.rs`) makes HTTP request to Claude API
4. Response is serialized and returned to frontend for display

### Configuration

- **Vite**: Fixed port 1420 for Tauri compatibility, ignores `src-tauri` directory
- **TypeScript**: Strict linting with unused parameter/variable detection
- **Tauri**: 800x600 default window, CSP disabled for development
- **Environment**: Uses `.env` file for API configuration (not committed)

### Important Notes

- API key is currently hardcoded in `prompt.rs` - should use environment variables
- Frontend uses `invoke('prompt_ai')` to communicate with Rust backend
- Tauri commands are registered in `lib.rs` via `generate_handler!` macro
