"use client";

import { useCallback, useEffect, useRef, useState } from "react";
import { api, ApiError } from "./api";

export interface RequestState<T> {
  data: T | null;
  error: string | null;
  loading: boolean;
  reload: () => void;
  setData: (data: T) => void;
}

/** Minimal data-fetching hook to keep the bundle free of extra deps. */
export function useRequest<T>(fetcher: () => Promise<T>, deps: unknown[]): RequestState<T> {
  const [data, setData] = useState<T | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [tick, setTick] = useState(0);
  const fetcherRef = useRef(fetcher);
  fetcherRef.current = fetcher;

  const reload = useCallback(() => setTick((t) => t + 1), []);

  useEffect(() => {
    let cancelled = false;
    setLoading(true);
    setError(null);
    fetcherRef
      .current()
      .then((d) => {
        if (!cancelled) setData(d);
      })
      .catch((e) => {
        if (!cancelled) {
          if (e instanceof ApiError && e.status === 401) {
            window.location.href = "/login";
            return;
          }
          setError(e instanceof Error ? e.message : "Request failed");
        }
      })
      .finally(() => {
        if (!cancelled) setLoading(false);
      });
    return () => {
      cancelled = true;
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [...deps, tick]);

  return { data, error, loading, reload, setData };
}

export function useApi<T>(path: string | null, deps: unknown[] = []): RequestState<T> {
  return useRequest<T>(
    useCallback(async () => {
      if (!path) throw new Error("No path");
      return (await api<{ data: T }>(path)).data;
    }, [path]),
    [...deps, path],
  );
}