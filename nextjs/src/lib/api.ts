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

export async function api<T>(path: string, options: RequestOptions = {}): Promise<T> {
  const { json, headers, ...rest } = options;

  const res = await fetch(`/api${path}`, {
    credentials: "include",
    ...rest,
    headers: {
      ...(json !== undefined ? { "Content-Type": "application/json" } : {}),
      ...headers,
    },
    body: json !== undefined ? JSON.stringify(json) : undefined,
  });

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