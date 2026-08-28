import type { Metadata } from "next";
import { ProjectDetailClient } from "./project-detail-client";

export const metadata: Metadata = { title: "Project — Grit Jira" };

export default async function ProjectDetailPage({
  params,
}: {
  params: Promise<{ id: string }>;
}) {
  const { id } = await params;
  return <ProjectDetailClient id={Number(id)} />;
}