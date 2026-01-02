# Session 2 Task Brief

## TL;DR
Implement Registry to store plugin metadata, wire it into desktop app, expose via Tauri commands.

## Steps
1. **Registry** - Create `backend/plugin-manager/src/registry.rs` with thread-safe storage
2. **Desktop Integration** - Replace old plugin_manager with new crate
3. **Tauri Commands** - Expose registry data to frontend

## Quick Start
```bash
# Verify current state compiles
cargo check -p cognexus-plugin-manager

# Start session with
"Let's implement the Registry (Step 6)"
```

See `NEXT_SESSION_PROMPT.md` for full details.
