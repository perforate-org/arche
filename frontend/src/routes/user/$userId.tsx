import { Title } from "@solidjs/meta";
import { createFileRoute } from "@tanstack/solid-router";
import type { JSXElement } from "solid-js";
import { backend } from "../../declarations/backend";
import { matchResult } from "../../utils/matchResult";

export const Route = createFileRoute("/user/$userId")({
  loader: ({ params }) => backend.fetch_user(params.userId),
  component: RouteComponent,
});

function RouteComponent(): JSXElement {
  const data = Route.useLoaderData();

  return matchResult(data(), {
    ok: (user) => (
      <main class="min-h-screen flex flex-col items-center justify-center">
        <Title>{user.name} | Arche</Title>

        <h1>User Details</h1>
        <p>Name: {user.name}</p>
      </main>
    ),
    err: (err) => (
      <main class="min-h-screen flex flex-col items-center justify-center">
        <Title>Error | Arche</Title>

        <p>Error: {err}</p>
      </main>
    ),
  });
}
