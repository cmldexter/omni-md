# Issue Tracker Workflow — Agent Reference

> **Purpose**: This document teaches agents how to work with Omni-MD issues.
> Read this at the start of a conversation when the user asks you to check or implement feature requests or bugs.
>
> ⚠️ **Omni-MD currently uses GitHub Issues exclusively.** A built-in issue tracker API is planned as **Epic 4** on the [roadmap](/projects/omni-md/roadmap). Once implemented, this doc will be updated with dual-tracking instructions (built-in API + GitHub Issues).

---

## Local Development

> ⚠️ **Omni-MD uses `docker compose` for all local development and testing.** See `docs/local-dev-testing-workflow.md` for full details.

```bash
# Quick start — full stack
docker compose up -d --build

# Omni-Console UI: http://localhost:5173
# Relay-Worker API: http://localhost:8080
```

---

## How to Start a Conversation

When a new conversation begins about Omni-MD issue work, follow this sequence:

### Step 1: Get Context

Read the project overview to understand the full architecture:

```
Review /Users/ealastre/Documents/GitHub/gsx-wiki/projects/omni-md/overview.md
```

Key things to absorb:
- **Architecture**: Rust backend (Axum/Tokio) + React frontend (Vite) + SQLite (WAL-optimized)
- **Backend (md-relay-worker)**: Webhook parsing, Git engine (CLI + libgit2), MPSC job queue, sync event logging
- **Frontend (omni-console)**: Dashboard for config mappings + sync history. Backstage-inspired dark theme.
- **Docker Compose**: Full-stack orchestration (`omni-console` on `:5173`, `md-relay-worker` on `:8080`)
- **Draft Protocol**: Bidirectional sync — publish (git CLI) and draft (.draft.md sidecar via libgit2)
- **Branding**: OmniMD 🧬

### Step 2: Check Open Issues

```bash
# All open issues
gh issue list --repo cmldexter/omni-md --state open

# Only feature requests
gh issue list --repo cmldexter/omni-md --state open --label feature

# View a specific issue
gh issue view {ID} --repo cmldexter/omni-md
```

### Step 2.5: Check the Roadmap

Before building a plan from scratch, check if the issue belongs to a **roadmap epic**:

```
Review /Users/ealastre/Documents/GitHub/gsx-wiki/projects/omni-md/roadmap.md
```

If the issue is covered by an epic:
- **Switch to the epic worksession workflow** instead: `docs/epic-worksession-workflow.md`
- The epic page already has a pre-written implementation plan with files, approach, and acceptance criteria
- This saves time — don't re-research what's already scoped

If the issue is **not** in any epic (e.g., a new issue filed after the roadmap was created), continue with Step 3 below.

### Step 3: Build an Implementation Plan

For each open issue, assess:
1. **Complexity** — Is it a quick fix or a multi-file feature?
2. **Dependencies** — Does it need a backend change, frontend change, or both?
3. **Type** — Feature, bug, sync, infra, UX, or performance?

Then create a plan:
```
I want to create an implementation plan to:
1. Implement Issue #N — [title] ([type], [severity])
   - Files to modify: ...
   - Approach: ...
2. Implement Issue #M — [title] ([type], [severity])
   - Files to modify: ...
   - Approach: ...
```

### Step 4: Execute → Commit → Resolve

For each issue:
1. **Implement** the change
2. **Apply changes** — Omni-MD uses Docker Compose (see `docs/local-dev-testing-workflow.md`):

   | Change Location | Hot-Reload? | Action Needed |
   |----------------|-------------|---------------|
   | `omni-console/src/**/*.tsx` | ❌ No (Nginx prod build) | **Rebuild**: `docker compose build omni-console && docker compose up -d omni-console` |
   | `omni-console/src/**/*.css` | ❌ No (Nginx prod build) | **Rebuild**: `docker compose build omni-console && docker compose up -d omni-console` |
   | `md-relay-worker/src/**/*.rs` | ❌ No (compiled Rust) | **Rebuild**: `docker compose build md-relay-worker && docker compose up -d md-relay-worker` |
   | `docker-compose.yml` | N/A | **Recreate**: `docker compose up -d --build` |

   **Rebuild full stack** if multiple services were modified:
   ```bash
   docker compose up -d --build
   curl -s http://localhost:8080/api/health/liveness  # verify worker is back
   ```

3. **Commit**: `git add -A && git commit -m "feat: Issue #N — description"`
4. **Close GitHub Issue**:
   ```bash
   gh issue close N --repo cmldexter/omni-md -c "Resolved in commit {HASH}"
   ```

### Related Docs

| Doc | Path | What it covers |
|-----|------|----------------|
| **Project Overview** | `gsx-wiki/projects/omni-md/overview.md` | Full architecture, API surface, tech stack |
| **This Doc** | `docs/issue-tracker-workflow.md` | Issue workflow, types, resolve flow |
| **Wiki Update Workflow** | `docs/wiki-update-workflow.md` | How to document completed work in gsx-wiki (run after implementing) |
| **Epic Worksession Workflow** | `docs/epic-worksession-workflow.md` | Focused work sessions on roadmap epics (use instead of this doc when working on an epic) |
| **Local Dev Testing** | `docs/local-dev-testing-workflow.md` | Docker Compose dev environment reference |
| **Project Roadmap** | `gsx-wiki/projects/omni-md/roadmap.md` | All epics with status tracking |

