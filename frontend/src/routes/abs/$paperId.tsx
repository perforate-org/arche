import { Title } from "@solidjs/meta";
import { createMemo } from "solid-js";
import { createFileRoute, Link } from "@tanstack/solid-router";
import { queryOptions, createQuery } from "@tanstack/solid-query";
import type { JSXElement } from "solid-js";
import { matchResult } from "../../utils/matchResult";
import { backend } from "../../declarations/backend";

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
    <main class="mt-12 grid place-content-center">
      {matchResult(data()!, {
        ok: (paper) => (
          <>
            <Title>{paper.title} | Arche</Title>

            <div class="grid w-screen max-w-[72rem] auto-rows-min grid-cols-5 gap-4 p-8">
              <article class="col-span-4 grid grid-cols-5 gap-4 rounded-lg border border-[0.5px] border-gray-500 px-8 py-4">
                <div class="col-span-5 h-auto">
                  <h1 class="text-2xl font-bold">{paper.title}</h1>
                  <Link
                    class="text-blue-600 transition-colors duration-100 hover:underline active:text-blue-400 active:underline dark:text-blue-500 dark:active:text-blue-700"
                    to={"/users/" + paper.lead_author[0]}
                  >
                    {paper.lead_author[1]}
                  </Link>
                  <p class="text-gray-600">{paper.ab}</p>
                </div>
                <div class="col-span-5">
                  <h2 class="text-xl font-bold">Content</h2>
                  <p class="text-gray-600">PDF</p>
                  <p class="text-gray-600">Typst</p>
                </div>
              </article>
            </div>
          </>
        ),
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
