# Epic Worksession Workflow — Agent Reference

> **Purpose**: Follow this workflow when the user references a specific epic page from the project roadmap.
> This narrows your focus to ONLY the issues and files in that epic.
> Do NOT check all open issues — the epic page IS your issue list.
>
> **Trigger**: The user references this document + an epic's overview page:
> ```
> I want to work on @docs/epic-worksession-workflow.md using @roadmap/epic-01-app-mode-expansion/overview.md
> ```

---

## Local Development

> ⚠️ **Omni-MD uses `docker compose` for all local development and testing.** See `docs/local-dev-testing-workflow.md` for full details on starting, rebuilding, and resetting the dev environment.

```bash
# Quick start — full stack
docker compose up -d --build

# Omni-Console UI: http://localhost:5173
# Relay-Worker API: http://localhost:8080
```

---

## Step 1: Read the Epic Page

Read the epic page the user referenced. Extract:

- **Epic title** and scope description
- **Issues table** — these are the ONLY issues you work on this session
- **Implementation plan** — files to modify, approach, acceptance criteria
- **Dependencies** — check if prerequisite epics are marked ✅ Done on `roadmap.md`
- **Session checklist** — follow this as your task list
- **Session log** — review what was done in previous sessions (if any)

> ⚠️ The epic page is your **single source of truth**. Do not query all open issues, do not research from scratch. The epic already contains the scoped plan.

### Also Read (for context, not for scope):

```
Review /Users/ealastre/Documents/GitHub/gsx-wiki/projects/omni-md/overview.md
```

This gives you the full architecture context (tech stack, file tree, API surface). You need this to implement, but your **scope** comes from the epic page only.

---

## Step 2: Check Current State

For each issue in the epic's issues table:

```bash
# Check if issue is still open (GitHub Issues)
gh issue view {ID} --repo cmldexter/omni-md

# List all open issues
gh issue list --repo cmldexter/omni-md --state open
```

1. If already **resolved** → skip it, note it in your session summary
2. If **open** → read the full description for implementation context
3. If the API is unreachable → the containers may not be running; start them first:
   ```bash
   docker compose up -d --build
   ```

Check the epic page's status:
- If the epic is **⬜ Not Started** → update `roadmap.md` to **🔄 In Progress**
- If the epic is **🔄 In Progress** → review the session log for what was done previously

---

## Step 3: Plan This Session's Work

From the epic's implementation plan, pick a coherent subset for this session:

- **Prefer dependency order** — implement prerequisites before dependents
- **Aim for 2–4 issues per session** — quality over quantity
- **Tell the user your plan** — what you'll tackle now, what you'll defer to the next session

Create your implementation plan from the epic's pre-written plan — don't research from scratch. The epic page already has:
- Files to modify
- Approach & architecture decisions
- Acceptance criteria per issue

### Plan Template

```
This session I'll work on Epic N — [Title]:

1. Issue #X — [title] → [brief approach]
2. Issue #Y — [title] → [brief approach]

Deferred to next session:
3. Issue #Z — [title] (depends on #X being done first)
```

---

## Step 4: Execute → Commit → Resolve

For each issue, follow the standard cycle from `docs/issue-tracker-workflow.md`:

### 4a. Implement

Refer to the epic's **Implementation Plan** section for:
- Which files to modify
- The recommended approach
- Acceptance criteria to verify against

**Key files by issue type:**

| Type | Where to look |
|------|---------------|
| `feature` | `md-relay-worker/src/` (Rust handlers), `omni-console/src/pages/` (React pages) |
| `bug` | Check error details, look in `md-relay-worker/src/`, `omni-console/src/components/` |
| `sync` | `md-relay-worker/src/git_engine.rs`, `webhook.rs`, `config.rs` |
| `infra` | `docker-compose.yml`, `md-relay-worker/Dockerfile`, `omni-console/Dockerfile` |
| `ux` | `omni-console/src/index.css`, `omni-console/src/components/`, page TSX files |
| `performance` | `md-relay-worker/src/db.rs`, `git_engine.rs`, queue handling in `main.rs` |

### 4b. Apply Changes

> ⚠️ **Omni-MD uses `docker compose` for all local development and testing.** Rust backend changes require a container rebuild. See `docs/local-dev-testing-workflow.md`.

| Change Location | Hot-Reload? | Action Needed |
|----------------|-------------|---------------|
| `omni-console/src/**/*.tsx` | ❌ No (Nginx prod build) | **Rebuild**: `docker compose build omni-console && docker compose up -d omni-console` |
| `omni-console/src/**/*.css` | ❌ No (Nginx prod build) | **Rebuild**: `docker compose build omni-console && docker compose up -d omni-console` |
| `md-relay-worker/src/**/*.rs` | ❌ No (compiled Rust) | **Rebuild**: `docker compose build md-relay-worker && docker compose up -d md-relay-worker` |
| `docker-compose.yml` | N/A | **Recreate**: `docker compose up -d --build` |
| `config.example.yml` | N/A | Copy to `config.yml`, restart worker |

**Rebuild full stack** if multiple services were modified:
```bash
docker compose up -d --build
curl -s http://localhost:8080/api/health/liveness  # verify worker is back
```

### 4c. Commit

```bash
git add -A && git commit -m "feat: Issue #N — description"
```

### 4d. Resolve

