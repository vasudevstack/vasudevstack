import { getAuthToken } from "../lib/auth";
import { redirect } from "next/navigation";

async function fetchProfile(token: string) {
  const base = process.env.ENGINE_BASE_URL || "http://localhost:8080";
  const res = await fetch(`${base}/api/protected/profile`, {
    headers: { Authorization: `Bearer ${token}` },
    cache: "no-store",
  });
  if (!res.ok) return null;
  return res.json();
}

async function fetchDashboard(token: string) {
  const base = process.env.ENGINE_BASE_URL || "http://localhost:8080";
  const res = await fetch(`${base}/api/protected/dashboard`, {
    headers: { Authorization: `Bearer ${token}` },
    cache: "no-store",
  });
  if (!res.ok) return null;
  return res.json();
}

export default async function DashboardPage() {
  const token = await getAuthToken();
  if (!token) {
    redirect("/login");
  }

  const [profile, dashboard] = await Promise.all([fetchProfile(token), fetchDashboard(token)]);

  return (
    <div className="min-h-screen bg-white text-black">
      <header className="border-b border-brand-100 bg-brand-600 text-white">
        <div className="max-w-5xl mx-auto px-6 h-14 flex items-center justify-between">
          <h1 className="font-semibold">Dashboard</h1>
          <form action="/api/logout" method="post">
            <button className="text-sm underline/50 hover:underline">Logout</button>
          </form>
        </div>
      </header>

      <main className="max-w-5xl mx-auto px-6 py-10 grid gap-8">
        <div className="grid md:grid-cols-2 gap-6">
          <section className="rounded-xl bg-white shadow-md p-6">
            <h2 className="text-lg font-medium mb-2">Token</h2>
            <pre className="text-xs whitespace-pre-wrap break-all bg-brand-50 rounded-md p-3">{token}</pre>
          </section>
          <section className="rounded-xl bg-white shadow-md p-6">
            <h2 className="text-lg font-medium mb-2">Welcome</h2>
            <p className="text-sm text-neutral-700">{dashboard?.message || "Authenticated"}</p>
          </section>
        </div>

        <section className="rounded-xl bg-white shadow-md p-6">
          <h2 className="text-lg font-medium mb-3">Profile</h2>
          <pre className="text-sm bg-brand-50 rounded-md p-3 overflow-auto">{JSON.stringify(profile, null, 2)}</pre>
        </section>

        <section className="rounded-xl bg-white shadow-md p-6">
          <h2 className="text-lg font-medium mb-3">Dashboard Data</h2>
          <pre className="text-sm bg-brand-50 rounded-md p-3 overflow-auto">{JSON.stringify(dashboard, null, 2)}</pre>
        </section>
      </main>
    </div>
  );
}


