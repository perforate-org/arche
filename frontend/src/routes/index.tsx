import { Title } from "@solidjs/meta";
import { createFileRoute, Link } from "@tanstack/solid-router";

export const Route = createFileRoute("/")({
  component: App,
});

function App() {
  let userId = "anonymous";
  let paperId = "2025-01-0001";

  return (
    <main class="flex min-h-screen flex-col items-center justify-center bg-stone-100 px-8 text-slate-950 md:px-16 dark:bg-stone-900 dark:text-slate-50">
      <Title>Arche: for Preprints</Title>
      <div
        class="pointer-events-none absolute inset-x-8 h-full md:inset-x-16"
        aria-hidden="true"
        role="presentation"
      >
        <div class="absolute h-full w-full bg-slate-50 dark:bg-slate-900">
          <div class="lattice h-full w-full" />
        </div>
        <div class="noise dark:mix-blend-color-luminosity h-full w-full opacity-15 mix-blend-color-dodge" />
      </div>
      <div class="z-2 px-4 text-center">
        <h1 class="mb-4 text-6xl font-bold">Arche</h1>
        <p class="mb-8 font-serif text-xl">Decentralized Preprint Server</p>
        <div class="space-x-4">
          <Link to={"/users/" + userId} class="btn btn-sm">
            Enter as John Doe
          </Link>
          <Link to={"/abs/" + paperId} class="btn btn-sm btn-primary">
            Explore Sample Paper
          </Link>
        </div>
      </div>
    </main>
  );
}
