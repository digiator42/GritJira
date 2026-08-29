import type { Metadata } from "next";
import { BurndownClient } from "./burndown-client";

export const metadata: Metadata = { title: "Burndown — Grit Jira" };

export default function BurndownPage() {
  return <BurndownClient />;
}