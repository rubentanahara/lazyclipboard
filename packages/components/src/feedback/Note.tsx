import { Info, type LucideIcon } from "lucide-react";
import { TEXT_BODY } from "./type";

const ICON_SIZE = 16;

interface NoteProps {
  icon?: LucideIcon;
  message: string;
  actionLabel?: string;
  onAction?: () => void;
}

export function Note({ icon: Icon = Info, message, actionLabel, onAction }: NoteProps) {
  return (
    <div className="flex items-center gap-(--space-sm) bg-(--color-surface-alt) px-(--space-md) py-(--space-sm) rounded-(--rounded-md)">
      <Icon aria-hidden size={ICON_SIZE} className="shrink-0 text-(--color-text-2)" />
      <p className={`${TEXT_BODY} flex-1 truncate text-(--color-text-2)`}>{message}</p>
      {actionLabel && (
        <button
          type="button"
          onClick={onAction}
          className={`${TEXT_BODY} shrink-0 rounded-(--rounded-sm) text-(--color-accent) underline hover:text-(--color-accent-hover) active:text-(--color-accent-active) focus-visible:outline-2 focus-visible:outline-(--color-border-focus)`}
        >
          {actionLabel}
        </button>
      )}
    </div>
  );
}
