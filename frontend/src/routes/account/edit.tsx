import { createFileRoute } from "@tanstack/solid-router";
import { createResource, createSignal, Show } from "solid-js";
import type { Paper, Result_2 } from "../../declarations/backend/backend.did";
import { matchResult } from "../../utils/result";
import { Title } from "@solidjs/meta";
import type { RouterContext } from "../__root";
import { createForm } from "@tanstack/solid-form";
import type { AnyFieldApi } from "@tanstack/solid-form";
import type { AuthStoreType } from "../../contexts/auth";
import { pushAlert } from "../../contexts/alert";
import type { PaperContents } from "../../declarations/backend/backend.did";

type EditSearch = {
  id: string;
};

async function handleResource(
  context: RouterContext,
  id: string,
): Promise<Result_2> {
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
  return (
    <>
      <Title>{paper.title} | Arche</Title>

      <div
        class="pointer-events-none absolute inset-x-0 top-0 h-full"
        aria-hidden="true"
        role="presentation"
      >
        <div class="absolute mt-24 w-screen border-t-[0.5px] border-slate-300 dark:border-slate-700"></div>
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
  // Determine the initial PDF source type (HTTP link or File upload)
  const initialPdfSourceType = paper.content.pdf[0]
    ? "Http" in paper.content.pdf[0]
      ? "link"
      : "file"
    : "file";

  const [contentSignal, setContentSignal] = createSignal<PaperContents>({
    ...paper.content,
  });
  const [pdfSourceType, setPdfSourceType] = createSignal<"file" | "link">(
    initialPdfSourceType,
  );
  const [pdfFileName, setPdfFileName] = createSignal<string>(
    paper.content.pdf[0] && "Raw" in paper.content.pdf[0]
      ? paper.content.pdf[0].Raw.name
      : "",
  );
  const [pdfUrl, setPdfUrl] = createSignal<string>(
    paper.content.pdf[0] && "Http" in paper.content.pdf[0]
      ? paper.content.pdf[0].Http
      : "",
  );
  const [hasUploadedPdf, setHasUploadedPdf] = createSignal<boolean>(
    paper.content.pdf[0] && "Raw" in paper.content.pdf[0] ? true : false,
  );

  const form = createForm(() => ({
    defaultValues: {
      ...paper,
    } as Paper,
    onSubmit: async (value) => {
      try {
        const title = value.value.title;
        const abstract = value.value.ab;
        let updatedContent = { ...value.value.content };

        // Handle PDF content
        if (pdfSourceType() === "link") {
          const url = pdfUrl();
          if (url) {
            // Validate URL
            if (!url.startsWith("http://") && !url.startsWith("https://")) {
              throw new Error("URL must start with http:// or https://");
            }

            try {
              const urlObj = new URL(url);
              const extension = urlObj.pathname.split(".").pop()?.toLowerCase();

              if (extension !== "pdf") {
                throw new Error("URL must point to a PDF file");
              }

              updatedContent.pdf[0] = { Http: url };
            } catch (error) {
              throw new Error("Invalid URL format");
            }
          } else {
            // If URL field is empty, remove the PDF
            updatedContent.pdf = [];
          }
        } else if (pdfSourceType() === "file") {
          // Keep the existing PDF file if no new one was uploaded
          if (!hasUploadedPdf() && paper.content.pdf[0]) {
            updatedContent.pdf[0] = paper.content.pdf[0];
          }
          // If no PDF is available, set to empty
          if (!hasUploadedPdf() && !paper.content.pdf[0]) {
            updatedContent.pdf = [];
          }
          // If a new PDF was uploaded, it's already in the form value
        }

        const updatedPaper = {
          ...paper,
          title,
          ab: abstract,
          content: updatedContent,
        };

        let response = await auth.backend.update_paper(updatedPaper);
        matchResult(response, {
          ok: () => {
            pushAlert({
              type: "success",
              message: "Paper saved successfully",
            });

            // Update the form's default values
            form.reset(updatedPaper);
          },
          err: (error) => {
            pushAlert({
              type: "error",
              message: error,
            });
          },
        });
      } catch (error) {
        pushAlert({
          type: "error",
          message: error instanceof Error ? error.message : "An error occurred",
        });
      }
    },
  }));

  // Use the format as the active tab
  const [activeTab, setActiveTab] = createSignal<"pdf" | "text">(
    paper.content.pdf[0] ? "pdf" : "text",
  );

  const handleFileUpload = (e: Event) => {
    const input = e.target as HTMLInputElement;
    if (input.files && input.files.length > 0) {
      const file = input.files[0];
      const extension = file.name.split(".").pop()?.toLowerCase();

      if (extension !== "pdf") {
        pushAlert({
          type: "error",
          message: "Only PDF files are supported",
        });
        input.value = "";
        return;
      }

      setPdfFileName(file.name);

      // Handle PDF files as binary
      const reader = new FileReader();
      reader.onload = (event) => {
        if (event.target?.result) {
          const result = event.target.result as ArrayBuffer;
          const contentCopy = { ...contentSignal() };

          // ファイル名も保存するように更新
          contentCopy.pdf[0] = {
            Raw: {
              content: new Uint8Array(result),
              name: file.name,
            },
          };

          setContentSignal(contentCopy);
          setHasUploadedPdf(true);

          // Update the form value
          form.setFieldValue("content", contentCopy);
        }
      };
      reader.onerror = () => {
        pushAlert({
          type: "error",
          message: "Failed to read the file",
        });
      };
      reader.readAsArrayBuffer(file);
    }
  };

  const handlePdfUrlChange = (url: string) => {
    setPdfUrl(url);

    // Update content in form
    const contentCopy = { ...contentSignal() };
    if (url.trim()) {
      contentCopy.pdf[0] = { Http: url };
    } else {
      contentCopy.pdf = [];
    }
    setContentSignal(contentCopy);
    form.setFieldValue("content", contentCopy);
  };

  const handleTextContentChange = (text: string) => {
    const contentCopy = { ...contentSignal() };
    contentCopy.text[0] = text;
    setContentSignal(contentCopy);
    form.setFieldValue("content", contentCopy);
  };

  return (
    <>
      <div class="flex h-12 items-center justify-end">
        <form.Subscribe
          selector={(state) => ({
            canSubmit: state.canSubmit,
            isSubmitting: state.isSubmitting,
            isDirty: state.isDirty,
          })}
          children={(state) => (
            <button
              type="submit"
              form="accountForm"
              class={`btn btn-xs btn-primary w-16 px-4 ${
                state().isSubmitting || !state().canSubmit || !state().isDirty
                  ? "btn-disabled"
                  : ""
              }`}
              disabled={!state().canSubmit || !state().isDirty}
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
      <div class="z-1 flex flex-row pt-4 pb-8">
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
                    id={field().name}
                    class="input text-base-content invalid:border-error invalid:outline-error w-full text-sm font-normal"
                    type="text"
                    name={field().name}
                    value={field().state.value}
                    placeholder="Type your title"
                    onInput={(e) => field().handleChange(e.target.value)}
                    disabled={form.state.isSubmitting}
                  />
                </div>
              );
            }}
          />
          <form.Field
            name="ab"
            children={(field) => {
              return (
                <div class="flex w-full flex-col gap-2">
                  <div class="flex items-center justify-between">
                    <label
                      class="text-base-content/50 text-lg font-bold"
                      for={field().name}
                    >
                      Abstract
                    </label>
                  </div>
                  <textarea
                    id={field().name}
                    class="textarea text-base-content invalid:border-error invalid:outline-error w-full text-sm font-normal"
                    name={field().name}
                    value={field().state.value}
                    onInput={(e) => field().handleChange(e.target.value)}
                    disabled={form.state.isSubmitting}
                    rows={5}
                  />
                </div>
              );
            }}
          />
          <div class="flex w-full flex-col gap-2">
            <div class="flex items-center justify-between">
              <label class="text-base-content/50 text-lg font-bold">
                Content
              </label>
            </div>
            <div class="tabs tabs-lift">
              <input
                type="radio"
                name="content_tabs"
                class="tab"
                aria-label="Text"
                checked={activeTab() === "text"}
                onClick={() => setActiveTab("text")}
              />
              {activeTab() === "text" && (
                <div class="tab-content bg-base-100 border-base-300">
                  <textarea
                    class="textarea text-base-content invalid:border-error invalid:outline-error textarea-md rounded-field min-h-full w-full border-0 font-normal"
                    value={contentSignal().text[0] || ""}
                    onInput={(e) => handleTextContentChange(e.target.value)}
                    disabled={form.state.isSubmitting}
                    rows={10}
                  />
                </div>
              )}

              <input
                type="radio"
                name="content_tabs"
                class="tab"
                aria-label="PDF"
                checked={activeTab() === "pdf"}
                onClick={() => setActiveTab("pdf")}
              />
              {activeTab() === "pdf" && (
                <div class="tab-content bg-base-100 border-base-300 p-6">
                  <div class="tabs tabs-sm mb-4">
                    <a
                      class={`tab tab-bordered ${pdfSourceType() === "file" ? "tab-active" : ""}`}
                      onClick={() => setPdfSourceType("file")}
                    >
                      Upload File
                    </a>
                    <a
                      class={`tab tab-bordered ${pdfSourceType() === "link" ? "tab-active" : ""}`}
                      onClick={() => setPdfSourceType("link")}
                    >
                      Link
                    </a>
                  </div>

                  <Show when={pdfSourceType() === "file"}>
                    <div class="flex flex-col gap-3">
                      <input
                        type="file"
                        class="file-input w-full"
                        accept=".pdf"
                        onChange={handleFileUpload}
                        disabled={form.state.isSubmitting}
                      />

                      <Show
                        when={
                          hasUploadedPdf() ||
                          (paper.content.pdf[0] &&
                            "Raw" in paper.content.pdf[0])
                        }
                      >
                        <div class="flex items-center gap-2 text-sm">
                          <span class="badge badge-success">PDF selected</span>
                          <span>{pdfFileName()}</span>
                        </div>
                      </Show>
                    </div>
                  </Show>

                  <Show when={pdfSourceType() === "link"}>
                    <div class="flex flex-col gap-2">
                      <input
                        class="input text-base-content invalid:border-error invalid:outline-error w-full font-normal"
                        type="url"
                        placeholder="https://example.com/document.pdf"
                        value={pdfUrl()}
                        onInput={(e) => handlePdfUrlChange(e.target.value)}
                        disabled={form.state.isSubmitting}
                      />
                      <p class="text-base-content/60 text-xs">
                        Enter the direct URL to a PDF file (must end with .pdf)
                      </p>
                    </div>
                  </Show>
                </div>
              )}
            </div>
          </div>
        </form>
      </div>
    </>
  );
}
