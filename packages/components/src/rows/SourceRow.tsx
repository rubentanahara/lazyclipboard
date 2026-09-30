export type SourceFate = { kind: "point"; number: number } | { kind: "dropped" };

export type SourceRowProps = {
  position: number;
  text: string;
  fate: SourceFate;
};

function fateLabel(fate: SourceFate) {
  return fate.kind === "point" ? `Point ${fate.number}` : "Dropped";
}

function fateColours(fate: SourceFate) {
  return fate.kind === "point"
    ? "bg-(--color-accent-soft) text-(--color-accent)"
    : "bg-(--color-surface-alt) text-(--color-text-2)";
}

export function SourceRow({ position, text, fate }: SourceRowProps) {
  return (
    <li className="flex items-center gap-(--space-md) border-b border-(--color-border) px-(--space-md) py-(--space-sm)">
      <span className="w-(--space-md) shrink-0 text-(length:--text-small-size) leading-(--text-small-line-height) text-(--color-text-3)">
        {position}
      </span>
      <span className="min-w-0 flex-1 truncate text-(length:--text-body-size) leading-(--text-body-line-height) font-medium text-(--color-text)">
        {text}
      </span>
      <span
        className={`shrink-0 rounded-(--rounded-pill) px-(--space-sm) text-(length:--text-caption-size) leading-(--text-caption-line-height) ${fateColours(fate)}`}
      >
        {fateLabel(fate)}
      </span>
    </li>
  );
}
