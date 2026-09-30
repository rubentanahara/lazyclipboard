import type { ReactNode } from "react";
import { TEXT_SMALL } from "./type";

interface FooterHintProps {
  shortcut: ReactNode;
  label: string;
}

export function FooterHint({ shortcut, label }: FooterHintProps) {
  return (
    <span className="flex items-center gap-(--space-sm)">
      {shortcut}
      <span className={`${TEXT_SMALL} text-(--color-text-3)`}>{label}</span>
    </span>
  );
}
