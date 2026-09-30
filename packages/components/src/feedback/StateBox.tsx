import { CircleAlert, Inbox, KeyRound, type LucideIcon } from "lucide-react";
import type { ReactNode } from "react";
import { TEXT_BODY, TEXT_TITLE } from "./type";

type StateBoxVariant = "error" | "empty" | "no-key";

const ICON_SIZE_LARGE = 32;

const ICONS: Record<StateBoxVariant, LucideIcon> = {
  error: CircleAlert,
  empty: Inbox,
  "no-key": KeyRound,
};

const ICON_COLOURS: Record<StateBoxVariant, string> = {
  error: "text-(--color-danger)",
  empty: "text-(--color-text-3)",
  "no-key": "text-(--color-text-3)",
};

interface StateBoxProps {
  variant: StateBoxVariant;
  title: string;
  message: string;
  primaryAction?: ReactNode;
  secondaryAction?: ReactNode;
}

export function StateBox({ variant, title, message, primaryAction, secondaryAction }: StateBoxProps) {
  const Icon = ICONS[variant];
  return (
    <div
      role={variant === "error" ? "alert" : "status"}
      className="flex flex-col items-center gap-(--space-md) p-(--space-xl) text-center"
    >
      <Icon aria-hidden size={ICON_SIZE_LARGE} className={ICON_COLOURS[variant]} />
      <div className="flex flex-col gap-(--space-xs)">
        <p className={`${TEXT_TITLE} text-(--color-text)`}>{title}</p>
        <p className={`${TEXT_BODY} text-(--color-text-2)`}>{message}</p>
      </div>
      {(primaryAction || secondaryAction) && (
        <div className="flex gap-(--space-sm)">
          {primaryAction}
          {secondaryAction}
        </div>
      )}
    </div>
  );
}
