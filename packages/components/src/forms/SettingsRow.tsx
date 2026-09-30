import { useId, type ReactNode } from "react";
import { BODY_TEXT, SMALL_TEXT } from "./styles";

type SettingsRowProps = {
  label: string;
  children: ReactNode;
  description?: string;
};

export function SettingsRow({ label, children, description }: SettingsRowProps) {
  const labelId = useId();
  return (
    <div role="group" aria-labelledby={labelId} className="flex items-center justify-between gap-[var(--space-lg)] py-[var(--space-md)]">
      <div className="flex flex-col gap-[var(--space-xs)]">
        <span id={labelId} className={BODY_TEXT}>
          {label}
        </span>
        {description && <span className={SMALL_TEXT}>{description}</span>}
      </div>
      {children}
    </div>
  );
}
