import { Title } from "@solidjs/meta";
import { createFileRoute, Link } from "@tanstack/solid-router";
import type { JSXElement } from "solid-js";
import { backend } from "../../declarations/backend";
import { matchResult } from "../../utils/matchResult";

export const Route = createFileRoute("/article/$articleId")({
  loader: ({ params }) => backend.fetch_article(params.articleId),
  component: RouteComponent,
});

function RouteComponent(): JSXElement {
  let data = Route.useLoaderData();

  return matchResult(data(), {
    ok: (article) => (
      <main class="grid place-content-center">
        <Title>{article.title} | Arche</Title>

        <div class="grid grid-cols-5 auto-rows-min gap-4 w-screen max-w-[72rem] min-h-screen p-8">
          <article class="col-span-4 grid grid-cols-5 gap-4 border border-[0.5px] border-gray-500 rounded-lg py-4 px-8">
            <div class="col-span-5 h-auto">
              <h1 class="text-2xl font-bold">{article.title}</h1>
              <Link
                class="text-blue-600"
                to={"/user/" + article.lead_author[0]}
              >
                {article.lead_author[1]}
              </Link>
              <p class="text-gray-600">{article.summary}</p>
            </div>
            <div class="col-span-5">
              <h2 class="text-xl font-bold">Content</h2>
              <p class="text-gray-600">PDF</p>
              <p class="text-gray-600">Typst</p>
            </div>
          </article>
        </div>
      </main>
    ),
    err: (err) => (
      <main class="min-h-screen flex flex-col items-center justify-center">
        <Title>Error | Arche</Title>

        <h1>Error: {err}</h1>
      </main>
    ),
  });
}
