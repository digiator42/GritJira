import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "Grit Jira",
  description:
    "A Jira-style issue tracker backed by a Rust (GritShield) JSON API.",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" className="dark">
      <body>{children}</body>
    </html>
  );
}