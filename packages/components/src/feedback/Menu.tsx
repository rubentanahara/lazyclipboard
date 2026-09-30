import { Trash, type LucideIcon } from "lucide-react";
import { DropdownMenu } from "radix-ui";
import type { ReactNode } from "react";
import { TEXT_BODY } from "./type";

const ICON_SIZE = 16;

interface MenuProps {
  trigger: ReactNode;
  children: ReactNode;
  open?: boolean;
  onOpenChange?: (open: boolean) => void;
}

export function Menu({ trigger, children, open, onOpenChange }: MenuProps) {
  return (
    <DropdownMenu.Root modal={false} open={open} onOpenChange={onOpenChange}>
      <DropdownMenu.Trigger asChild>{trigger}</DropdownMenu.Trigger>
      <DropdownMenu.Portal>
        <DropdownMenu.Content
          align="start"
          className="flex min-w-48 flex-col rounded-(--rounded-md) border border-(--color-border) bg-(--color-surface-raised) p-(--space-xs) shadow-[0_var(--elevation-menu-y)_var(--elevation-menu-blur)_var(--color-shadow-color)]"
        >
          {children}
        </DropdownMenu.Content>
      </DropdownMenu.Portal>
    </DropdownMenu.Root>
  );
}

type MenuItemProps =
  | { variant?: "default"; icon: LucideIcon; label: string; onSelect: () => void }
  | { variant: "danger"; label: string; onSelect: () => void };

const ITEM_BASE = `${TEXT_BODY} flex cursor-default items-center gap-(--space-md) rounded-(--rounded-sm) px-(--space-md) py-(--space-sm) outline-none data-[highlighted]:bg-(--color-accent-soft) data-[highlighted]:shadow-[inset_2px_0_0_var(--color-accent)]`;

export function MenuItem(props: MenuItemProps) {
  const danger = props.variant === "danger";
  const Icon = props.variant === "danger" ? Trash : props.icon;
  return (
    <DropdownMenu.Item
      onSelect={props.onSelect}
      className={`${ITEM_BASE} ${danger ? "text-(--color-danger)" : "text-(--color-text)"}`}
    >
      <Icon aria-hidden size={ICON_SIZE} />
      {props.label}
    </DropdownMenu.Item>
  );
}
