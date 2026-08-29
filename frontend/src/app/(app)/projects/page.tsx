import type { Metadata } from "next";
import { ProjectsClient } from "./projects-client";

export const metadata: Metadata = { title: "Projects — Grit Jira" };

export default function ProjectsPage() {
  return <ProjectsClient />;
}