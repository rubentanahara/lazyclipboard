import { TypeIcon, type ContentKind } from "./TypeIcon";

const KIND_LABEL: Record<ContentKind, string> = {
  text: "Text",
  link: "Link",
  file: "File",
};

const ICON_SIZE = 14;

export type CapturedStripProps = {
  kind: ContentKind;
  text: string;
  source: string;
};

export function CapturedStrip({ kind, text, source }: CapturedStripProps) {
  return (
    <section
      aria-label="Captured"
      className="flex flex-col gap-(--space-sm) bg-(--color-surface-alt) px-(--space-lg) py-(--space-md)"
    >
      <span className="text-(length:--text-overline-size) leading-(--text-overline-line-height) font-semibold tracking-wider text-(--color-text-3) uppercase">
        Captured
      </span>
      <p className="m-0 line-clamp-2 text-(length:--text-body-size) leading-(--text-body-line-height) font-medium text-(--color-text)">
        {text}
      </p>
      <span className="flex items-center gap-(--space-sm) text-(length:--text-small-size) leading-(--text-small-line-height) text-(--color-text-3)">
        <TypeIcon kind={kind} size={ICON_SIZE} />
        {KIND_LABEL[kind]} · {source}
      </span>
    </section>
  );
}
