import { GroupTag } from "./GroupTag";
import { SelectionBar } from "./SelectionBar";
import { TypeIcon, type ContentKind } from "./TypeIcon";

export type SearchResultRowProps = {
  kind: ContentKind;
  title: string;
  match: string;
  meta: string;
  group: string;
  selected?: boolean;
};

function splitAtMatch(title: string, match: string) {
  const start = title.toLowerCase().indexOf(match.toLowerCase());
  if (match === "" || start === -1) return { before: title, found: "", after: "" };
  const end = start + match.length;
  return { before: title.slice(0, start), found: title.slice(start, end), after: title.slice(end) };
}

export function SearchResultRow({ kind, title, match, meta, group, selected = false }: SearchResultRowProps) {
  const { before, found, after } = splitAtMatch(title, match);
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
      <span className={`flex shrink-0 ${iconColour}`}>
        <TypeIcon kind={kind} />
      </span>
      <span className="flex min-w-0 flex-1 flex-col gap-(--space-xs)">
        <span className="truncate text-(length:--text-body-size) leading-(--text-body-line-height) font-medium text-(--color-text)">
          {before}
          {found && (
            <mark className="rounded-(--rounded-sm) bg-(--color-selection) text-(--color-text) underline">{found}</mark>
          )}
          {after}
        </span>
        <span className="text-(length:--text-small-size) leading-(--text-small-line-height) text-(--color-text-3)">
          {meta}
        </span>
      </span>
      <GroupTag name={group} />
    </div>
  );
}
