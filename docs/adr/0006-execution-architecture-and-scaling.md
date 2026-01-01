# ADR-0006: Execution Architecture: Standalone Runner & Sidecar Pattern

**Status:** Accepted  
**Date:** 2024-12-31  
**Deciders:** Tony  
**Related:** ADR-0005 (Plugin Discovery)

## Context

Cognexus requires a robust execution engine that works identically on a User's Desktop and a Cloud Server. Previous considerations involved running execution logic inside the main process or via library calls. However, this introduces risks:
1.  **Stability:** A plugin crash could take down the entire Desktop App or Web Server.
2.  **Coupling:** It tempts developers to share mutable state between UI and Execution, leading to race conditions.
3.  **Parity:** Running "In-Process" on Desktop vs "Remote Worker" on Web creates two different code paths to maintain.

**The Goal:** A unified execution architecture that offers **Process Isolation** and **Identical Behavior** across all platforms.

## Decision

### 1. The `cognexus-runner` CLI (Standalone Binary)

We will implement the execution engine as a completely standalone CLI binary, `cognexus-runner`.

*   **Input:** Receives a serialized `Graph` and `Inputs` (JSON or Protobuf) via **Stdin** (or file path).
*   **Configuration:** Accepts a path to the `plugins/` directory via CLI argument.
*   **Process:**
    1.  Loads necessary plugins using `backend/plugin-manager` (Native/Wasmtime).
    2.  Builds the execution graph.
    3.  Runs the flow.
*   **Output:** Emits execution events (Node Started, Node Finished, Error, Log) as a stream of Protobuf messages via **Stdout**.

### 2. Desktop Strategy: The Sidecar Pattern

On Desktop, we use Tauri's **Sidecar** capability.

*   **Architecture:** The Tauri App (Parent) spawns `cognexus-runner` (Child) when the user clicks "Run".
*   **Isolation:** If the Runner crashes (e.g., bad WASM, OOM), the UI remains responsive and simply reports "Execution Failed".
*   **State:** The Tauri App serializes its current Graph State and pipes it to the child process.

### 3. Web Strategy: Client-Authoritative & Remote Worker

On the Web, the architecture shifts to a **Client-Authoritative** model for authoring, preserving the stateless nature of the server.

*   **Authoring:** The Browser (WASM) holds the Master Graph State. All editing is local (zero latency).
*   **Execution:**
    1.  Browser serializes the Graph.
    2.  POSTs payload to Axum Server (`/api/run`).
    3.  Axum Server dispatches payload to a **Worker Node**.
    4.  Worker Node runs `cognexus-runner` (same binary as Desktop).
    5.  Axum streams the Stdout response back to the Browser (WebSocket/SSE).

### 4. Communication Protocol

We will use **Protobuf** over Stdio for reliable machine-to-machine communication.

*   **Input (Stdin):** `ExecutionRequest` (Graph, Inputs, Config).
*   **Output (Stdout):** Stream of `ExecutionEvent` (NodeID, Status, OutputData).

## Consequences

### Positive

1.  **Crash Proof:** The main application (UI/Server) is immune to execution failures.
2.  **Perfect Parity:** The exact same binary (`cognexus-runner`) executes the logic on macOS, Windows, and Linux Servers. Debugging is identical everywhere.
3.  **Simplicity:** The Runner is a "Pure Function" at the process level (Input -> Output). It has no knowledge of UI, WebViews, Databases, or Users.
4.  **Language Agnostic:** If we ever needed to, we could write a runner in another language (e.g., Python for AI heavy lifting) and swap it in, as long as it speaks the Stdio protocol.

### Negative

1.  **Startup Latency:** Spawning a new process for every execution adds overhead (10-50ms).
    *   *Mitigation:* For long-running workflows, this is negligible. For high-frequency loops, we can implement a "Daemon Mode" for the runner later.
2.  **Binary Size:** We must bundle the `cognexus-runner` binary with the Desktop App.

## Implementation Plan

1.  **Create Crate:** `apps/runner` (The CLI binary).
2.  **Define Protocol:** Create `execution.proto` (Request/Event definitions).
3.  **Implement CLI:** Build the main loop (Read Stdin -> Parse -> Execute -> Write Stdout).
4.  **Integrate Desktop:** Configure `tauri.conf.json` to bundle the sidecar. Implement `Command::sidecar` invocation in Tauri.
5.  **Integrate Web:** Implement the Axum handler to accept the payload and spawn the runner.

## References

- **Tauri Sidecar Guide:** https://tauri.app/v1/guides/building/sidecar
- **The Twelve-Factor App (Processes):** Execute the app as one or more stateless processes.
