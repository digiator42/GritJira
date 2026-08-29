import type { Metadata } from "next";
import { IssueTypesSettingsClient } from "./issue-types-client";

export const metadata: Metadata = { title: "Issue types — Grit Jira" };

export default function SettingsIssueTypesPage() {
  return <IssueTypesSettingsClient />;
}