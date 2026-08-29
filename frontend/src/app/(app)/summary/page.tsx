import type { Metadata } from "next";
import { SummaryClient } from "./summary-client";

export const metadata: Metadata = { title: "Summary — Grit Jira" };

export default function SummaryPage() {
  return <SummaryClient />;
}