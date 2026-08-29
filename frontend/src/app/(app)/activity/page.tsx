import type { Metadata } from "next";
import { ActivityClient } from "./activity-client";

export const metadata: Metadata = { title: "Activity — Grit Jira" };

export default function ActivityPage() {
  return <ActivityClient />;
}