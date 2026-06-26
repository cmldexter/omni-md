# Wiki Update Workflow — Agent Reference

> **Purpose**: After implementing features or resolving issues in Omni-MD, follow this workflow to document the work in the gsx-wiki. This creates a permanent engineering record and keeps the project overview current.
>
> Run this workflow at the end of a feature session — after all code is committed and issues are resolved in Omni-MD.

---

## Paths

| What | Path |
|------|------|
| **Project dev blogs** | `/Users/ealastre/Documents/GitHub/gsx-wiki/projects/omni-md/dev-blog/` |
| **Epic dev blogs** | `/Users/ealastre/Documents/GitHub/gsx-wiki/projects/omni-md/roadmap/epic-NN-*/` |
| **General dev blogs** (for format reference) | `/Users/ealastre/Documents/GitHub/gsx-wiki/knowledge-base/dev-blog/` |
| **Project overview** | `/Users/ealastre/Documents/GitHub/gsx-wiki/projects/omni-md/overview.md` |
| **gsx-wiki repo root** | `/Users/ealastre/Documents/GitHub/gsx-wiki` |

---

## Step 0: Update Epic Page (If Working on an Epic)

If this session was part of a roadmap epic (i.e., you used `docs/epic-worksession-workflow.md`), update the epic **before** creating the dev blog:

1. **Mark resolved issues** in the epic's `overview.md` issues table (change ⬜ → ✅)
2. **Add a session log entry** with date, issues completed, and notes
3. **Update epic status** in `roadmap.md`:
   - All issues done → ✅ Done
   - Some remaining → 🔄 In Progress

Epic folders live at:
```
/Users/ealastre/Documents/GitHub/gsx-wiki/projects/omni-md/roadmap/epic-NN-<slug>/
├── overview.md          ← implementation plan + issues table
└── YYYY-MM-DD-<slug>.md ← session dev blogs (created by epic-worksession-workflow)
```

Roadmap index:
```
/Users/ealastre/Documents/GitHub/gsx-wiki/projects/omni-md/roadmap.md
```

> **Dev blog routing:** Epic session dev blogs go **inside the epic folder** (per `docs/epic-worksession-workflow.md` Step 6). General project dev blogs (ad-hoc work, non-epic) go in `dev-blog/` as described in Step 1 below.

If this session was **not** part of an epic, skip to Step 1.

---

## Step 1: Create a Dev Blog Entry

Create a new markdown file in the **project dev blog** directory (for non-epic work):

```
/Users/ealastre/Documents/GitHub/gsx-wiki/projects/omni-md/dev-blog/YYYY-MM-DD-<slug>.md
```

### Naming Convention

- **Date**: Use today's date in `YYYY-MM-DD` format
- **Slug**: Kebab-case summary of the feature, e.g. `file-api-cors-middleware`
- **Example**: `2026-06-26-file-api-cors-middleware.md`

### Frontmatter (Required)

Wiki.js requires YAML frontmatter. Copy this template exactly:

```yaml
---
title: "Part N — <Title>"
description: "<1-2 sentence description of what changed and why>"
published: true
date: YYYY-MM-DDT00:00:00.000Z
tags: "omni-md, <relevant-tags>"
editor: markdown
dateCreated: YYYY-MM-DDT00:00:00.000Z
---
```

**How to determine the Part number**: Count the existing dev blog entries in both the project directory AND the general dev blog directory that are Omni-MD-related. The latest Part is visible in the References section at the bottom of `overview.md`. Increment by 1.

### Content Structure

