import { Toast as ToastPrimitive } from "radix-ui";
import { useEffect, useState } from "react";
import { TEXT_BODY, TEXT_SMALL } from "./type";

const DEFAULT_DURATION_MS = 6000;
const PAUSED_DETAIL = "Paused";

interface ToastProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  message: string;
  actionLabel?: string;
  onAction?: () => void;
  detail?: string;
  duration?: number;
}

export function Toast({ open, onOpenChange, message, actionLabel, onAction, detail, duration = DEFAULT_DURATION_MS }: ToastProps) {
  const [paused, setPaused] = useState(false);
  useEffect(() => {
    if (!open) setPaused(false);
  }, [open]);

  const shownDetail = paused ? PAUSED_DETAIL : detail;

  return (
    <ToastPrimitive.Provider duration={duration}>
      <ToastPrimitive.Root
        open={open}
        onOpenChange={onOpenChange}
        onPause={() => setPaused(true)}
        onResume={() => setPaused(false)}
        className="flex items-center gap-(--space-md) rounded-(--rounded-md) bg-(--color-surface-inverse) px-(--space-lg) py-(--space-md) text-(--color-text-on-inverse)"
      >
        <ToastPrimitive.Description className={`${TEXT_BODY} flex-1`}>{message}</ToastPrimitive.Description>
        {actionLabel && (
          <ToastPrimitive.Action altText={actionLabel} asChild>
            <button
              type="button"
              onClick={onAction}
              className={`${TEXT_BODY} shrink-0 rounded-(--rounded-sm) underline focus-visible:outline-2 focus-visible:outline-(--color-border-focus)`}
            >
              {actionLabel}
            </button>
          </ToastPrimitive.Action>
        )}
        {shownDetail && <span className={`${TEXT_SMALL} shrink-0`}>{shownDetail}</span>}
      </ToastPrimitive.Root>
      <ToastPrimitive.Viewport className="absolute bottom-(--space-lg) left-1/2 w-85 -translate-x-1/2" />
    </ToastPrimitive.Provider>
  );
}
