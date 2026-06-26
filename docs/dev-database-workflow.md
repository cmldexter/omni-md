# Dev Database Workflow — Agent Reference

> **Purpose**: Follow this workflow when you need to work with the local dev database,
> particularly when you need to inspect, reset, or back up the SQLite sync event store.
>
> **Trigger**: Use this when debugging sync events, testing webhook pipelines,
> or when the dev database needs to be reset to a clean state.

---

## Architecture

Omni-MD uses an **embedded SQLite database** with WAL (Write-Ahead Logging) mode, stored inside the `md-relay-worker` Docker container. Unlike PostgreSQL-based projects, there is no separate database container — the database file lives on a Docker volume.

```
Local Dev (Docker Compose)
├── Container: md-relay-worker
│   ├── Runtime: Rust binary (Axum + Tokio)
│   ├── Database: /app/data/omni.db (SQLite, WAL mode)
│   └── Connection: DATABASE_URL=sqlite:///app/data/omni.db
│
└── Volume: omni_db_data → /app/data/
    ├── omni.db         (main database)
    ├── omni.db-wal     (WAL journal)
    └── omni.db-shm     (shared memory index)
```

**Key points:**
- Database is managed by `sqlx` with compile-time checked queries
- WAL mode allows concurrent reads during writes (ideal for the MPSC job queue pattern)
- The `omni_db_data` Docker volume persists across container restarts
- The non-root `genome` user inside the container owns the data directory

---

## Prerequisites

| Requirement | Check | Fix |
|------------|-------|-----|
| Docker Compose running | `docker compose ps` shows `md-relay-worker` running | `docker compose up -d --build` |
| Health check passing | `curl http://localhost:8080/api/health/liveness` returns OK | Check logs: `docker compose logs md-relay-worker` |

---

## Commands

### Inspect the Database

```bash
# Open an interactive SQLite shell inside the container
docker exec -it md-relay-worker sqlite3 /app/data/omni.db

# List all tables
docker exec md-relay-worker sqlite3 /app/data/omni.db ".tables"

# Check row counts
docker exec md-relay-worker sqlite3 /app/data/omni.db "SELECT COUNT(*) FROM sync_events;"

# View recent sync events
docker exec md-relay-worker sqlite3 /app/data/omni.db \
  "SELECT * FROM sync_events ORDER BY created_at DESC LIMIT 10;"

# Check WAL mode is enabled
docker exec md-relay-worker sqlite3 /app/data/omni.db "PRAGMA journal_mode;"
# Should output: wal
```

### Query via API

The sync event log is also available via the REST API:

```bash
# Get all sync events (newest first)
curl -s http://localhost:8080/api/logs | python3 -m json.tool

# Health check (includes DB readiness)
curl -s http://localhost:8080/api/health/readiness | python3 -m json.tool
```

### Reset to Empty DB

Wipe the database volume and rebuild from scratch:

```bash
# Stop containers, remove volumes, rebuild, restart
docker compose down -v && docker compose up -d --build

# Verify health
curl -s http://localhost:8080/api/health/liveness
```

This drops the `omni_db_data` volume entirely. The Rust app recreates the schema on startup via `sqlx` migrations.

### Backup the Database

```bash
# Copy the DB file out of the container
docker cp md-relay-worker:/app/data/omni.db ./omni-backup.db

# Or with timestamp
docker cp md-relay-worker:/app/data/omni.db ./omni-backup-$(date +%Y%m%d-%H%M%S).db
```

> ⚠️ **WAL checkpoint before backup**: For a fully consistent backup, checkpoint the WAL first:
> ```bash
> docker exec md-relay-worker sqlite3 /app/data/omni.db "PRAGMA wal_checkpoint(TRUNCATE);"
> docker cp md-relay-worker:/app/data/omni.db ./omni-backup.db
> ```

### Restore a Backup

```bash
# Stop the worker
docker compose stop md-relay-worker

# Copy backup into the volume
docker cp ./omni-backup.db md-relay-worker:/app/data/omni.db

# Restart
docker compose start md-relay-worker

# Verify
curl -s http://localhost:8080/api/health/readiness
```

---

## When to Reset

| Scenario | Reset? | Reason |
|----------|--------|--------|
| Testing webhook pipeline from scratch | ✅ Yes | Need clean sync event log |
| Debugging a corrupted database | ✅ Yes | Easiest fix for embedded SQLite |
| Schema changed (new migration) | ✅ Yes | `docker compose down -v && up --build` picks up new schema |
| UI-only changes (CSS, layout) | ❌ No | No data dependency |
| Backend logic changes (no schema) | ❌ No | Data can be preserved |
| After adding new sync route configs | ❌ No | Config is in YAML, not the DB |

---

## Troubleshooting

### Database is Locked

```
Error: database is locked
```

**Fix:** This usually means another process has an exclusive lock. Check if:
1. Multiple containers are running against the same volume: `docker compose ps`
2. A zombie process holds the lock: restart the container: `docker compose restart md-relay-worker`
3. WAL checkpoint stalled: `docker exec md-relay-worker sqlite3 /app/data/omni.db "PRAGMA wal_checkpoint(TRUNCATE);"`

### Volume Permission Errors

```
Error: unable to open database file
```

**Fix:** The container runs as the `genome` user (non-root). If volume permissions are wrong:
```bash
# Nuclear option — recreate the volume
docker compose down -v && docker compose up -d --build
```

### Container Won't Start (Health Check Fails)

```
md-relay-worker is unhealthy
```

**Fix:**
1. Check logs: `docker compose logs md-relay-worker --tail=50`
2. If DB-related: reset the volume: `docker compose down -v && docker compose up -d --build`
3. If build-related: rebuild from scratch: `docker compose build --no-cache md-relay-worker`

### Missing sqlite3 in Container

If `sqlite3` is not installed in the production image (multi-stage builds may strip it):

```bash
# Alternative: copy the DB out and inspect locally
docker cp md-relay-worker:/app/data/omni.db /tmp/omni-inspect.db
sqlite3 /tmp/omni-inspect.db ".tables"
```

---

## Related Docs

| Doc | Path | When to use |
|-----|------|-------------|
| **This Doc** | `docs/dev-database-workflow.md` | When inspecting, resetting, or backing up the SQLite dev DB |
| **Local Dev Testing** | `docs/local-dev-testing-workflow.md` | Docker Compose dev environment reference |
| **Epic Worksession** | `docs/epic-worksession-workflow.md` | Start of every focused epic session |
| **Issue Tracker** | `docs/issue-tracker-workflow.md` | For ad-hoc issue work outside epics |
