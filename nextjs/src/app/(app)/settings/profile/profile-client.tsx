"use client";

import { useState } from "react";
import { api, ApiError } from "@/lib/api";
import { useApp } from "@/lib/AppContext";
import { Avatar, ErrorBox, Field } from "@/components/ui";

export function ProfileSettingsClient() {
  const { me, refreshProjects } = useApp();
  const [username, setUsername] = useState(me?.username ?? "");
  const [email, setEmail] = useState(me?.email ?? "");
  const [currentPassword, setCurrentPassword] = useState("");
  const [newPassword, setNewPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [notice, setNotice] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  async function saveProfile(e: React.FormEvent) {
    e.preventDefault();
    setError(null);
    setNotice(null);
    setBusy(true);
    try {
      await api("/api/v1/users/me", {
        method: "PATCH",
        json: {
          username: username.trim() || undefined,
          email: email.trim() || undefined,
        },
      });
      await refreshProjects();
      setNotice("Profile updated.");
      setEmail(email.trim());
      setUsername(username.trim());
    } catch (err) {
      setError(err instanceof ApiError ? err.message : "Failed to update profile");
    } finally {
      setBusy(false);
    }
  }

  async function savePassword(e: React.FormEvent) {
    e.preventDefault();
    setError(null);
    setNotice(null);
    if (!currentPassword || !newPassword) return;
    if (newPassword !== confirmPassword) {
      setError("New password and confirmation do not match.");
      return;
    }
    setBusy(true);
    try {
      await api("/api/v1/users/me/password", {
        method: "POST",
        json: { current_password: currentPassword, new_password: newPassword },
      });
      setCurrentPassword("");
      setNewPassword("");
      setConfirmPassword("");
      setNotice("Password changed.");
    } catch (err) {
      setError(err instanceof ApiError ? err.message : "Failed to change password");
    } finally {
      setBusy(false);
    }
  }

  return (
    <div className="mx-auto max-w-3xl p-4">
      <div className="mb-4">
        <h1 className="text-base font-semibold text-jira-text">Profile &amp; account</h1>
        <p className="text-xs text-jira-muted">Your name, email, and password.</p>
      </div>

      {error ? (
        <div className="mb-4">
          <ErrorBox message={error} />
        </div>
      ) : null}
      {notice ? (
        <div className="panel mb-4 border-emerald-800/40 bg-emerald-950/20 p-3 text-sm text-emerald-300">
          {notice}
        </div>
      ) : null}

      <div className="mb-4 flex items-center gap-3">
        <Avatar name={me?.username ?? "?"} size={48} />
        <div>
          <p className="text-sm font-medium text-jira-text">{me?.username ?? "–"}</p>
          <p className="text-xs text-jira-muted">{me?.email ?? "–"}</p>
        </div>
      </div>

      <form onSubmit={saveProfile} className="panel space-y-3 p-4">
        <h2 className="text-[10px] font-semibold uppercase tracking-widest text-jira-faint">
          Name &amp; email
        </h2>
        <Field label="Username">
          <input
            required
            className="input"
            value={username}
            onChange={(e) => setUsername(e.target.value)}
            placeholder="username"
          />
        </Field>
        <Field label="Email">
          <input
            type="email"
            required
            className="input"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            placeholder="you@example.com"
          />
        </Field>
        <div className="pt-1">
          <button type="submit" disabled={busy} className="btn-primary">
            {busy ? "Saving…" : "Save profile"}
          </button>
        </div>
      </form>

      <form onSubmit={savePassword} className="panel mt-4 space-y-3 p-4">
        <h2 className="text-[10px] font-semibold uppercase tracking-widest text-jira-faint">
          Change password
        </h2>
        <Field label="Current password">
          <input
            type="password"
            className="input"
            value={currentPassword}
            onChange={(e) => setCurrentPassword(e.target.value)}
            autoComplete="current-password"
          />
        </Field>
        <Field label="New password">
          <input
            type="password"
            className="input"
            value={newPassword}
            onChange={(e) => setNewPassword(e.target.value)}
            autoComplete="new-password"
            placeholder="At least 6 characters"
          />
        </Field>
        <Field label="Confirm new password">
          <input
            type="password"
            className="input"
            value={confirmPassword}
            onChange={(e) => setConfirmPassword(e.target.value)}
            autoComplete="new-password"
          />
        </Field>
        <div className="pt-1">
          <button
            type="submit"
            disabled={busy || !currentPassword || !newPassword || newPassword !== confirmPassword}
            className="btn-primary"
          >
            {busy ? "Saving…" : "Change password"}
          </button>
        </div>
      </form>
    </div>
  );
}