```bash
# Close GitHub Issue
gh issue close N --repo cmldexter/omni-md -c "Resolved in commit {HASH}"
```

---

## Step 5: Update the Epic Page

After completing issues in this session, update the epic page in gsx-wiki:

### 5a. Mark Resolved Issues

In the epic's **Issues** table, change the status column:

```markdown
| ✅ | Add CORS middleware for cross-origin requests | `abc1234` |
```

### 5b. Update the Session Log

Add a row to the **Session Log** table at the bottom of the epic page:

```markdown
## Session Log

| Date | Issues Completed | Conversation | Notes |
|------|-----------------|--------------|-------|
| 2026-06-26 | #1, #2 | [Link to conversation] | File tree API + CORS middleware |
```

### 5c. Update Roadmap Status

Edit `/Users/ealastre/Documents/GitHub/gsx-wiki/projects/omni-md/roadmap.md`:

- If **all issues** in the epic are resolved → mark **✅ Done**
- If **some issues** remain → keep **🔄 In Progress**
- Update the progress fraction (e.g., `3/5 issues`)

---

## Step 6: Session Dev Blog (End of Session)

Create a dev blog entry **inside the epic's folder** (not in the general project dev blog directory). This keeps all epic-related documentation self-contained.

### Dev Blog Location

```
/Users/ealastre/Documents/GitHub/gsx-wiki/projects/omni-md/roadmap/epic-NN-<slug>/YYYY-MM-DD-<session-slug>.md
```

**Example:**
```
roadmap/epic-03-backend-wiring/2026-06-26-file-api-cors.md
```

### Frontmatter

```yaml
---
title: "Epic 3 Session — <Session Title>"
description: "<1-2 sentence description>"
published: true
date: YYYY-MM-DDT00:00:00.000Z
tags: "omni-md, epic-3, <relevant-tags>"
editor: markdown
dateCreated: YYYY-MM-DDT00:00:00.000Z
---
```

### Content Structure

Follow the same structure as `docs/wiki-update-workflow.md` Step 1, but scoped to this session's work:

```markdown
# Epic N Session — <Title>

**Date:** YYYY-MM-DD
**Epic:** [Epic N — <Title>](/projects/omni-md/roadmap/epic-NN-<slug>/overview)
**Issues Resolved:** #A, #B

---

## What Changed

### Issue #A — <Title>
<Description of the change, key code decisions, relevant code snippets>

| File | Change |
|------|--------|
| `md-relay-worker/src/file_api.rs` | Brief description |

### Issue #B — <Title>
...

---

## Verification Results
| Check | Result |
|-------|--------|
| <what you tested> | ✅ <result> |
```

> **When to use the general project dev blog instead:** If the work session was NOT triggered by this workflow (i.e., ad-hoc work, cross-epic work, or non-issue work), use the general project dev blog path from `docs/wiki-update-workflow.md`.

After creating the session dev blog, also follow `docs/wiki-update-workflow.md` Steps 2–3 to update the project overview and commit to gsx-wiki.

---

## Quick Reference

| What | Path |
|------|------|
| **Roadmap index** | `/Users/ealastre/Documents/GitHub/gsx-wiki/projects/omni-md/roadmap.md` |
| **Epic folders** | `/Users/ealastre/Documents/GitHub/gsx-wiki/projects/omni-md/roadmap/epic-NN-*/` |
| **Epic overview** | `roadmap/epic-NN-*/overview.md` (implementation plan + issues table) |
| **Epic dev blogs** | `roadmap/epic-NN-*/YYYY-MM-DD-<slug>.md` (session dev blogs) |
| **Project dev blogs** | `/Users/ealastre/Documents/GitHub/gsx-wiki/projects/omni-md/dev-blog/` |
| **Health API** | `http://localhost:8080/api/health/liveness` |
| **Sync Logs API** | `http://localhost:8080/api/logs` |
| **Webhook API** | `http://localhost:8080/webhook` |
| **Project overview** | `/Users/ealastre/Documents/GitHub/gsx-wiki/projects/omni-md/overview.md` |
| **Issue workflow** | `/Users/ealastre/Documents/GitHub/omni-md/docs/issue-tracker-workflow.md` |
| **Wiki workflow** | `/Users/ealastre/Documents/GitHub/omni-md/docs/wiki-update-workflow.md` |
| **Local dev workflow** | `/Users/ealastre/Documents/GitHub/omni-md/docs/local-dev-testing-workflow.md` |

---

## Related Docs

| Doc | Path | When to use |
|-----|------|-------------|
| **This Doc** | `docs/epic-worksession-workflow.md` | Start of every focused epic session |
| **Issue Tracker Workflow** | `docs/issue-tracker-workflow.md` | For ad-hoc issue work outside epics |
| **Wiki Update Workflow** | `docs/wiki-update-workflow.md` | End of session — document what you built |
| **Local Dev Testing** | `docs/local-dev-testing-workflow.md` | Docker Compose dev environment reference |
| **Dev Database Workflow** | `docs/dev-database-workflow.md` | When resetting or inspecting the SQLite dev DB |
| **Roadmap** | `gsx-wiki/projects/omni-md/roadmap.md` | See all epics and their status |
| **Project Overview** | `gsx-wiki/projects/omni-md/overview.md` | Full architecture, API surface, tech stack |
