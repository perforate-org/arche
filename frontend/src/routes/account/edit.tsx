import { createFileRoute } from "@tanstack/solid-router";
import { Link } from "@tanstack/solid-router";
import { createEffect, onMount, onCleanup, createResource } from "solid-js";
import type {
  Paper,
  Result_1,
  User,
} from "../../declarations/backend/backend.did";
import { matchResult } from "../../utils/result";
import { Title } from "@solidjs/meta";
import type { RouterContext } from "../__root";
import { createForm } from "@tanstack/solid-form";
import type { AnyFieldApi } from "@tanstack/solid-form";
import type { AuthStoreType } from "../../contexts/auth";
import { pushAlert } from "../../contexts/alert";
import { fromOption } from "../../utils/option";
import { validateId, validateName } from "../../features/account";

type EditSearch = {
  id: string;
};

async function handleResource(
  context: RouterContext,
  id: string,
): Promise<Result_1> {
  return await context.auth.backend.fetch_paper_as_author(id);
}

export const Route = createFileRoute("/account/edit")({
  validateSearch: (search: Record<string, unknown>): EditSearch => {
    return {
      id: (search.id as string) || "",
    };
  },
  component: RouteComponent,
});

function RouteComponent() {
  const context = Route.useRouteContext();
  const search = Route.useSearch();

  const [data, _] = createResource(() =>
    handleResource(context(), search().id),
  );

  return (
    <main class="flex min-h-screen flex-col items-center justify-center">
      {data()
        ? matchResult(data()!, {
            ok: (paper) => <Editor paper={paper} auth={context().auth} />,
            err: (err) => (
              <>
                <Title>Error | Arche</Title>
                <h1>Error</h1>
                <p>{err}</p>
              </>
            ),
          })
        : null}
    </main>
  );
}

function Editor({ paper, auth }: { paper: Paper; auth: AuthStoreType }) {
  let articleRef: HTMLElement | undefined;
  let articleBorderRef: HTMLDivElement | undefined;

  function updateHeight() {
    if (articleRef && articleBorderRef) {
      articleBorderRef.style.height = `${articleRef.offsetHeight}px`;
    }
  }

  onMount(() => {
    window.addEventListener("resize", updateHeight);
  });

  onCleanup(() => {
    window.removeEventListener("resize", updateHeight);
  });

  createEffect(() => {
    updateHeight();
  });

  return (
    <>
      <Title>{paper.title} | Arche</Title>

      <div
        class="pointer-events-none absolute inset-x-0 h-full"
        aria-hidden="true"
        role="presentation"
      >
        <div
          ref={articleBorderRef}
          class="absolute mt-24 w-screen border-y-[0.5px] border-slate-300 dark:border-slate-700"
        ></div>
      </div>
      <div class="mx-auto flex w-screen max-w-256 px-8 md:px-16">
        <div class="min-h-screen w-full border-x-[0.5px] border-slate-300 px-8 pt-12 pb-12 dark:border-slate-700">
          <Form paper={paper} auth={auth} />
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

function Form({ paper, auth }: { paper: Paper; auth: AuthStoreType }) {
  let titleInputRef: HTMLInputElement | undefined;
  let contentInputRef: HTMLTextAreaElement | undefined;

  paper.content.content_format = { Text: null };
  let rawContent = paper.content;
  if (!("Raw" in rawContent.content_source)) {
    rawContent.content_source = { Raw: [] };
  }
  const textDecoder = new TextDecoder();
  const data =
    rawContent.content_source.Raw instanceof Uint8Array
      ? rawContent.content_source.Raw
      : new Uint8Array(rawContent.content_source.Raw);
  let content_str = textDecoder.decode(data);

  const form = createForm(() => ({
    defaultValues: {
      ...paper,
      content: content_str,
    },
    onSubmit: async (value) => {
      const title = value.value.title;
      const content = value.value.content;
      const encoder = new TextEncoder();
      const updatedContent = encoder.encode(content);
      paper.content.content_source = { Raw: updatedContent };
      const updatedPaper = {
        ...paper,
        title,
      };
      let response = await auth.backend.update_paper(updatedPaper);
      matchResult(response, {
        ok: () => {
          paper = updatedPaper;
          pushAlert({
            type: "success",
            message: "Paper updated successfully",
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
        ...paper,
        title,
        content,
      });
    },
  }));

  return (
    <>
      <div class="flex h-12 items-center justify-end">
        <form.Subscribe
          selector={(state) => {
            const isDirty = state.values === form.options.defaultValues!;
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
      <div class={`z-1 flex flex-row pt-4 pb-8`}>
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
            name="title"
            children={(field) => {
              return (
                <div class="flex w-full flex-col gap-2">
                  <div class="flex items-center justify-between">
                    <label
                      class="text-base-content/50 text-lg font-bold"
                      for={field().name}
                    >
                      Title
                    </label>
                    <FieldInfo field={field} />
                  </div>
                  <input
                    ref={titleInputRef}
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
          <form.Field
            name="content"
            children={(field) => {
              return (
                <div class="flex w-full flex-col gap-2">
                  <div class="flex items-center justify-between">
                    <label
                      class="text-base-content/50 text-lg font-bold"
                      for={field().name}
                    >
                      Content
                    </label>
                    <FieldInfo field={field} />
                  </div>
                  <textarea
                    ref={contentInputRef}
                    id={field().name}
                    class="textarea text-base-content invalid:border-error invalid:outline-error textarea-md w-full font-normal"
                    name={field().name}
                    value={field().state.value}
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
