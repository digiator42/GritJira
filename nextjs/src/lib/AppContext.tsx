"use client";

import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
} from "react";
import { api } from "@/lib/api";
import type { Me, Project, User } from "@/lib/types";

interface AppContextValue {
  me: Me | null;
  projects: Project[];
  users: User[];
  currentProject: Project | null;
  error: string | null;
  selectProject: (id: number) => void;
  refreshProjects: () => Promise<void>;
}

const AppContext = createContext<AppContextValue | null>(null);

const STORAGE_KEY = "grit.currentProjectId";

export function useApp(): AppContextValue {
  const ctx = useContext(AppContext);
  if (!ctx) throw new Error("useApp must be used within AppProvider");
  return ctx;
}

export function AppProvider({ children }: { children: React.ReactNode }) {
  const [me, setMe] = useState<Me | null>(null);
  const [projects, setProjects] = useState<Project[]>([]);
  const [users, setUsers] = useState<User[]>([]);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    try {
      const [meData, projectsData, usersData] = await Promise.all([
        api<{ data: Me }>("/api/v1/auth/me"),
        api<{ data: Project[] }>("/api/v1/projects"),
        api<{ data: User[] }>("/api/v1/users"),
      ]);
      setMe(meData.data);
      setProjects(projectsData.data);
      setUsers(usersData.data);
      setError(null);
    } catch (e) {
      setError(e instanceof Error ? e.message : "Failed to load app data");
    }
  }, []);

  useEffect(() => {
    load();
  }, [load]);

  const refreshProjects = useCallback(async () => {
    const [meData, projectsData] = await Promise.all([
      api<{ data: Me }>("/api/v1/auth/me"),
      api<{ data: Project[] }>("/api/v1/projects"),
    ]);
    setMe(meData.data);
    setProjects(projectsData.data);
  }, []);

  const currentProject = useMemo(() => {
    if (projects.length === 0) return null;
    const stored = parseInt(localStorage.getItem(STORAGE_KEY) ?? "", 10);
    const fromStorage = Number.isFinite(stored) ? projects.find((p) => p.id === stored) : undefined;
    if (fromStorage) return fromStorage;
    const fromMe = me?.current_project_id
      ? projects.find((p) => p.id === me.current_project_id)
      : undefined;
    return fromMe ?? projects[0];
  }, [projects, me]);

  const selectProject = useCallback((id: number) => {
    localStorage.setItem(STORAGE_KEY, String(id));
  }, []);

  const value = useMemo(
    () => ({ me, projects, users, currentProject, error, selectProject, refreshProjects }),
    [me, projects, users, currentProject, error, selectProject, refreshProjects],
  );

  return <AppContext.Provider value={value}>{children}</AppContext.Provider>;
}