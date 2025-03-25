import { type JSXElement, For } from "solid-js";
import type { ContentFormat } from "../../declarations/backend/backend.did";

export function encodeRawContent(
  format: ContentFormat,
  rawData: Uint8Array | number[],
): JSXElement {
  // Convert number[] to Uint8Array if necessary
  const data =
    rawData instanceof Uint8Array ? rawData : new Uint8Array(rawData);

  if ("Text" in format || "Markdown" in format) {
    const textDecoder = new TextDecoder();
    let str = textDecoder.decode(data);

    return (
      <>
        <For each={str.split("\n\n")}>{(paragraph) => <p>{paragraph}</p>}</For>
      </>
    );
  }
  if ("Pdf" in format) {
    // PDF content is binary and its encoding is not implemented.
    throw new Error("PDF content encoding is not implemented");
  }
  throw new Error("Unsupported content format");
}
