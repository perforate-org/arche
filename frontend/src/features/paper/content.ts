import { type JSXElement } from "solid-js";
import type { PaperContent } from "../../declarations/backend/backend.did";
import { encodeRawContent } from "./encode";

export function getContent(content: PaperContent): JSXElement {
  const { content_format, content_source } = content;
  if ("Raw" in content_source && content_source.Raw !== undefined) {
    return encodeRawContent(content_format, content_source.Raw);
  }
  return undefined;
}
