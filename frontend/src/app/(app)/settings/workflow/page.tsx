import type { Metadata } from "next";
import { WorkflowSettingsClient } from "./workflow-client";

export const metadata: Metadata = { title: "Workflow — Grit Jira" };

export default function WorkflowSettingsPage() {
  return <WorkflowSettingsClient />;
}