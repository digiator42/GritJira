"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import Link from "next/link";
import { api, ApiError } from "@/lib/api";
import type { AuthResponse } from "@/lib/types";

export default function LoginPage() {
  const router = useRouter();
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  async function submit(e: React.FormEvent) {
    e.preventDefault();
    setError(null);
    setBusy(true);
    try {
      await api<AuthResponse>("/api/v1/auth/login", {
        method: "POST",
        json: { email, password },
      });
      router.replace("/board");
      router.refresh();
    } catch (err) {
      setError(err instanceof ApiError ? err.message : "Login failed");
    } finally {
      setBusy(false);
    }
  }

  return (
    <div className="panel p-6">
      <div className="mb-5 flex items-center gap-2">
        <span className="flex h-8 w-8 items-center justify-center rounded bg-jira-blue text-base font-bold text-white">
          G
        </span>
        <h1 className="text-lg font-bold text-jira-text">Grit Jira</h1>
      </div>

      <h2 className="mb-4 text-sm font-semibold text-jira-text">Sign in</h2>

      <form onSubmit={submit} className="space-y-4">
        <div>
          <label className="label">Email</label>
          <input
            type="email"
            required
            autoComplete="email"
            className="input"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            placeholder="you@example.com"
          />
        </div>
        <div>
          <label className="label">Password</label>
          <input
            type="password"
            required
            autoComplete="current-password"
            className="input"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            placeholder="••••••••"
          />
        </div>

        {error ? (
          <p className="rounded-md border border-red-900/50 bg-red-950/20 px-3 py-2 text-xs text-red-300">
            {error}
          </p>
        ) : null}

        <button type="submit" disabled={busy} className="btn-primary w-full">
          {busy ? "Signing in…" : "Sign in"}
        </button>
      </form>

      <p className="mt-4 text-center text-xs text-jira-muted">
        New here?{" "}
        <Link href="/register" className="text-jira-blue hover:underline">
Create an account
            </Link>
      </p>
      <p className="mt-3 rounded-md bg-jira-panel px-3 py-2 text-center text-[10px] text-jira-faint">
        Seed users: admin@gritjira.local / admin123 · alex@gritjira.local / alex123
      </p>
    </div>
  );
}