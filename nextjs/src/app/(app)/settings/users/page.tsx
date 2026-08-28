import type { Metadata } from "next";
import { UsersSettingsClient } from "./users-client";

export const metadata: Metadata = { title: "Users & Members — Grit Jira" };

export default function SettingsUsersPage() {
  return <UsersSettingsClient />;
}