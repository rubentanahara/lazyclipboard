import { useId } from "react";
import { BODY_TEXT, KEY_CAP } from "./styles";

type BindingRowProps = {
  action: string;
  vim: string;
  always: string;
  vimEnabled: boolean;
};

export function BindingRow({ action, vim, always, vimEnabled }: BindingRowProps) {
  const vimOffId = useId();
  return (
    <div className="flex items-center justify-between gap-[var(--space-lg)] py-[var(--space-sm)]">
      <span className={BODY_TEXT}>{action}</span>
      <span className="inline-flex items-center gap-[var(--space-sm)]">
        <kbd className={`${KEY_CAP} ${vimEnabled ? "" : "text-[color:var(--color-text-3)] line-through"}`} aria-describedby={vimEnabled ? undefined : vimOffId}>
          {vim}
        </kbd>
        {!vimEnabled && (
          <span id={vimOffId} className="sr-only">
            Vim binding off
          </span>
        )}
        <kbd className={KEY_CAP}>{always}</kbd>
      </span>
    </div>
  );
}
