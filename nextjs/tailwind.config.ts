import type { Config } from "tailwindcss";

const config: Config = {
  content: ["./src/**/*.{ts,tsx}"],
  theme: {
    extend: {
      colors: {
        jira: {
          blue: "#0052cc",
          "blue-hover": "#0747a6",
          bg: "#0f1216",
          panel: "#161a21",
          border: "#232931",
          text: "#d9e0e8",
          muted: "#8b99a7",
          faint: "#5c6b7a",
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