import type { Metadata } from "next";
import { WebhooksSettingsClient } from "./webhooks-client";

export const metadata: Metadata = { title: "Webhooks — Grit Jira" };

export default function SettingsWebhooksPage() {
  return <WebhooksSettingsClient />;
}