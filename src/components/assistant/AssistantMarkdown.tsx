import hljs from "highlight.js/lib/common";
import { Marked, type Tokens } from "marked";
import { useMemo } from "react";

function escapeHtml(text: string): string {
  return text
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}

const assistantMarkdown = new Marked({
  async: false,
  breaks: true,
  gfm: true,
  renderer: {
    code({ lang, text }: Tokens.Code) {
      const rawLanguage = lang ?? "";
      const normalizedLanguage = rawLanguage.match(/^[\w-]+/)?.[0] ?? "";
      const highlighted =
        normalizedLanguage && hljs.getLanguage(normalizedLanguage)
          ? hljs.highlight(text, {
              ignoreIllegals: true,
              language: normalizedLanguage,
            }).value
          : hljs.highlightAuto(text).value;
      const className = normalizedLanguage
        ? `hljs language-${normalizedLanguage}`
        : "hljs";
      return `<pre><code class="${className}">${highlighted}</code></pre>`;
    },
  },
});

interface AssistantMarkdownProps {
  content: string;
}

export function AssistantMarkdown({ content }: AssistantMarkdownProps) {
  const html = useMemo(
    () => assistantMarkdown.parse(escapeHtml(content)) as string,
    [content],
  );

  return (
    <div
      className="assistant-markdown markdown-body text-sm [&_a]:text-sky-300 [&_a:hover]:underline [&_blockquote]:border-l-[3px] [&_blockquote]:border-slate-600 [&_blockquote]:pl-3 [&_blockquote]:text-slate-400 [&_code]:rounded [&_code]:bg-slate-800 [&_code]:px-1.5 [&_code]:py-0.5 [&_code]:text-xs [&_code]:font-normal [&_h1]:mt-3 [&_h1]:mb-1 [&_h1]:text-base [&_h1]:font-semibold [&_h2]:mt-2 [&_h2]:mb-1 [&_h2]:text-sm [&_h2]:font-semibold [&_h3]:mt-2 [&_h3]:mb-1 [&_h3]:text-sm [&_h3]:font-medium [&_li]:ml-4 [&_li]:list-disc [&_ol]:ml-4 [&_ol]:list-decimal [&_p]:mb-2 [&_p]:last:mb-0 [&_pre]:my-2 [&_pre]:rounded-lg [&_pre]:bg-slate-900 [&_pre]:p-3 [&_pre]:text-xs [&_strong]:font-semibold [&_ul]:mb-2"
      dangerouslySetInnerHTML={{ __html: html }}
    />
  );
}
