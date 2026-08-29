import type { Metadata } from "next";
import BacklogClient from "./backlog-client";

export const metadata: Metadata = { title: "Backlog — Grit Jira" };

export default async function BacklogPage({
  searchParams,
}: {
  searchParams: Promise<{ project_id?: string }>;
}) {
  const sp = await searchParams;
  return <BacklogClient initialProjectId={sp.project_id ? Number(sp.project_id) : undefined} />;
}