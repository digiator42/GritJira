"use client";

import { useEffect, useState } from "react";
import { usePathname, useRouter } from "next/navigation";
import { api } from "@/lib/api";
import type { Me } from "@/lib/types";
import { AppProvider } from "@/lib/AppContext";
import { Sidebar, Topbar } from "./sidebar-topbar";
import { CommandPalette } from "./CommandPalette";
import { Spinner } from "./ui";

type Phase = "loading" | "anon" | "ok";

export function AppShell({ children }: { children: React.ReactNode }) {
  const [phase, setPhase] = useState<Phase>("loading");
  const [mobileNav, setMobileNav] = useState(false);
  const pathname = usePathname();
  const router = useRouter();

  useEffect(() => {
    let cancelled = false;
    api<Me>("/api/v1/auth/me")
      .then(() => {
        if (!cancelled) setPhase("ok");
      })
      .catch((e) => {
        if (cancelled) return;
        const status = (e as { status?: number }).status;
        if (status === 401) {
          setPhase("anon");
        } else {
          // backend unreachable — surface the error screen instead of looping
          setPhase("ok");
        }
      });
    return () => {
      cancelled = true;
    };
  }, []);

  useEffect(() => {
    if (phase === "anon") {
      router.replace("/login");
    }
  }, [phase, router]);

  useEffect(() => {
    setMobileNav(false);
  }, [pathname]);

  if (phase === "loading") {
    return (
      <div className="flex h-screen items-center justify-center">
        <Spinner label="Loading workspace…" />
      </div>
    );
  }

  if (phase === "anon") {
    return <div className="flex h-screen items-center justify-center" />;
  }

  return (
    <AppProvider>
      <div className="flex h-screen overflow-hidden bg-jira-bg">
        <Sidebar pathname={pathname} mobileOpen={mobileNav} onClose={() => setMobileNav(false)} />
        <div className="flex min-w-0 flex-1 flex-col">
          <Topbar onMenu={() => setMobileNav(true)} />
          <main className="min-h-0 flex-1 overflow-y-auto">{children}</main>
        </div>
      </div>
      <CommandPalette />
    </AppProvider>
  );
}