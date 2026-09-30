import type { ReactNode } from "react";

type ChipProps = { children: ReactNode };

export const Chip = ({ children }: ChipProps) => (
  <span className="inline-flex items-center rounded-(--rounded-pill) bg-(--color-surface-alt) px-(--space-sm) py-(--space-2xs) font-(family-name:--text-caption-family) text-(length:--text-caption-size) font-(--text-ui-weight) leading-(--text-caption-line-height) text-(color:--color-text-3)">
    {children}
  </span>
);
