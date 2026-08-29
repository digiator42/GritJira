import type { Metadata } from "next";
import { GeneralSettingsClient } from "./general-client";

export const metadata: Metadata = { title: "General — Grit Jira" };

export default function SettingsGeneralPage() {
  return <GeneralSettingsClient />;
}