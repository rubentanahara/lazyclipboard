import { Folder } from "lucide-react";

const ICON_SIZE = 12;

export type GroupTagProps = {
  name: string;
};

export function GroupTag({ name }: GroupTagProps) {
  return (
    <span className="inline-flex shrink-0 items-center gap-(--space-xs) rounded-(--rounded-pill) bg-(--color-accent-soft) px-(--space-sm) text-(length:--text-caption-size) leading-(--text-caption-line-height) text-(--color-accent)">
      <Folder size={ICON_SIZE} aria-hidden="true" />
      {name}
    </span>
  );
}
