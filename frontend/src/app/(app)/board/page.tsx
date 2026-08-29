import type { Metadata } from "next";
import BoardClient from "./board-client";

export const metadata: Metadata = { title: "Board — Grit Jira" };

export default async function BoardPage({
  searchParams,
}: {
  searchParams: Promise<{ project_id?: string; sprint_id?: string }>;
}) {
  const sp = await searchParams;
  return (
    <BoardClient
      initialProjectId={sp.project_id ? Number(sp.project_id) : undefined}
      initialSprintId={sp.sprint_id ? Number(sp.sprint_id) : undefined}
    />
  );
}