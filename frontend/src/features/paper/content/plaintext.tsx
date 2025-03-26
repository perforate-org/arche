import { type JSXElement } from "solid-js";

interface PlainTextProps {
  text: string;
}

export function PlainText({ text }: PlainTextProps): JSXElement {
  const paragraphs = text.split("\n\n").filter((p) => p.trim() !== "");

  return (
    <div class="flex flex-col gap-4">
      {paragraphs.map((paragraph, _) => (
        <p class="text-base leading-relaxed">{paragraph}</p>
      ))}
    </div>
  );
}
