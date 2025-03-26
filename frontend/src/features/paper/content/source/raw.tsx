import {
  type JSXElement,
  createSignal,
  createEffect,
  onCleanup,
} from "solid-js";
import type { RawFile } from "../../../../declarations/backend/backend.did";

interface RawSourceProps {
  file: RawFile;
}

export function RawSource({ file }: RawSourceProps): JSXElement {
  const [dataUrl, setDataUrl] = createSignal<string>("");
  const [loading, setLoading] = createSignal<boolean>(true);
  const [error, setError] = createSignal<string | null>(null);

  createEffect(() => {
    if (file && file.content && file.content.length > 0) {
      try {
        // バイナリデータからBlobを作成し、データURLに変換
        const blob = new Blob([new Uint8Array(file.content)], {
          type: "application/pdf",
        });
        const url = URL.createObjectURL(blob);
        setDataUrl(url);
        setLoading(false);

        // コンポーネントがアンマウントされる際にURLをリリース
        onCleanup(() => {
          if (url) URL.revokeObjectURL(url);
        });
      } catch (err) {
        console.error("Error creating data URL:", err);
        setError(`Error: ${err instanceof Error ? err.message : String(err)}`);
        setLoading(false);
      }
    } else {
      setError("Invalid file data");
      setLoading(false);
    }
  });

  return (
    <div class="flex flex-col gap-2">
      <div class="bg-base-200/50 border-base-300 rounded-md border p-4">
        <div class="flex flex-col gap-2">
          <h3 class="font-semibold">{file.name || "PDF Document"}</h3>

          {loading() ? (
            <div class="text-base-content/50 flex items-center gap-2">
              <div class="h-4 w-4 animate-spin rounded-full border-2 border-current border-t-transparent"></div>
              <span>Preparing document...</span>
            </div>
          ) : error() ? (
            <p class="text-sm text-red-500">{error()}</p>
          ) : (
            <div class="mt-2 flex justify-end">
              <a
                href={dataUrl()}
                download={file.name || "document.pdf"}
                class="bg-primary text-primary-content hover:bg-primary/90 mr-2 rounded-md px-4 py-2 text-sm font-medium transition-colors"
              >
                Download
              </a>
              <a
                href={dataUrl()}
                target="_blank"
                rel="noopener noreferrer"
                class="bg-primary text-primary-content hover:bg-primary/90 rounded-md px-4 py-2 text-sm font-medium transition-colors"
              >
                View PDF
              </a>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
