export class ApiError extends Error {
  status: number;
  constructor(status: number, message: string) {
    super(message);
    this.name = "ApiError";
    this.status = status;
  }
}

import type { ApiResponse } from "./types";

type RequestOptions = Omit<RequestInit, "body"> & {
  json?: unknown;
};

const TOKEN_KEY = "grit.authToken";

export function getToken(): string | null {
  if (typeof window === "undefined") return null;
  return window.localStorage.getItem(TOKEN_KEY);
}

export function setToken(token: string): void {
  if (typeof window === "undefined") return;
  window.localStorage.setItem(TOKEN_KEY, token);
}

export function clearToken(): void {
  if (typeof window === "undefined") return;
  window.localStorage.removeItem(TOKEN_KEY);
}

export async function api<T>(path: string, options: RequestOptions = {}): Promise<T> {
  const { json, headers, ...rest } = options;

  const token = getToken();
  const url = path.startsWith("/api") ? path : `/api${path}`;
  const res = await fetch(url, {
    credentials: "include",
    ...rest,
    headers: {
      ...(json !== undefined ? { "Content-Type": "application/json" } : {}),
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
      ...headers,
    },
    body: json !== undefined ? JSON.stringify(json) : undefined,
  });

  if (res.status === 401) clearToken();

  if (!res.ok) {
    let message = `Request failed (${res.status})`;
    try {
      const body = await res.json();
      if (body?.message) message = body.message;
      else if (body?.error) message = typeof body.error === "string" ? body.error : message;
      else if (typeof body === "string" && body.length > 0 && body.length < 400) message = body;
    } catch {
      // ignore parse failure
    }
    throw new ApiError(res.status, message);
  }

  return (await res.json()) as T;
}

// Convenience: unwrap the `data` field of ApiResponse<T>
export function data<T>(res: ApiResponse<T>): T {
  return res.data;
}

export const apiData = <T>(path: string, options?: RequestOptions) =>
  api<ApiResponse<T>>(path, options).then((r) => r.data);