import type { LucideIcon } from "lucide-react";
import { SelectionBar } from "./SelectionBar";

const ICON_SIZE = 16;

export type NavItemProps = {
  icon: LucideIcon;
  label: string;
  count?: number;
  selected?: boolean;
  tone?: "default" | "accent";
};

export function NavItem({ icon: Icon, label, count, selected = false, tone = "default" }: NavItemProps) {
  const isAccent = selected || tone === "accent";
  const background = selected ? "bg-(--color-accent-soft)" : "bg-transparent";
  const textColour = isAccent ? "text-(--color-accent)" : "text-(--color-text)";
  const iconColour = isAccent ? "text-(--color-accent)" : "text-(--color-text-2)";

  return (
    <button
      type="button"
      aria-current={selected ? "true" : undefined}
      className={`relative flex w-full items-center gap-(--space-md) rounded-(--rounded-md) p-(--space-sm) text-left outline-none focus-visible:ring-(length:--space-2xs) focus-visible:ring-(--color-border-focus) ${background}`}
    >
      {selected && <SelectionBar />}
      <span className={`flex shrink-0 ${iconColour}`}>
        <Icon size={ICON_SIZE} aria-hidden="true" />
      </span>
      <span className={`flex-1 truncate text-(length:--text-body-size) leading-(--text-body-line-height) font-medium ${textColour}`}>
        {label}
      </span>
      {count !== undefined && (
        <span className="shrink-0 text-(length:--text-small-size) leading-(--text-small-line-height) text-(--color-text-3)">
          {count}
        </span>
      )}
    </button>
  );
}
