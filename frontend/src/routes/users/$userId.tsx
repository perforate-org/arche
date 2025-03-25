import { Title } from "@solidjs/meta";
import { createMemo, createEffect, onMount, onCleanup, For } from "solid-js";
import { createFileRoute, Link } from "@tanstack/solid-router";
import { queryOptions, createQuery } from "@tanstack/solid-query";
import type { JSXElement } from "solid-js";
import { matchResult } from "../../utils/result";
import { backend } from "../../declarations/backend";
import type { User } from "../../declarations/backend/backend.did";

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
        ok: (user) => <User user={user} />,
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

function User({ user }: { user: User }) {
  let userRef: HTMLElement | undefined;
  let userBorderRef: HTMLDivElement | undefined;

  function updateHeight() {
    if (userRef && userBorderRef) {
      userBorderRef.style.height = `${userRef.offsetHeight}px`;
    }
  }

  onMount(() => {
    window.addEventListener("resize", updateHeight);
  });

  onCleanup(() => {
    window.removeEventListener("resize", updateHeight);
  });

  createEffect(() => {
    updateHeight();
  });

  return (
    <>
      <Title>{user.name} | Arche</Title>

      <div
        class="pointer-events-none absolute inset-x-0 h-full"
        aria-hidden="true"
        role="presentation"
      >
        <div
          ref={userBorderRef}
          class="absolute mt-24 w-screen border-y-[0.5px] border-slate-300 dark:border-slate-700"
        ></div>
      </div>
      <div class="mx-auto flex w-screen max-w-256 px-8 md:px-16">
        <div class="min-h-screen w-full border-x-[0.5px] border-slate-300 px-8 pt-24 pb-12 dark:border-slate-700">
          <article ref={userRef} class="flex flex-col gap-4 py-8">
            <h1 class="text-3xl font-bold">{user.name}</h1>
            <div>
              <h2 class="text-base-content/50 cursor-default py-1 text-xl font-bold">
                Papers
              </h2>
              <div class="flex flex-col gap-4 pl-2">
                <div>
                  <h3 class="text-base-content/75 cursor-default py-1 font-semibold">
                    First-Author Papers
                  </h3>
                  <ul class="flex flex-col gap-4 py-2 pl-2">
                    <For each={user.lead_authored_papers}>
                      {(paper) => (
                        <li>
                          <Link
                            to={"/abs/$paperId"}
                            params={{ paperId: paper.id }}
                            class="text-primary active:text-primary/50 hover:underline"
                          >
                            {paper.title}
                          </Link>
                        </li>
                      )}
                    </For>
                  </ul>
                </div>
                <div>
                  <ul></ul>
                </div>
              </div>
            </div>
          </article>
        </div>
      </div>
    </>
  );
}
