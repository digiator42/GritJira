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
      <head>
        <script
          dangerouslySetInnerHTML={{
            __html: `(function(){try{var t=localStorage.getItem("gritjira-theme")||"dark";document.documentElement.className=t==="light"?"light":"dark";}catch(e){document.documentElement.className="dark";}})();`,
          }}
        />
      </head>
      <body>{children}</body>
    </html>
  );
}