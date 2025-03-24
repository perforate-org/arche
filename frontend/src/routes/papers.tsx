import { createFileRoute } from '@tanstack/solid-router'

export const Route = createFileRoute('/papers')({
  component: RouteComponent,
})

function RouteComponent() {
  return <div>Hello "/papers"!</div>
}
