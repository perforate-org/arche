import { type JSXElement } from "solid-js";
import type { PaperContents } from "../../declarations/backend/backend.did";
import { HttpSource } from "./content/source/http";
import { RawSource } from "./content/source/raw";
import { PlainText } from "./content/plaintext";

export function getContent(content: PaperContents): JSXElement {
  // Show text content, if any
  if (content.text && content.text.length > 0 && content.text[0]) {
    return <PlainText text={content.text[0]} />;
  }

  // If there is PDF content
  if (content.pdf && content.pdf.length > 0) {
    const source = content.pdf[0];

    // Display appropriate components according to PDF source type
    if (source) {
      if ("Http" in source && source.Http) {
        return <HttpSource url={source.Http} />;
      } else if ("Raw" in source && source.Raw) {
        return <RawSource file={source.Raw} />;
      }
    }
  }

  // If there is no content
  return (
    <div class="flex flex-col gap-4 py-4">
      <p class="text-base-content/50 italic">No content available</p>
      <p class="text-base-content/40 text-sm">
        The author has not provided any content for this paper yet.
      </p>
    </div>
  );
}
