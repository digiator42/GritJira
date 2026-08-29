import type { Config } from "tailwindcss";

const config: Config = {
  darkMode: "class",
  content: ["./src/**/*.{ts,tsx}"],
  theme: {
    extend: {
      colors: {
        jira: {
          blue: "var(--jira-blue)",
          "blue-hover": "var(--jira-blue-hover)",
          bg: "var(--jira-bg)",
          panel: "var(--jira-panel)",
          card: "var(--jira-card)",
          "card-hover": "var(--jira-card-hover)",
          border: "var(--jira-border)",
          text: "var(--jira-text)",
          muted: "var(--jira-muted)",
          faint: "var(--jira-faint)",
        },
      },
      fontFamily: {
        sans: [
          "Inter",
          "-apple-system",
          "BlinkMacSystemFont",
          "Segoe UI",
          "Roboto",
          "Helvetica Neue",
          "Arial",
          "sans-serif",
        ],
      },
    },
  },
  plugins: [],
};

export default config;