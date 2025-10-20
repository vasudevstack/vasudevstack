"use client";

import { useEffect, useState } from "react";
import { LogIn, ShieldCheck } from "lucide-react";
import { Button } from "../components/ui/button";
import { Input } from "../components/ui/input";
import { Label } from "../components/ui/label";
import { toast } from "sonner";
import { useRouter } from "next/navigation";

export default function LoginPage() {
  const router = useRouter();
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    // Server-side redirects already handle auth; no client prefetch needed
  }, [router]);

  const validate = () => {
    const issues: string[] = [];
    const u = username.trim();
    const p = password;
    if (!u) issues.push("Username is required");
    if (u && u.length < 3) issues.push("Username must be at least 3 characters");
    if (u && !/^[a-zA-Z0-9._-]+$/.test(u))
      issues.push("Username can contain letters, numbers, ., _, - only");
    if (!p) issues.push("Password is required");
    if (p && p.length < 6) issues.push("Password must be at least 6 characters");
    return issues;
  };

  const onSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);
    setLoading(true);
    try {
      const issues = validate();
      if (issues.length) {
        issues.forEach((m) => toast.error(m));
        setError(issues[0]);
        setLoading(false);
        return;
      }
      const res = await fetch("/api/login", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ username, password }),
      });
      const data = await res.json();
      if (!res.ok) {
        const msg = data?.reason || "Login failed";
        setError(msg);
        toast.error(msg);
      } else {
        toast.success("Login successful");
        router.replace("/dashboard");
      }
    } catch (err) {
      setError("Network error");
      toast.error("Network error");
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="min-h-screen grid place-items-center bg-white text-black relative overflow-hidden">
      <div className="pointer-events-none absolute -top-24 -left-24 h-80 w-80 rounded-full bg-brand-50 blur-3xl" />
      <div className="pointer-events-none absolute -bottom-24 -right-24 h-80 w-80 rounded-full bg-brand-100 blur-3xl" />

      <div className="w-full max-w-md rounded-2xl border border-brand-100 shadow-sm p-8 bg-white/90 backdrop-blur">
        <div className="flex items-center gap-3 mb-6">
          <div className="h-10 w-10 rounded-full bg-brand-600 grid place-items-center">
            <ShieldCheck className="text-white" size={20} />
          </div>
          <div>
            <h1 className="text-2xl font-semibold tracking-tight">Welcome</h1>
            <p className="text-sm text-brand-700/80">Secure access to your dashboard</p>
          </div>
        </div>

        <form onSubmit={onSubmit} className="space-y-4">
          <div>
            <Label className="mb-1 block">Username</Label>
            <Input
              value={username}
              onChange={(e) => setUsername(e.target.value)}
              onBlur={() => {
                const issues = validate();
                const msg = issues.find((m) => m.toLowerCase().includes("username"));
                if (msg) toast.error(msg);
              }}
              placeholder="your.username"
              autoComplete="username"
            />
          </div>
          <div>
            <Label className="mb-1 block">Password</Label>
            <Input
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              onBlur={() => {
                const issues = validate();
                const msg = issues.find((m) => m.toLowerCase().includes("password"));
                if (msg) toast.error(msg);
              }}
              placeholder="••••••••"
              autoComplete="current-password"
            />
          </div>

          {error ? (
            <div className="text-sm text-brand-700 bg-brand-50 border border-brand-100 rounded-md p-2">
              {error}
            </div>
          ) : null}

          <Button type="submit" disabled={loading} className="w-full h-11 gap-2">
            <LogIn size={18} />
            {loading ? "Signing in..." : "Sign in"}
          </Button>
        </form>

        <p className="text-xs text-center mt-4 text-neutral-600">
          By continuing you agree to our terms and privacy policy.
        </p>
      </div>

      <div className="fixed top-0 left-0 right-0 h-1 bg-red-600" />
    </div>
  );
}


