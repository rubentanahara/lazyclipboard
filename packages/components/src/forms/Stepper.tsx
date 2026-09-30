import { Minus, Plus } from "lucide-react";
import { BODY_TEXT, FIELD_SURFACE, FOCUS_RING, ICON_SIZE } from "./styles";

type StepperProps = {
  label: string;
  value: number;
  min: number;
  max: number;
  onChange: (value: number) => void;
  locked?: boolean;
};

const STEP_BUTTON = `${FOCUS_RING} inline-flex cursor-pointer items-center justify-center p-[var(--space-sm)] text-[color:var(--color-text)] hover:bg-[var(--color-surface-raised)] disabled:cursor-not-allowed disabled:text-[color:var(--color-text-3)]`;

export function Stepper({ label, value, min, max, onChange, locked = false }: StepperProps) {
  return (
    <div role="group" aria-label={label} className={`${FIELD_SURFACE} inline-flex items-center overflow-hidden`}>
      <button
        type="button"
        aria-label={`Decrease ${label}`}
        className={STEP_BUTTON}
        disabled={locked || value <= min}
        onClick={() => onChange(value - 1)}
      >
        <Minus size={ICON_SIZE} aria-hidden />
      </button>
      <output className={`${BODY_TEXT} min-w-10 text-center`}>{value}</output>
      <button
        type="button"
        aria-label={`Increase ${label}`}
        className={STEP_BUTTON}
        disabled={locked || value >= max}
        onClick={() => onChange(value + 1)}
      >
        <Plus size={ICON_SIZE} aria-hidden />
      </button>
    </div>
  );
}
