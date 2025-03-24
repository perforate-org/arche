import { MetaProvider } from "@solidjs/meta";
import { QueryClient, QueryClientProvider } from "@tanstack/solid-query";
import { RouterProvider, createRouter } from "@tanstack/solid-router";
import { render } from "solid-js/web";
import { AuthProvider, useAuth } from "./contexts/auth";
import { GlobalLoadingIndicator } from "./components/loadingIndicator";
import type { AuthStoreType } from "./contexts/auth";
import { AlertsProvider } from "./contexts/alert";

import { routeTree } from "./routeTree.gen";
import "./styles.css";

export interface RouterContext {
  queryClient: QueryClient;
  auth: AuthStoreType;
}

const queryClient = new QueryClient();

export const router = createRouter({
  routeTree,
  context: {
    queryClient,
    auth: undefined!,
  },
  defaultPreload: "intent",
  scrollRestoration: true,
  defaultStaleTime: 5000,
  defaultPendingMinMs: 0,
  defaultPendingMs: 0,
  defaultViewTransition: true,
});

declare module "@tanstack/solid-router" {
  interface Register {
    router: typeof router;
  }
}

function InnerApp() {
  const auth = useAuth();

  return (
    <>
      <GlobalLoadingIndicator />
      <RouterProvider router={router} context={{ auth }} />
    </>
  );
}

function App() {
  return (
    <>
      <AuthProvider router={router}>
        <QueryClientProvider client={queryClient}>
          <MetaProvider>
            <AlertsProvider>
              <InnerApp />
            </AlertsProvider>
          </MetaProvider>
        </QueryClientProvider>
      </AuthProvider>
    </>
  );
}

const rootElement = document.getElementById("app");
if (rootElement) {
  render(() => <App />, rootElement);
}
