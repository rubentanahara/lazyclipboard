import type { ReactNode } from "react";

type KeyCapProps = { children: ReactNode };

export const KeyCap = ({ children }: KeyCapProps) => (
  <kbd className="inline-flex min-w-(--space-xl) items-center justify-center rounded-(--rounded-sm) bg-(--color-surface-alt) px-(--space-xs) py-(--space-2xs) font-(family-name:--text-caption-family) text-(length:--text-caption-size) font-(--text-ui-weight) leading-(--text-caption-line-height) text-(color:--color-text-2)">
    {children}
  </kbd>
);
