import { focusRing } from "./focusRing";

type ToggleProps = {
  checked: boolean;
  onChange: (checked: boolean) => void;
  label: string;
  disabled?: boolean;
};

const TRACK =
  "relative inline-flex h-(--space-xl) w-[calc(var(--space-2xl)+var(--space-sm))] shrink-0 items-center rounded-(--rounded-pill) p-(--space-xs) transition-colors duration-(--motion-duration-fast) ease-(--motion-easing-standard) disabled:cursor-not-allowed disabled:opacity-60";

const KNOB =
  "block size-(--space-lg) rounded-(--rounded-pill) bg-(--color-surface-raised) transition-transform duration-(--motion-duration-fast) ease-(--motion-easing-standard)";

export const Toggle = ({ checked, onChange, label, disabled }: ToggleProps) => (
  <button
    type="button"
    role="switch"
    aria-checked={checked}
    aria-label={label}
    disabled={disabled}
    onClick={() => onChange(!checked)}
    className={`${TRACK} ${checked ? "bg-(--color-accent)" : "bg-(--color-text-3)"} ${focusRing}`}
  >
    <span className={`${KNOB} ${checked ? "translate-x-(--space-lg)" : ""}`} />
  </button>
);
