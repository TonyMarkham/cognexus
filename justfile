# Constants
WASM_TARGET := "wasm32-wasip1"

# Build types component (debug)
build-types-debug:
    cargo component build -p cognexus-types
    mkdir -p target/debug/resources/builtin
    cp target/{{WASM_TARGET}}/debug/cognexus_types.wasm target/debug/resources/builtin/

# Build nodes component (debug)
build-nodes-debug:
    cargo component build -p cognexus-nodes
    mkdir -p target/debug/resources/builtin
    cp target/{{WASM_TARGET}}/debug/cognexus_nodes.wasm target/debug/resources/builtin/

# Build types component (release)
build-types-release:
    cargo component build --release -p cognexus-types
    mkdir -p target/release/resources/builtin
    cp target/{{WASM_TARGET}}/release/cognexus_types.wasm target/release/resources/builtin/

# Build nodes component (release)
build-nodes-release:
    cargo component build --release -p cognexus-nodes
    mkdir -p target/release/resources/builtin
    cp target/{{WASM_TARGET}}/release/cognexus_nodes.wasm target/release/resources/builtin/

# Build both WASM components (debug)
build-wasm-debug: build-types-debug build-nodes-debug

# Build both WASM components (release)
build-wasm-release: build-types-release build-nodes-release

# Development mode
dev: build-wasm-debug
    cd apps/desktop/cognexus && cargo tauri dev

# Build desktop app (release)
build-desktop:
    cd apps/desktop/cognexus && cargo tauri build

# Build everything (release)
build-all: build-wasm-release build-desktop