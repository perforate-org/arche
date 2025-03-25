import { createFileRoute, Link } from "@tanstack/solid-router";
import { queryOptions, createQuery } from "@tanstack/solid-query";
import { backend } from "../declarations/backend";
import { createMemo, For, onMount, createEffect, onCleanup } from "solid-js";
import { Title } from "@solidjs/meta";

const paperListQueryOptions = (actor: typeof backend) =>
  queryOptions({
    queryKey: ["paperList"],
    queryFn: async () => {
      return await actor.fetch_all_paper_summaries();
    },
    staleTime: 1000 * 60 * 5, // 5 minutes
    throwOnError: true,
  });

export const Route = createFileRoute("/papers")({
  loader: ({ context }) =>
    context.queryClient.ensureQueryData(
      paperListQueryOptions(context.auth.backend),
    ),
  component: RouteComponent,
});

function RouteComponent() {
  const context = Route.useRouteContext();
  const userQuery = createQuery(() =>
    paperListQueryOptions(context().auth.backend),
  );
  const papers = createMemo(() => userQuery.data);

  let listRef: HTMLDivElement | undefined;
  let listBorderRef: HTMLDivElement | undefined;

  function updateHeight() {
    if (listRef && listBorderRef) {
      listBorderRef.style.height = `${listRef.offsetHeight}px`;
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
    <main class="flex min-h-screen flex-col items-center justify-center">
      <Title>Paper List | Arche</Title>

      <div
        class="pointer-events-none absolute inset-x-0 h-full"
        aria-hidden="true"
        role="presentation"
      >
        <div
          ref={listBorderRef}
          class="absolute mt-24 w-screen border-y-[0.5px] border-slate-300 dark:border-slate-700"
        ></div>
      </div>
      <div class="mx-auto flex w-screen max-w-256 px-8 md:px-16">
        <div class="min-h-screen w-full border-x-[0.5px] border-slate-300 pt-24 pb-12 dark:border-slate-700">
          <div ref={listRef} class="p-8 md:p-16">
            <h1 class="text-base-content/75 mb-4 text-3xl font-bold">
              Paper List
            </h1>
            <ul class="flex flex-col gap-4">
              <For each={papers()}>
                {(paper) => (
                  <li class="flex flex-col gap-1">
                    <h2>
                      <Link
                        to={"/abs/$paperId"}
                        params={{ paperId: paper.id }}
                        class="group text-primary active:text-primary/50 text-bold text-xl hover:underline"
                      >
                        {paper.title}
                        <span class="text-primary/75 group-active:text-primary/50 text-[1.2rem]">
                          {": " + paper.id}
                        </span>
                      </Link>
                    </h2>
                    <p class="text-sm">
                      by{" "}
                      <Link
                        to={"/users/$userId"}
                        params={{ userId: paper.lead_author_id }}
                        class="text-primary active:text-primary/50 hover:underline"
                      >
                        {paper.lead_author_name
                          ? paper.lead_author_name
                          : paper.lead_author_id}
                      </Link>
                    </p>
                  </li>
                )}
              </For>
            </ul>
          </div>
        </div>
      </div>
    </main>
  );
}
