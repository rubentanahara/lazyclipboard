import { TriangleAlert, Trash } from "lucide-react";
import { AlertDialog } from "radix-ui";
import { TEXT_BODY, TEXT_TITLE } from "./type";

const ICON_SIZE = 16;
const ICON_SIZE_LARGE = 24;
const CANCEL_LABEL = "Cancel";

const BUTTON_BASE = `${TEXT_BODY} flex items-center gap-(--space-sm) rounded-(--rounded-md) px-(--space-md) py-(--space-sm) focus-visible:outline-2 focus-visible:outline-(--color-border-focus)`;

interface ConfirmDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onConfirm: () => void;
  title: string;
  description: string;
  confirmLabel: string;
}

export function ConfirmDialog({ open, onOpenChange, onConfirm, title, description, confirmLabel }: ConfirmDialogProps) {
  return (
    <AlertDialog.Root open={open} onOpenChange={onOpenChange}>
      <AlertDialog.Portal>
        <AlertDialog.Overlay className="fixed inset-0 bg-(--color-overlay)" />
        <AlertDialog.Content className="fixed top-1/2 left-1/2 flex w-85 -translate-x-1/2 -translate-y-1/2 flex-col gap-(--space-lg) rounded-(--rounded-lg) bg-(--color-surface-raised) p-(--space-xl) shadow-[0_var(--elevation-panel-y)_var(--elevation-panel-blur)_var(--color-shadow-color)]">
          <TriangleAlert aria-hidden size={ICON_SIZE_LARGE} className="text-(--color-warning)" />
          <div className="flex flex-col gap-(--space-xs)">
            <AlertDialog.Title className={`${TEXT_TITLE} text-(--color-text)`}>{title}</AlertDialog.Title>
            <AlertDialog.Description className={`${TEXT_BODY} text-(--color-text-2)`}>{description}</AlertDialog.Description>
          </div>
          <div className="flex justify-end gap-(--space-sm)">
            <AlertDialog.Cancel className={`${BUTTON_BASE} bg-(--color-surface-alt) text-(--color-text)`}>{CANCEL_LABEL}</AlertDialog.Cancel>
            <AlertDialog.Action onClick={onConfirm} className={`${BUTTON_BASE} bg-(--color-danger) text-(--color-text-on-accent)`}>
              <Trash aria-hidden size={ICON_SIZE} />
              {confirmLabel}
            </AlertDialog.Action>
          </div>
        </AlertDialog.Content>
      </AlertDialog.Portal>
    </AlertDialog.Root>
  );
}
