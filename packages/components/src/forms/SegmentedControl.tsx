import { ToggleGroup } from "radix-ui";
import { BODY_TEXT, FIELD_SURFACE, FOCUS_RING } from "./styles";

export type SegmentOption = { value: string; label: string };

type SegmentedControlProps = {
  label: string;
  options: SegmentOption[];
  value: string;
  onValueChange: (value: string) => void;
  locked?: boolean;
};

export function SegmentedControl({ label, options, value, onValueChange, locked = false }: SegmentedControlProps) {
  return (
    <ToggleGroup.Root
      type="single"
      aria-label={label}
      value={value}
      disabled={locked}
      onValueChange={(next) => {
        if (next) onValueChange(next);
      }}
      className={`${FIELD_SURFACE} inline-flex gap-[var(--space-xs)] p-[var(--space-xs)]`}
    >
      {options.map((option) => (
        <ToggleGroup.Item
          key={option.value}
          value={option.value}
          className={`${BODY_TEXT} ${FOCUS_RING} cursor-pointer rounded-[var(--rounded-sm)] border-2 border-transparent px-[var(--space-md)] py-[var(--space-xs)] data-[state=on]:border-[color:var(--color-accent)] data-[state=on]:bg-[var(--color-accent-soft)] data-[state=on]:text-[color:var(--color-accent)] disabled:cursor-not-allowed disabled:text-[color:var(--color-text-3)]`}
        >
          {option.label}
        </ToggleGroup.Item>
      ))}
    </ToggleGroup.Root>
  );
}
