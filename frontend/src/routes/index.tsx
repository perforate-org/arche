import { Title } from "@solidjs/meta";
import { createFileRoute, Link } from "@tanstack/solid-router";

export const Route = createFileRoute("/")({
  component: App,
});

function App() {
  let user = "anonymous";

  return (
    <main class="min-h-screen dark:bg-slate-950 flex flex-col items-center justify-center text-white lattice">
      <div
        class="absolute w-full h-full noise mix-blend-color-luminosity opacity-15 pointer-events-none"
        aria-hidden="true"
        role="presentation"
      />
      <Title>Arche: for Preprints</Title>
      <div class="text-center px-4">
        <h1 class="text-6xl font-extrabold mb-4">Arche</h1>
        <p class="text-2xl mb-8 font-serif">Decentralized Preprint Server</p>
        <div class="space-x-4">
          <Link
            to={"/user/" + user}
            class="inline-block px-6 py-3 bg-white text-slate-900 font-semibold rounded-lg shadow hover:bg-gray-100 transition"
          >
            Enter as John Doe
          </Link>
          <a
            href="/article/2025-01-0001"
            class="inline-block px-6 py-3 border border-white text-white font-semibold rounded-lg hover:bg-white hover:text-purple-600 transition"
          >
            Explore Sample Article
          </a>
        </div>
      </div>
    </main>
  );
}
