import { Check, ChevronDown, Lock } from "lucide-react";
import { Select as RadixSelect } from "radix-ui";
import { BODY_TEXT, FIELD_PADDING, FIELD_SURFACE, FOCUS_RING, ICON_SIZE } from "./styles";

export type SelectOption = { value: string; label: string };

type SelectProps = {
  label: string;
  options: SelectOption[];
  onValueChange: (value: string) => void;
  value?: string;
  placeholder?: string;
  locked?: boolean;
};

const EMPTY_OPTIONS_LABEL = "No options";

export function Select({ label, options, onValueChange, value, placeholder, locked = false }: SelectProps) {
  const isEmpty = options.length === 0;
  return (
    <RadixSelect.Root value={value} onValueChange={onValueChange} disabled={locked || isEmpty}>
      <RadixSelect.Trigger
        aria-label={label}
        className={`${BODY_TEXT} ${FIELD_SURFACE} ${FIELD_PADDING} ${FOCUS_RING} inline-flex min-w-40 cursor-pointer items-center justify-between gap-[var(--space-sm)] disabled:cursor-not-allowed disabled:text-[color:var(--color-text-3)]`}
      >
        {isEmpty ? (
          <span>{EMPTY_OPTIONS_LABEL}</span>
        ) : (
          <RadixSelect.Value placeholder={placeholder} />
        )}
        <RadixSelect.Icon>
          {locked ? <Lock size={ICON_SIZE} aria-hidden /> : <ChevronDown size={ICON_SIZE} aria-hidden />}
        </RadixSelect.Icon>
      </RadixSelect.Trigger>
      <RadixSelect.Portal>
        <RadixSelect.Content
          position="popper"
          sideOffset={4}
          className={`${BODY_TEXT} ${FIELD_SURFACE} z-50 min-w-[var(--radix-select-trigger-width)] bg-[var(--color-surface-raised)] p-[var(--space-xs)] shadow-[0_var(--elevation-menu-y)_var(--elevation-menu-blur)_var(--color-shadow-color)]`}
        >
          <RadixSelect.Viewport>
            {options.map((option) => (
              <RadixSelect.Item
                key={option.value}
                value={option.value}
                className="flex cursor-pointer items-center justify-between gap-[var(--space-sm)] rounded-[var(--rounded-sm)] px-[var(--space-sm)] py-[var(--space-xs)] outline-none data-[highlighted]:bg-[var(--color-accent-soft)] data-[state=checked]:text-[color:var(--color-accent)]"
              >
                <RadixSelect.ItemText>{option.label}</RadixSelect.ItemText>
                <RadixSelect.ItemIndicator>
                  <Check size={ICON_SIZE} aria-hidden />
                </RadixSelect.ItemIndicator>
              </RadixSelect.Item>
            ))}
          </RadixSelect.Viewport>
        </RadixSelect.Content>
      </RadixSelect.Portal>
    </RadixSelect.Root>
  );
}
