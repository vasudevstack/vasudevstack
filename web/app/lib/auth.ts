"use server";

import { cookies } from "next/headers";

const AUTH_COOKIE = "auth_token";

export async function getAuthToken(): Promise<string | undefined> {
  try {
    const store = await cookies();
    return store.get(AUTH_COOKIE)?.value;
  } catch {
    return undefined;
  }
}

export async function clearAuthToken(): Promise<void> {
  const store = await cookies();
  store.delete(AUTH_COOKIE);
}


