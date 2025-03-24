import { Outlet, createRootRouteWithContext } from "@tanstack/solid-router";
import { QueryClient } from "@tanstack/solid-query";
// import { TanStackRouterDevtools } from "@tanstack/solid-router-devtools";
import { Header } from "../components/header";
import type { AuthStoreType } from "../contexts/auth";
import { AlertStack } from "../contexts/alert";

export interface RouterContext {
  queryClient: QueryClient;
  auth: AuthStoreType;
}

export const Route = createRootRouteWithContext<RouterContext>()({
  component: RootComponent,
});

function RootComponent() {
  return (
    <>
      <AlertStack />
      <Header />
      <Outlet />
      {/* <TanStackRouterDevtools /> */}
    </>
  );
}
