import { Title } from "@solidjs/meta";
import { createMemo, createEffect, onCleanup, onMount } from "solid-js";
import { createFileRoute, Link } from "@tanstack/solid-router";
import { queryOptions, createQuery } from "@tanstack/solid-query";
import type { JSXElement } from "solid-js";
import { matchResult } from "../../utils/result";
import { backend } from "../../declarations/backend";
import type { Paper } from "../../declarations/backend/backend.did";
import { getContent } from "../../features/paper";

const paperQueryOptions = (actor: typeof backend, paperId: string) =>
  queryOptions({
    queryKey: ["paper", { paperId }],
    queryFn: async () => {
      return await actor.fetch_paper(paperId);
    },
    staleTime: 1000 * 60 * 5, // 5 minutes
    throwOnError: true,
  });

export const Route = createFileRoute("/abs/$paperId")({
  loader: ({ params, context }) =>
    context.queryClient.ensureQueryData(
      paperQueryOptions(context.auth.backend, params.paperId),
    ),
  component: RouteComponent,
});

function RouteComponent(): JSXElement {
  const context = Route.useRouteContext();
  const params = Route.useParams();
  const paperQuery = createQuery(() =>
    paperQueryOptions(context().auth.backend, params().paperId),
  );
  const data = createMemo(() => paperQuery.data);

  return (
    <main class="grid place-content-center">
      {matchResult(data()!, {
        ok: (paper) => <Abstract paper={paper} />,
        err: (err) => (
          <>
            <Title>Error | Arche</Title>

            <h1>Error: {err}</h1>
          </>
        ),
      })}
    </main>
  );
}

function Abstract({ paper }: { paper: Paper }) {
  let articleRef: HTMLElement | undefined;
  let articleBorderRef: HTMLDivElement | undefined;

  function updateHeight() {
    if (articleRef && articleBorderRef) {
      articleBorderRef.style.height = `${articleRef.offsetHeight}px`;
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

  const content = getContent(paper.content);

  return (
    <>
      <Title>{paper.title} | Arche</Title>

      <div
        class="pointer-events-none absolute inset-x-0 h-full"
        aria-hidden="true"
        role="presentation"
      >
        <div
          ref={articleBorderRef}
          class="absolute mt-24 w-screen border-y-[0.5px] border-slate-300 dark:border-slate-700"
        ></div>
      </div>
      <div class="mx-auto flex w-screen max-w-256 px-8 md:px-16">
        <div class="min-h-screen w-full border-x-[0.5px] border-slate-300 pt-24 pb-12 dark:border-slate-700">
          <article
            ref={articleRef}
            class="flex w-full flex-1 flex-col gap-4 px-8 py-8 md:px-16 md:py-16"
          >
            <div class="flex flex-col gap-2">
              <h1 class="text-3xl font-bold">{paper.title}</h1>
              <Link
                class="text-primary active:text-primary/50 w-fit transition duration-100 hover:underline active:underline"
                to={"/users/$userId"}
                params={{ userId: paper.lead_author[0] }}
              >
                {paper.lead_author[1]}
              </Link>
              <p class="text-base-content/75">{paper.ab}</p>
            </div>
            <div class="flex-1">
              <h2 class="text-base-content/50 cursor-default text-2xl font-bold">
                Content
              </h2>
              <div class="text-base-content my-2 flex flex-col gap-4">
                {content}
              </div>
            </div>
          </article>
        </div>
      </div>
    </>
  );
}
