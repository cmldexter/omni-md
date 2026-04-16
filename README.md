<div align="center">
  <h1>🧬 Omni-MD</h1>
  <p><b>Bidirectional Documentation Synchronizer (GitLab & Wiki.js)</b></p>
</div>

---

Omni-MD is a lightweight, asynchronous monorepo explicitly developed to resolve the **"One-Way Markdown"** disconnect inside modern platform engineering teams. 

It seamlessly bridges the gap between software engineers keeping `docs/` tightly coupled to code within feature repos (e.g. Backstage TechDocs) and non-technical stakeholders reading documentation on an overarching central platform (e.g. Wiki.js).

## The Architecture
The platform is broken natively into two highly optimized Microservices communicating securely:

### 1. `md-relay-worker` (Backend)
An ultra-high performance **Rust web service** utilizing the `axum` and `tokio` asynchronous frameworks. 
* **Zero Git Locks:** Incoming Webhooks are instantaneously parsed and sent to a background `mpsc` queue for sequentially executed Git routing.
* **Persistent Tuning:** Stores real-time trace events in a fully WAL-optimized embedded `sqlite` datastore via `sqlx`.
* **Deep Security:** Compiled using a multi-stage Docker build that implicitly drops container execution privileges to a locked non-root user (`genome`).

### 2. `omni-console` (Frontend)
A beautifully designed, premium **Vite + React** dashboard mirroring the strict aesthetic identity tokens heavily utilized by standard developer portals like Backstage. It grants platform engineers the ability to:
* Determine repository configuration mapping endpoints.
* Visually troubleshoot and debug background queue synchronization histories natively ingested straight from the backend event loops.

### Directory Structure
```text
omni-md/
├── docker-compose.yml       # Production-ready orchestration binding Frontend/Backend
├── config.example.yml       # Wiki.js compliant configuration mapping template
├── md-relay-worker/         # [Backend] Rust Webhook Engine
│   ├── Dockerfile           # Multi-stage Debian container tuned for C-bindings
│   └── src/
│       ├── main.rs          # Axum HTTP server, Tokio MPSC queue, K8s health probes
│       ├── webhook.rs       # Agnostic parser for incoming GitHub/GitLab payloads
│       ├── config.rs        # YAML deserializer for syncing route configurations
│       ├── db.rs            # WAL-optimized SQLite thread-safe logging engine
│       └── git_engine.rs    # Dual-path Git Orchestrator (`git CLI` vs `git2` memory drafting)
└── omni-console/            # [Frontend] React Platform Dashboard
    ├── Dockerfile           # Highly optimized Nginx delivery container
    └── src/
        ├── index.css        # Premium Backstage-inspired CSS Design Tokens
        ├── App.tsx          # Core React Router and Application State
        ├── components/      # Reusable UI architecture (Sidebar, Headers)
        └── pages/           # Dedicated Dashboard Views (Config Map & Sync Logs)
```

---

## Bidirectional Safe-Syncing (The Draft Protocol)
We fundamentally do not allow non-technical writers operating directly inside the Wiki WYSIWYG editor to overwrite developer structural codebases causing unpredictable pipelines breaks.

Instead, Omni-MD utilizes a unique **"Draft Protocol"**:
1. **Developer to Wiki (Publish):** Uses rapid underlying `git clone --depth 1` CLI executions to port massive chunks of directories straight into the wiki folders natively.
2. **Wiki to Developer (Draft):** Uses pure C-Bound memory manipulations (`libgit2`) to detect a user mutating an existing markdown file in the Wiki. The engine reconstructs the raw string payload and pushes it cleanly backwards into the Source codebase as an isolated `.draft.md` file!

This allows an **AI Agent** sitting on the source code branch to instantly detect the `.draft.md`, perform conflict-comparisons, and elegantly merge the non-technical user feedback gracefully.

---

## Getting Started
Omni-MD features a production-ready container orchestration out of the box featuring isolated user privileges, interconnected `healthchecks`, and protected volume routing bindings.

```bash
docker-compose up -d --build
```
* **Omni-Console UI:** http://localhost:5173
* **Relay-Worker API:** http://localhost:8080