Follow this structure (omit sections that don't apply):

```markdown
# Part N — <Title>

**Date:** YYYY-MM-DD
**Context:** <1-2 sentences: what triggered this work, what problem it solves>

---

## The Problem / Motivation
<Background context — what was broken, what was requested, why it matters>

## What Changed

### 1. <Feature/Fix Name>
<Description of the change, key code decisions, relevant code snippets>

| File | Change |
|------|--------|
| `md-relay-worker/src/file_api.rs` | Brief description of change |

### 2. <Next Feature/Fix>
...

---

## Architecture
<If applicable: mermaid diagrams, data flow explanations, design decisions>

## Verification Results
| Check | Result |
|-------|--------|
| <what you tested> | ✅ <result> |

---

## Files Modified/Created
| File | Change |
|------|--------|
| `path/to/file` | [NEW] or description |

```

### Reference Material

If you need additional context on the writing style or depth expected, read existing entries:

- **Sibling project dev blogs** (for format reference): `ls /Users/ealastre/Documents/GitHub/gsx-wiki/projects/shrikestash/dev-blog/*.md`
- **Sibling project dev blogs** (strix-view): `ls /Users/ealastre/Documents/GitHub/gsx-wiki/projects/strix-view/ktbr-dev-blog/*.md`
- **General dev blogs**: `ls /Users/ealastre/Documents/GitHub/gsx-wiki/knowledge-base/dev-blog/*.md`

Read 1-2 recent entries to match the tone and level of detail. Key patterns:
- Include actual code snippets for non-trivial changes
- Use tables for file-change summaries
- Use mermaid diagrams for architecture explanations
- Include root cause analysis for bugs
- End with a "What Didn't Change" section if it's a focused fix (helps scope)

---

## Step 2: Update the Project Overview

Edit `/Users/ealastre/Documents/GitHub/gsx-wiki/projects/omni-md/overview.md` with three updates:

### 2a. Update the Status Line

The `**Status:**` line tracks completed phases. If this work represents a new phase, append it:

```markdown
**Status:** Active — ... Phase 1 complete (<Brief Description>), Phase 2 pending
```

If it's a minor addition within the current phase, you may skip this.

### 2b. Update the Completed ✅ Feature Table

Add new rows to the `### Completed ✅` table for each new feature or significant fix. Follow the existing format:

```markdown
| **<Component Name>** | <Brief description of what it does> | `<key files>` |
```

**Guidelines:**
- One row per distinct capability (not per commit)
- Key files should be the primary files a developer would look at
- Keep descriptions concise — aim for 1 line
- Group related features logically near existing similar rows

### 2c. Update the References Section (bottom of file)

Add a new dev blog reference entry:

```markdown
- **Dev Blog:** [Part N — <Title>](/projects/omni-md/dev-blog/YYYY-MM-DD-<slug>) — <Brief description>
```

Insert it before the `**Technical Docs:**` entries, maintaining chronological order.

### 2d. Update Architecture Sections (if applicable)

If the feature adds new API endpoints, update the **API Surface** section with new rows.

If the feature changes the file tree, update the **Architecture** section's directory trees.

---

## Step 3: Commit to gsx-wiki

```bash
cd /Users/ealastre/Documents/GitHub/gsx-wiki

# Stage the new blog post and overview changes
git add projects/omni-md/

# Commit with a descriptive message
git commit -m "blog(omni-md): Part N — <title>

- New dev blog: <slug>.md
- Updated overview: <brief list of what was updated>"

# Push
git push
```

### Commit Message Format

```
blog(omni-md): Part N — <Short Title>

- New dev blog: <filename>.md
- Updated overview: added <features> to completed table, updated references
```

---

## Quick Checklist

Use this checklist to verify completeness:

- [ ] Dev blog file created in `gsx-wiki/projects/omni-md/dev-blog/` (or epic folder)
- [ ] Frontmatter has correct `title`, `description`, `published: true`, `date`, `tags`
- [ ] Part number is correct (incremented from last)
- [ ] Blog content covers: context, what changed, files modified
- [ ] `overview.md` Status line updated (if new phase)
- [ ] `overview.md` Completed ✅ table has new rows for each feature
- [ ] `overview.md` References section has new dev blog link
- [ ] `overview.md` API Surface / Architecture updated (if applicable)
- [ ] Committed and pushed to gsx-wiki

---

## Related Docs

| Doc | Path | What it covers |
|-----|------|----------------|
| **Issue Tracker Workflow** | `docs/issue-tracker-workflow.md` | How to check/implement/resolve issues (run BEFORE this workflow) |
| **Epic Worksession Workflow** | `docs/epic-worksession-workflow.md` | Focused work sessions on roadmap epics (runs BEFORE this workflow) |
| **This Doc** | `docs/wiki-update-workflow.md` | How to document completed work in gsx-wiki (run AFTER implementing) |
| **Local Dev Testing** | `docs/local-dev-testing-workflow.md` | Docker Compose dev environment reference |
| **Project Overview** | `gsx-wiki/projects/omni-md/overview.md` | Full architecture, API surface, tech stack |
| **Project Roadmap** | `gsx-wiki/projects/omni-md/roadmap.md` | All epics with status tracking |
