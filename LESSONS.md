# Lessons Learned

## Project Setup & Tooling

### Manual Setup vs Scaffolding Tools

**Decision:** We manually set up Tauri instead of using `create-tauri-app`

**Why it matters:**
- Scaffolding tools (`create-tauri-app`, `create-react-app`, etc.) are opinionated about structure
- Their defaults assume a specific project layout that may not fit your needs
- You end up spending time fighting their conventions, moving files, and updating paths

**What we gained by doing it manually:**
- Full control over directory structure (our Blazor frontend lives in `frontend/cognexus/`, not where Tauri expects)
- Clean Rust workspace organization with proper separation of concerns
- Understanding of every piece - no magic or hidden configuration
- No unnecessary boilerplate or example code to delete
- Avoided JavaScript build tool configuration (webpack, vite, etc.) that comes with JS templates

**What the scaffolding tool actually does:**
- `create-tauri-app` is just a code generator - it creates files and directories
- The actual necessary dependencies are just `tauri` (runtime) and `tauri-build` (build system)
- Everything else is convenience/boilerplate

**The pattern:**
1. Understand what the tool generates (read docs, look at examples)
2. Identify what's actually required vs. what's convention
3. Build the minimal required structure yourself
4. Maintain full control over your architecture

**Trade-off:**
- Manual setup requires more knowledge upfront
- Scaffolding is faster for standard/conventional projects
- But for custom architectures (like our Rust workspace with Blazor), manual wins

### Cargo Workspace Dependency Management

**Decision:** Centralize all dependency versions in root `Cargo.toml` with `[workspace.dependencies]`

**Why it matters:**
- Version consistency across all workspace members
- Single source of truth for dependency versions
- Easier to update dependencies (change in one place)
- Prevents version conflicts between workspace members

**How it works:**
```toml
# Root Cargo.toml
[workspace.dependencies]
tauri = { version = "2.9.5", features = [] }
serde = { version = "1.0.228", features = ["derive"] }

# Workspace member Cargo.toml
[dependencies]
tauri = { workspace = true }
serde = { workspace = true }
```

**This is production-grade practice, not toy code.**

### Rust Naming Conventions

**The quirk:** Package names use hyphens, but Rust code uses underscores

**Example:**
- `Cargo.toml`: `name = "cognexus-model"`
- Rust code: `use cognexus_model::`

**Why:** Cargo automatically converts hyphens to underscores for identifiers (Rust doesn't allow hyphens in identifiers)

**Best practice:** Use hyphens in package names (idiomatic Rust, Clippy-friendly), accept the mental translation in code

### Tauri Project Structure

**Standard Tauri convention:**
```
project/
├── frontend files
└── src-tauri/      # Tauri Rust project here
```

**Our custom structure:**
```
cognexus/
├── backend/
│   ├── model/
│   └── renderer/
├── frontend/
│   └── cognexus/   # Blazor here
└── apps/
    └── desktop/
        └── cognexus/   # Tauri here (no src-tauri nesting)
```

**Key insight:** Tauri's documentation assumes you're following their convention, but you can structure it however you want. Just configure `tauri.conf.json` with the correct paths:
- `build.devUrl`: Where your dev server runs
- `build.frontendDist`: Where compiled frontend assets live

**From the docs:** "If you want to work with Rust code only, simply remove everything else and use the `src-tauri/` folder as your top level project or as a member of your Rust workspace"

We chose workspace member approach for better organization.

## JavaScript in Tauri + Blazor

**Reality check:** There IS JavaScript involved, but it's minimal and hidden:
1. Blazor WebAssembly uses a small JS bootstrap to initialize the WASM runtime
2. Tauri's IPC bridge uses JavaScript under the hood for webview ↔ Rust communication

**BUT:** We don't write, maintain, or deal with JavaScript directly. It's infrastructure code provided by the frameworks.

**Our code:**
- ✅ 100% C# (Blazor UI)
- ✅ 100% Rust (Tauri wrapper, rendering engine)
- ❌ 0% JavaScript written by us

**No JS ecosystem baggage:**
- No npm/package.json/node_modules
- No webpack/vite/rollup configuration
- No build tool dependency hell

**The JS is just plumbing** - like how you don't care that HTTP uses TCP/IP packets. It's there, but it's not your problem.

## Development Mindset

### Focus on Correctness, Not Speed

**Principle:** Doing things right is more important than doing things fast

**Why it matters:**
- Rushing leads to technical debt
- Bad patterns have to be unlearned later
- Taking time to understand prevents future pain
- Production-grade code from day one scales better

### Honesty Over Confidence

**Principle:** If you don't know something, be honest and research before answering - never guess

**Why it matters:**
- Confidently teaching the wrong thing is worse than admitting ignorance
- Bad information leads to wasted time and frustration
- Research builds deeper understanding
- Trustworthy guidance > fast answers

**Example from this project:** 
- Initially suggested `resolver = "2"` based on outdated knowledge
- User corrected with current information (resolver "3" exists in Cargo 1.91+)
- Better to check docs than guess based on old information

## What We Built

### Phase 1: Hello World in Tauri
- ✅ Basic Tauri window
- ✅ Rust workspace setup
- ✅ Production-grade dependency management
- ✅ Compiled and ran native code

### Phase 2: Blazor Integration
- ✅ Blazor dev server integration
- ✅ Blazor rendering in native Tauri window
- ✅ Full desktop application with web technologies
- ✅ Zero JavaScript written by us

**Outcome:** Cross-platform desktop application foundation with clean architecture, ready for node graph editor implementation.

## Next Steps

Future phases will involve:
- Protobuf communication layer between Blazor and Rust
- Node graph data model in Rust
- WGPU rendering engine
- Production build configuration
