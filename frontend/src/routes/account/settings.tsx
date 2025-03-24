import { Title } from "@solidjs/meta";
import { createMemo, createEffect } from "solid-js";
import { createFileRoute, redirect } from "@tanstack/solid-router";
import { queryOptions, createQuery } from "@tanstack/solid-query";
import { matchResult } from "../../utils/matchResult";
import type { AuthStoreType } from "../../contexts/auth";
import type { User } from "../../declarations/backend/backend.did";
import { createForm } from "@tanstack/solid-form";
import type { AnyFieldApi } from "@tanstack/solid-form";
import { fromOption } from "../../utils/matchOption";
import { validateId, validateName } from "../../features/account";
import { pushAlert } from "../../contexts/alert";

const accountQueryOptions = (auth: AuthStoreType) =>
  queryOptions({
    queryKey: ["account", "settings"],
    queryFn: async () => {
      return await auth.backend.fetch_caller();
    },
    throwOnError: true,
  });

export const Route = createFileRoute("/account/settings")({
  beforeLoad: async ({ context, location }) => {
    while (context.auth.isInitializing) {
      await new Promise((resolve) => setTimeout(resolve, 20));
    }
    if (!context.auth.isAuthenticated) {
      pushAlert({
        type: "warning",
        message: "You must be logged in to access account settings.",
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
    context.queryClient.ensureQueryData(accountQueryOptions(context.auth)),
  component: RouteComponent,
});

function RouteComponent() {
  const context = Route.useRouteContext();
  const userQuery = createQuery(() => accountQueryOptions(context().auth));
  const data = createMemo(() => userQuery.data);

  return (
    <main class="min-h-screen">
      {matchResult(data()!, {
        ok: (user) => <Account user={user} auth={context().auth} />,
        err: (err) => (
          <>
            <Title>Error | Arche</Title>

            <h1>Error</h1>
            <p>{err}</p>
          </>
        ),
      })}
    </main>
  );
}

function Account({ user, auth }: { user: User; auth: AuthStoreType }) {
  let menuRef: HTMLUListElement | undefined;
  let contentRef: HTMLDivElement | undefined;
  let contentBorderRef: HTMLDivElement | undefined;

  const setContentRef = (el: HTMLDivElement) => {
    contentRef = el;
    updateBorderHeight();
  };

  const updateBorderHeight = () => {
    if (menuRef && contentBorderRef) {
      const menuHeight = menuRef.offsetHeight;
      const contentHeight = contentRef ? contentRef.offsetHeight : 0;
      contentBorderRef.style.height = `${Math.max(menuHeight, contentHeight)}px`;
    }
  };

  createEffect(() => {
    updateBorderHeight();
  });

  return (
    <>
      <Title>Account | Arche</Title>

      <div
        class="pointer-events-none absolute inset-x-0 h-full"
        aria-hidden="true"
        role="presentation"
      >
        <div
          ref={contentBorderRef}
          class="absolute mt-24 w-screen border-y-[0.5px] border-slate-300 dark:border-slate-700"
        ></div>
      </div>
      <div class="mx-auto flex w-screen max-w-256 px-8 md:px-16">
        <div class="h-full min-h-screen w-32 border-x-[0.5px] border-slate-300 pt-24 text-xs md:w-48 md:text-sm dark:border-slate-700">
          <ul
            ref={menuRef}
            class="menu w-full px-1 py-4 text-xs md:px-2 md:text-sm"
          >
            <li>
              <button class="px-2 md:px-3">Account Settings</button>
            </li>
          </ul>
        </div>
        <div class="flex h-full min-h-screen flex-1 flex-col border-r-[0.5px] border-slate-300 px-4 pt-12 dark:border-slate-700">
          <Form user={user} auth={auth} setContentRef={setContentRef} />
        </div>
      </div>
    </>
  );
}

function FieldInfo({ field }: { field: () => AnyFieldApi }) {
  return (
    <>
      {field().state.meta.isTouched && field().state.meta.errors.length ? (
        <em class="text-error text-xs">
          {field().state.meta.errors.join(",")}
        </em>
      ) : null}
      {!field().state.meta.errors.length && field().state.meta.isValidating ? (
        <span class="loading loading-ring loading-xs opacity-50" />
      ) : null}
    </>
  );
}

function Form({
  user,
  auth,
  setContentRef,
}: {
  user: User;
  auth: AuthStoreType;
  setContentRef: (el: HTMLDivElement) => void;
}) {
  let idInputRef: HTMLInputElement | undefined;
  let nameInputRef: HTMLInputElement | undefined;

  let baseId: string = fromOption(user.id) ?? "";
  const form = createForm(() => ({
    defaultValues: {
      id: baseId,
      name: user.name,
    },
    onSubmit: async (value) => {
      const id = value.value.id;
      const name = value.value.name;
      const id_optional: [string] | [] = id ? [id] : [];
      const updatedUser = {
        ...user,
        id: id_optional,
        name,
      };
      let response = await auth.backend.update_caller(updatedUser);
      matchResult(response, {
        ok: () => {
          user = updatedUser;
          baseId = id;
          pushAlert({
            type: "success",
            message: "Account updated successfully",
          });
        },
        err: (error) => {
          pushAlert({
            type: "error",
            message: error,
          });
        },
      });
      form.reset({
        id,
        name,
      });
    },
  }));

  return (
    <>
      <div class="flex h-12 items-center justify-end">
        <form.Subscribe
          selector={(state) => {
            const isDirty =
              state.values.id === form.options.defaultValues!.id &&
              state.values.name === form.options.defaultValues!.name;
            return {
              canSubmit: state.canSubmit && !isDirty,
              isSubmitting: state.isSubmitting,
            };
          }}
          children={(state) => (
            <button
              type="submit"
              form="accountForm"
              class={`btn btn-xs btn-primary w-16 px-4 ${
                state().isSubmitting || !state().canSubmit ? "btn-disabled" : ""
              }`}
              aria-disabled={!state().canSubmit}
            >
              {state().isSubmitting ? (
                <progress
                  class="progress h-1 w-8 opacity-50"
                  aria-label="Saving"
                />
              ) : (
                "Save"
              )}
            </button>
          )}
        />
      </div>
      <div ref={setContentRef} class={`z-1 flex flex-row pt-4 pb-8`}>
        <form
          id="accountForm"
          class="flex w-full flex-col gap-4"
          onSubmit={(e) => {
            e.preventDefault();
            e.stopPropagation();
            form.handleSubmit();
          }}
        >
          <form.Field
            name="id"
            validators={{
              onChange: ({ value }) => {
                const error = value ? validateId(value) : undefined;
                if (idInputRef) {
                  if (error) {
                    idInputRef.setCustomValidity(error);
                  } else {
                    idInputRef.setCustomValidity("");
                  }
                }
                return error;
              },
              onChangeAsyncDebounceMs: 500,
              onChangeAsync: async ({ value }) => {
                if (value && value != baseId) {
                  const exists = matchResult(
                    await auth.backend.user_exists_by_id(value),
                    {
                      ok: (exists) => exists,
                      err: () => false,
                    },
                  );
                  if (exists) {
                    if (idInputRef) {
                      idInputRef.setCustomValidity("ID already exists");
                    }
                    return "ID already exists";
                  }
                }
              },
            }}
            children={(field) => {
              return (
                <div class="flex w-full flex-col gap-2">
                  <div class="flex items-center justify-between">
                    <label
                      class="text-base-content/50 text-lg font-bold"
                      for={field().name}
                    >
                      ID
                    </label>
                    <FieldInfo field={field} />
                  </div>
                  <input
                    ref={idInputRef}
                    id={field().name}
                    class="input text-base-content invalid:border-error invalid:outline-error w-full text-sm font-normal"
                    name={field().name}
                    value={field().state.value}
                    placeholder={"p_" + auth.principal?.toString()}
                    onBlur={field().handleBlur}
                    onInput={(e) => field().handleChange(e.target.value)}
                    disabled={form.state.isSubmitting}
                  />
                </div>
              );
            }}
          />
          <form.Field
            name="name"
            validators={{
              onChange: ({ value }) => {
                const error = validateName(value);
                if (nameInputRef) {
                  if (error) {
                    nameInputRef.setCustomValidity(error);
                  } else {
                    nameInputRef.setCustomValidity("");
                  }
                }
                return error;
              },
            }}
            children={(field) => {
              return (
                <div class="flex w-full flex-col gap-2">
                  <div class="flex items-center justify-between">
                    <label
                      class="text-base-content/50 text-lg font-bold"
                      for={field().name}
                    >
                      Name
                    </label>
                    <FieldInfo field={field} />
                  </div>
                  <input
                    ref={nameInputRef}
                    id={field().name}
                    class="input text-base-content invalid:border-error invalid:outline-error w-full text-sm font-normal"
                    type="text"
                    name={field().name}
                    value={field().state.value}
                    placeholder="Type your name"
                    onInput={(e) => field().handleChange(e.target.value)}
                    disabled={form.state.isSubmitting}
                  />
                </div>
              );
            }}
          />
        </form>
      </div>
    </>
  );
}
