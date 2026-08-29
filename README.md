# Grit Jira

A Jira-style project management and issue-tracking application. Rust (GritShield)
backend exposing a JSON API over PostgreSQL, plus a modern Next.js frontend.

## Repository Layout

```
GritJira/
├── backend/    Rust GritShield API (controllers, services, repositories, migrations)
└── frontend/   Next.js 15 (App Router) + React + Tailwind UI
```

## Key Features

- **Projects** — create/manage projects, each with its own workflow and members
- **Issues** — types (bug, story, task, custom), priorities, story points,
  time tracking, due dates, attachments, comments
- **Agile boards** — Kanban columns with drag & drop, sprints with goals,
  start/complete/reopen, and burndown charts
- **Workflow** — customizable status columns (steps) per project
- **Search** — simple JQL-style issue search (`issue_type = bug`, `summary LIKE deploy`)
- **Settings** — project general, custom issue types, members & roles, workflow,
  webhooks, and your own profile
- **Notifications & activity** — event-driven feed (issues, comments, sprints)
- **RBAC** — Admin / Manager / Developer / Tester / Viewer caps enforced in the
  backend; members view the roster, admins manage roles

## Technology Stack

- **Backend**: Rust, GritShield framework, SeaORM, PostgreSQL
  (migrations auto-run on startup, seed data included)
- **Frontend**: Next.js 15, React 19, Tailwind CSS, TypeScript
- **API**: JSON under `/api/v1`, session auth (`GSESSION_ID` cookie); the
  frontend proxies `/api/*` → `localhost:8080` in dev

## Getting Started

Requirements: Rust toolchain, Node.js 20+, PostgreSQL (local database `grit_jira`).

1. **Configure the backend**

   Copy/check `backend/.env` — it must point at your database, e.g.
   `DATABASE_URL=postgres://postgres:admin@localhost:5432/grit_jira`.

2. **Start the backend** (from `backend/`)

   ```bash
   cargo run
   ```

   Migrations in `backend/migrations/` apply automatically, demo data is seeded,
   and the server listens on `http://localhost:8080`.

3. **Start the frontend** (from `frontend/`)

   ```bash
   npm install
   npm run dev
   ```

   Open http://localhost:3000

## Demo Logins

Seeded by the backend on startup:

| Email                  | Password   | Role      |
| ---------------------- | ---------- | --------- |
| `admin@gritjira.local` | `admin123` | Admin     |
| `alex@gritjira.local`  | `alex123`  | Developer |

## API Notes

- Responses are wrapped as `{ success, data }` (see `api()` / `apiData()` in
  `frontend/src/lib/api.ts`).
- Issue search uses the backend JQL engine (single `column op value` condition;
  `LIKE` → ILIKE on Postgres).
- Sprint statuses are reported by the backend as `Planning` / `active` /
  `completed`; `normalizeSprintStatus()` in `frontend/src/lib/format.ts`
  normalizes them.

See `frontend/README.md` for frontend details.