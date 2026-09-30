import { SelectionBar } from "./SelectionBar";
import { TypeIcon, type ContentKind } from "./TypeIcon";

export type ItemKind = ContentKind | "image";

export type ItemRowProps = {
  kind: ItemKind;
  title: string;
  meta: string;
  selected?: boolean;
  formatted?: boolean;
  thumbnailSrc?: string;
};

export function ItemRow({ kind, title, meta, selected = false, formatted = false, thumbnailSrc }: ItemRowProps) {
  const background = selected ? "bg-(--color-accent-soft)" : "bg-(--color-surface)";
  const iconColour = selected ? "text-(--color-accent)" : "text-(--color-text-2)";

  return (
    <div
      role="option"
      aria-selected={selected}
      tabIndex={-1}
      className={`relative flex items-center gap-(--space-md) rounded-(--rounded-md) p-(--space-md) outline-none focus-visible:ring-(length:--space-2xs) focus-visible:ring-(--color-border-focus) ${background}`}
    >
      {selected && <SelectionBar />}
      {kind === "image" ? (
        <img src={thumbnailSrc} alt="" className="size-(--space-2xl) shrink-0 rounded-(--rounded-sm) object-cover" />
      ) : (
        <span className={`flex shrink-0 ${iconColour}`}>
          <TypeIcon kind={kind} />
        </span>
      )}
      <span className="line-clamp-2 min-w-0 flex-1 text-(length:--text-body-size) leading-(--text-body-line-height) font-medium text-(--color-text)">
        {title}
      </span>
      {formatted && (
        <span className="shrink-0 rounded-(--rounded-pill) bg-(--color-surface-alt) px-(--space-sm) text-(length:--text-caption-size) leading-(--text-caption-line-height) text-(--color-text-2)">
          Formatted
        </span>
      )}
      <span className="shrink-0 text-(length:--text-small-size) leading-(--text-small-line-height) text-(--color-text-3)">
        {meta}
      </span>
    </div>
  );
}
