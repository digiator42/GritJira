# Grit Jira — Next.js frontend

Modern Jira-style project tracking UI for the GritJira Rust backend
(GritShield + SeaORM + PostgreSQL). The Next.js app talks to the JSON-only
API exposed by the Rust server, proxied in dev to avoid CORS.

## Architecture

- **Backend** (`../backend`): Rust (GritShield framework). Serves a
  JSON-only API under `/api/v1`, owns sessions (`GSESSION_ID` cookie) and is
  the single source of truth. Postgres DB required.
- **Frontend** (this directory): Next.js 15 (App Router) + React 19 +
  Tailwind CSS. Dark Jira-style theme.
- Dev proxy: `next.config.mjs` rewrites `/api/*` → `http://localhost:8080/api/*`

## Requirements

- Node.js 20+ (tested with Node 24)
- Rust toolchain (for the backend)
- PostgreSQL running locally with a database named `grit_jira`
  (set `DATABASE_URL=postgres://postgres:admin@localhost:5432/grit_jira`
  in the backend `../backend/.env`)

## Running both servers

1. **Start the backend** (from `../backend`):

   ```bash
   cargo run
   ```

   On first launch this seeds demo users, projects and issues
   (`database/seeder`). Listens on `http://localhost:8080`.

2. **Install and start the frontend** (from this directory):

   ```bash
   npm install
   npm run dev
   ```

   Open http://localhost:3000

## Demo logins

Seeded by the backend on startup:

| Email                    | Password | Role      |
| ------------------------ | -------- | --------- |
| `admin@gritjira.local`   | `admin123` | Admin   |
| `alex@gritjira.local`    | `alex123` | Developer |

## Pages

- `/login`, `/register` — auth (also serves `/`)
- `/board?project_id=N&sprint_id=M` — Kanban board with drag & drop moves
- `/backlog` — unassigned issues + sprint management (create / start / complete / delete)
- `/issues/:id` — issue detail (edit summary/description, move workflow step,
  assignee/priority/type/points, comments, delete)
- `/projects` and `/projects/:id` — project list/create/delete and issue list
- `/search` — issue searching with single-condition JQL + project search
- `/settings/users` — project member management
- `/settings/workflow` — workflow column (status) management

## API notes

- All API responses are wrapped as `{ success, data }`.
  `api<T>()` in `src/lib/api.ts` returns the raw JSON document; callers that
  need the payload use `api<{ data: T }>(path)` (see `data()` / `apiData()`
  helpers).
- Issue search uses the backend JQL engine, which supports a **single**
  `column op value` condition at a time:
  - `priority = 3`, `issue_type = bug`, `summary LIKE deploy`, `key LIKE GRIT`
  - `LIKE` is case-insensitive (ILIKE on Postgres); `=` is exact/case-sensitive
  - a bare keyword is converted to `summary LIKE <keyword>` (cross-project)
- Sprint statuses are reported inconsistently by the backend
  (`Planning` / `active` / `completed`); `normalizeSprintStatus()`
  in `src/lib/format.ts` normalizes them.

## Production build

```bash
npm run build
npm run start   # serves the production build (keeps the /api proxy rewrites)
```