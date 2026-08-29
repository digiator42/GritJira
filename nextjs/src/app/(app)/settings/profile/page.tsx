import type { Metadata } from "next";
import { ProfileSettingsClient } from "./profile-client";

export const metadata: Metadata = { title: "Profile — Grit Jira" };

export default function SettingsProfilePage() {
  return <ProfileSettingsClient />;
}