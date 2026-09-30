import { ChevronLeft, type LucideIcon } from "lucide-react";
import type { ReactNode } from "react";
import { TEXT_TITLE } from "./type";

const ICON_SIZE = 18;
const BACK_LABEL = "Back";

const HEADER = "flex items-center gap-(--space-md) border-b border-(--color-border) bg-(--color-surface) px-(--space-xl) py-(--space-md)";

interface PanelHeaderProps {
  icon: LucideIcon;
  title: string;
}

export function PanelHeader({ icon: Icon, title }: PanelHeaderProps) {
  return (
    <header className={HEADER}>
      <Icon aria-hidden size={ICON_SIZE} className="text-(--color-text-2)" />
      <h2 className={`${TEXT_TITLE} text-(--color-text)`}>{title}</h2>
    </header>
  );
}

interface PanelHeaderDetailProps {
  title: string;
  onBack: () => void;
  count: ReactNode;
}

export function PanelHeaderDetail({ title, onBack, count }: PanelHeaderDetailProps) {
  return (
    <header className={HEADER}>
      <button
        type="button"
        aria-label={BACK_LABEL}
        onClick={onBack}
        className="rounded-(--rounded-sm) text-(--color-text-2) focus-visible:outline-2 focus-visible:outline-(--color-border-focus)"
      >
        <ChevronLeft aria-hidden size={ICON_SIZE} />
      </button>
      <h2 className={`${TEXT_TITLE} flex-1 text-(--color-text)`}>{title}</h2>
      {count}
    </header>
  );
}
