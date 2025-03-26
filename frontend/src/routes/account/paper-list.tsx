import { createFileRoute } from "@tanstack/solid-router";
import { redirect, Link, useRouter } from "@tanstack/solid-router";
import {
  queryOptions,
  createQuery,
  type QueryClient,
} from "@tanstack/solid-query";
import { pushAlert } from "../../contexts/alert";
import { type AuthStoreType } from "../../contexts/auth";
import type { Principal } from "@dfinity/principal";
import {
  createEffect,
  onMount,
  onCleanup,
  createMemo,
  For,
  createSignal,
} from "solid-js";
import type { User } from "../../declarations/backend/backend.did";
import { matchResult } from "../../utils/result";
import { Title } from "@solidjs/meta";
import { router } from "../../main";

const accountPapersQueryOptions = (
  auth: AuthStoreType,
  principal: Principal | null,
) =>
  queryOptions({
    queryKey: ["account", "paperList", { principal }],
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
  loader: ({ context }) =>
    context.queryClient.ensureQueryData(
      accountPapersQueryOptions(context.auth, context.auth.principal),
    ),
  component: RouteComponent,
});

function RouteComponent() {
  const context = Route.useRouteContext();
  const userQuery = createQuery(() =>
    accountPapersQueryOptions(context().auth, context().auth.principal),
  );
  const data = createMemo(() => userQuery.data);

  return (
    <main class="flex min-h-screen flex-col items-center justify-center">
      {context().auth.isAuthenticated ? (
        matchResult(data()!, {
          ok: (user) => (
            <List
              user={user}
              auth={context().auth}
              queryClient={context().queryClient}
            />
          ),
          err: (err) => (
            <>
              <Title>Error | Arche</Title>
              <h1>Error</h1>
              <p>{err}</p>
            </>
          ),
        })
      ) : (
        <p>Please log in to access your account settings.</p>
      )}
    </main>
  );
}

function List({
  user,
  auth,
  queryClient,
}: {
  user: User;
  auth: AuthStoreType;
  queryClient: QueryClient;
}) {
  let router = useRouter();
  const [paperToDelete, setPaperToDelete] = createSignal<string | null>(null);
  let modalRef: HTMLDialogElement | undefined;

  let listRef: HTMLElement | undefined;
  let listBorderRef: HTMLDivElement | undefined;

  function updateHeight() {
    if (listRef && listBorderRef) {
      listBorderRef.style.height = `${listRef.offsetHeight}px`;
    }
  }

  onMount(() => {
    window.addEventListener("resize", updateHeight);
  });

  onCleanup(() => {
    window.removeEventListener("resize", updateHeight);
    // Close modal if component unmounts
    if (modalRef) modalRef.close();
  });

  createEffect(() => {
    updateHeight();
  });

  async function deletePaper(paperId: string) {
    let response = await auth.backend.delete_paper(paperId);
    matchResult(response, {
      ok: () => {
        pushAlert({
          type: "success",
          message: "Paper deleted successfully",
        });
        // Deactivate and reacquire query after deleting papers
        queryClient.invalidateQueries({
          queryKey: ["account", "paperList"],
        });
        queryClient.invalidateQueries({
          queryKey: ["paperList"],
        });
      },
      err: (error) => {
        pushAlert({
          type: "error",
          message: `Failed to delete paper: ${error}`,
        });
      },
    });
    // Close the modal after deletion attempt
    if (modalRef) modalRef.close();
    setPaperToDelete(null);
  }

  return (
    <>
      <div
        class="pointer-events-none absolute inset-x-0 h-full"
        aria-hidden="true"
        role="presentation"
      >
        <div
          ref={listBorderRef}
          class="absolute mt-24 w-screen border-y-[0.5px] border-slate-300 dark:border-slate-700"
        ></div>
      </div>

      {/* Confirmation Modal */}
      <dialog ref={modalRef} class="modal">
        <div class="modal-box">
          <h3 class="text-lg font-bold">Confirm Deletion</h3>
          <p class="py-4">
            Are you sure you want to delete this paper? This action cannot be
            undone.
          </p>
          <div class="modal-action">
            <button
              class="btn btn-outline"
              onClick={() => {
                if (modalRef) modalRef.close();
                setPaperToDelete(null);
              }}
            >
              Cancel
            </button>
            <button
              class="btn btn-error"
              onClick={() => paperToDelete() && deletePaper(paperToDelete()!)}
            >
              Delete
            </button>
          </div>
        </div>
        <form method="dialog" class="modal-backdrop">
          <button onClick={() => setPaperToDelete(null)}>close</button>
        </form>
      </dialog>

      <div class="mx-auto flex w-screen max-w-256 px-8 md:px-16">
        <div class="min-h-screen w-full border-x-[0.5px] border-slate-300 pt-12 pb-12 dark:border-slate-700">
          <div class="flex h-12 items-center justify-end px-2 md:px-4">
            <button
              class="btn btn-primary btn-xs"
              onClick={async () => handleNewPaper(auth, router)}
            >
              New Paper
            </button>
          </div>
          <article ref={listRef} class="flex flex-col gap-8 p-8 md:p-16">
            <h1 class="text-base-content/50 cursor-default text-3xl font-bold">
              My Papers
            </h1>
            <ul class="flex flex-col gap-4">
              <For each={user.lead_authored_papers}>
                {(paper) => (
                  <li>
                    <div class="flex items-center justify-between">
                      <Link
                        to={"/abs/$paperId"}
                        params={{ paperId: paper.id }}
                        class="group text-primary active:text-primary/50 text-bold text-xl hover:underline"
                      >
                        {paper.title}
                        <span class="text-primary/75 group-active:text-primary/50 text-[1.2rem]">
                          {": " + paper.id}
                        </span>
                      </Link>
                      <div class="join">
                        <button
                          class="btn btn-xs join-item"
                          onClick={() => {
                            router.navigate({
                              to: "/account/edit",
                              search: { id: paper.id },
                            });
                          }}
                        >
                          Edit
                        </button>
                        <button
                          class="btn btn-xs join-item"
                          onClick={() => {
                            setPaperToDelete(paper.id);
                            if (modalRef) modalRef.showModal();
                          }}
                        >
                          Delete
                        </button>
                      </div>
                    </div>
                  </li>
                )}
              </For>
            </ul>
          </article>
        </div>
      </div>
    </>
  );
}

async function handleNewPaper(auth: AuthStoreType, routerProp: typeof router) {
  let paperId = await auth.backend.create_draft();
  routerProp.navigate({
    to: "/account/edit",
    search: { id: paperId },
  });
}
