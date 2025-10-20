import { NextRequest, NextResponse } from "next/server";

const ENGINE_BASE_URL = process.env.ENGINE_BASE_URL || "http://localhost:8080";

export async function POST(req: NextRequest) {
  try {
    const body = await req.json();

    const res = await fetch(`${ENGINE_BASE_URL}/api/auth/login`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(body),
    });

    const data = await res.json();

    if (!res.ok) {
      return NextResponse.json(data, { status: res.status });
    }

    const response = NextResponse.json(data, { status: 200 });

    if (data?.token) {
      response.cookies.set("auth_token", data.token, {
        httpOnly: true,
        sameSite: "lax",
        secure: process.env.NODE_ENV === "production",
        path: "/",
        maxAge: 60 * 60 * 24, // 1 day
      });
    }

    return response;
  } catch (error) {
    return NextResponse.json(
      { status: "error", reason: "invalid request" },
      { status: 400 }
    );
  }
}


