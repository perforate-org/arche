import { type JSXElement } from "solid-js";

interface HttpSourceProps {
  url: string;
}

export function HttpSource({ url }: HttpSourceProps): JSXElement {
  return (
    <div class="flex flex-col gap-2">
      <div class="bg-base-200 border-base-300 rounded-md border p-4">
        <div class="flex flex-col gap-2">
          <h3 class="font-semibold">PDF Document</h3>
          <p class="text-base-content/70 text-sm break-all">{url}</p>
          <div class="mt-2 flex justify-end">
            <a
              href={url}
              target="_blank"
              rel="noopener noreferrer"
              class="bg-primary text-primary-content hover:bg-primary/90 rounded-md px-4 py-2 text-sm font-medium transition-colors"
            >
              View PDF
            </a>
          </div>
        </div>
      </div>
    </div>
  );
}
