import { File, Link, Type, type LucideIcon } from "lucide-react";

export type ContentKind = "text" | "link" | "file";

const ICON_BY_KIND: Record<ContentKind, LucideIcon> = {
  text: Type,
  link: Link,
  file: File,
};

const ICON_SIZE = 16;

export type TypeIconProps = {
  kind: ContentKind;
  size?: number;
};

export function TypeIcon({ kind, size = ICON_SIZE }: TypeIconProps) {
  const Icon = ICON_BY_KIND[kind];
  return <Icon size={size} aria-hidden="true" />;
}
