import { createFileRoute } from "@tanstack/solid-router";
import { redirect } from "@tanstack/solid-router";
import { queryOptions, createQuery } from "@tanstack/solid-query";
import { pushAlert } from "../../contexts/alert";
import { type AuthStoreType } from "../../contexts/auth";

const accountPapersQueryOptions = (auth: AuthStoreType) =>
  queryOptions({
    queryKey: ["account", "paperList"],
    queryFn: async () => {
      return await auth.backend.fetch_caller();
    },
    throwOnError: true,
  });

export const Route = createFileRoute("/account/paper-list")({
  beforeLoad: async ({ context, location }) => {
    while (context.auth.isInitializing) {
      await new Promise((resolve) => setTimeout(resolve, 20));
    }
    if (!context.auth.isAuthenticated) {
      pushAlert({
        type: "warning",
        message: "You must be logged in to access account's paper list.",
      });
      throw redirect({
        to: "/",
        search: {
          redirect: location.href,
        },
      });
    }
  },
  component: RouteComponent,
});

function RouteComponent() {
  return (
    <main class="pt-12">
      <p>Hello</p>
    </main>
  );
}
