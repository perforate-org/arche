import { Title } from "@solidjs/meta";
import { createFileRoute } from "@tanstack/solid-router";

export const Route = createFileRoute("/about")({
  component: RouteComponent,
});

function RouteComponent() {
  return (
    <main>
      <Title>About | Arche</Title>

      <p>Hello "/about"!</p>
    </main>
  );
}
