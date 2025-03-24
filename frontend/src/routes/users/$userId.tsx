import { Title } from "@solidjs/meta";
import { createMemo } from "solid-js";
import { createFileRoute } from "@tanstack/solid-router";
import { queryOptions, createQuery } from "@tanstack/solid-query";
import type { JSXElement } from "solid-js";
import { matchResult } from "../../utils/matchResult";
import { backend } from "../../declarations/backend";

const userQueryOptions = (actor: typeof backend, userId: string) =>
  queryOptions({
    queryKey: ["user", { userId }],
    queryFn: async () => {
      return await actor.fetch_user(userId);
    },
    staleTime: 1000 * 60 * 5, // 5 minutes
    throwOnError: true,
  });

export const Route = createFileRoute("/users/$userId")({
  loader: ({ params, context }) =>
    context.queryClient.ensureQueryData(
      userQueryOptions(context.auth.backend, params.userId),
    ),
  component: RouteComponent,
});

function RouteComponent(): JSXElement {
  const context = Route.useRouteContext();
  const params = Route.useParams();
  const userQuery = createQuery(() =>
    userQueryOptions(context().auth.backend, params().userId),
  );
  const data = createMemo(() => userQuery.data);

  return (
    <main class="flex min-h-screen flex-col items-center justify-center">
      {matchResult(data()!, {
        ok: (user) => (
          <>
            <Title>{user.name} | Arche</Title>

            <h1>User Details</h1>
            <p>Name: {user.name}</p>
          </>
        ),
        err: (err) => (
          <>
            <Title>Error | Arche</Title>

            <p>Error: {err}</p>
          </>
        ),
      })}
    </main>
  );
}