---

## Quick Start: The Issue Loop

```
User reports issue → Agent files issue → Agent/Dev implements → Issue resolved
```

### 1. Check Open Issues

```bash
# All open issues
gh issue list --repo cmldexter/omni-md --state open

# Only feature requests
gh issue list --repo cmldexter/omni-md --state open --label feature

# Only bugs
gh issue list --repo cmldexter/omni-md --state open --label bug
```

### 2. Implement the Feature/Fix

For each issue, read its `title`, `body`, and `labels` for context.

**Key files by issue type:**

| Type | Where to look |
|------|---------------|
| `feature` | `md-relay-worker/src/` (Rust handlers), `omni-console/src/pages/` (React pages) |
| `bug` | Check error details, look in `md-relay-worker/src/`, `omni-console/src/components/` |
| `sync` | `md-relay-worker/src/git_engine.rs`, `webhook.rs`, `config.rs` |
| `infra` | `docker-compose.yml`, `md-relay-worker/Dockerfile`, `omni-console/Dockerfile` |
| `ux` | `omni-console/src/index.css`, `omni-console/src/components/`, page TSX files |
| `performance` | `md-relay-worker/src/db.rs`, `git_engine.rs`, queue handling in `main.rs` |

### 3. Resolve the Issue

After implementing and committing, close the GitHub Issue:

```bash
gh issue close {ID} --repo cmldexter/omni-md -c "Resolved in commit {HASH} — brief description"
```

---

## Issue Types

Omni-MD uses GitHub Issue labels to categorize work:

| Type | Label | Icon | Description |
|------|-------|------|-------------|
| `bug` | `bug` | 🐛 | Something broken |
| `feature` | `feature` | ✨ | New feature request |
| `sync` | `sync` | 🔄 | Git sync / webhook / Draft Protocol issue |
| `infra` | `infra` | 🏗️ | Infrastructure (Docker, Dockerfile, compose) |
| `ux` | `ux` | 🎨 | UI/UX improvement (omni-console) |
| `performance` | `performance` | ⚡ | Performance optimization |

### Severity Labels

| Severity | When to use |
|----------|-------------|
| `low` | Nice-to-have |
| `medium` | Default for features |
| `high` | Impacting workflow |
| `critical` | Blocking work |

---

## Filing New Issues

When an agent discovers a bug or the user requests a feature, file it on GitHub:

```bash
gh issue create --repo cmldexter/omni-md \
  --title "Add batch webhook processing" \
  --body "Description of the feature or bug..." \
  --label "feature" --label "medium"
```

> **Note:** Once Epic 4 (Built-In Issue Tracker) is complete, agents will also be able to create issues via the REST API (`POST /api/issues`). Until then, use `gh` CLI.

---

## Server Management

> ⚠️ **All local development uses Docker Compose.** See `docs/local-dev-testing-workflow.md` for the full reference.

### Quick Commands

```bash
# Start everything
docker compose up -d --build

# Check status
docker compose ps

# View logs
docker compose logs -f md-relay-worker
docker compose logs -f omni-console

# Health check
curl -s http://localhost:8080/api/health/liveness

# Rebuild after code changes
docker compose build md-relay-worker && docker compose up -d md-relay-worker

# Full reset (wipe DB)
docker compose down -v && docker compose up -d --build
```

> ⚠️ **Never start the server as a conversation background task.** Use a terminal tab or `nohup`. Background tasks started by agents are killed when the conversation ends.

---

## Example: Full Issue Cycle

```bash
# 1. Check open issues
gh issue list --repo cmldexter/omni-md --state open

# 2. Read issue #1
gh issue view 1 --repo cmldexter/omni-md
# → title: "Add file tree endpoint for workspace browsing"
# → labels: feature, high

# 3. Implement the feature
# ... edit files, rebuild containers ...
docker compose build md-relay-worker && docker compose up -d md-relay-worker

# 4. Commit
git add -A && git commit -m "feat: Issue #1 — Add file tree endpoint" && git push

# 5. Close GitHub Issue
gh issue close 1 --repo cmldexter/omni-md -c "Resolved in commit abc123"
```

---

## Related Docs

| Doc | Path | When to use |
|-----|------|-------------|
| **This Doc** | `docs/issue-tracker-workflow.md` | Start of issue work conversations |
| **Epic Worksession Workflow** | `docs/epic-worksession-workflow.md` | Focused work sessions on roadmap epics |
| **Wiki Update Workflow** | `docs/wiki-update-workflow.md` | End of session — document what you built |
| **Local Dev Testing** | `docs/local-dev-testing-workflow.md` | Docker Compose dev environment reference |
| **Dev Database Workflow** | `docs/dev-database-workflow.md` | When resetting or inspecting the SQLite dev DB |
| **Roadmap** | `gsx-wiki/projects/omni-md/roadmap.md` | See all epics and their status |
| **Project Overview** | `gsx-wiki/projects/omni-md/overview.md` | Full architecture, API surface, tech stack |
