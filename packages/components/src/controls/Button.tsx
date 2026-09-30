import type { ButtonHTMLAttributes } from "react";
import { LoaderCircle, Trash2 } from "lucide-react";
import { focusRing } from "./focusRing";

type Variant = "primary" | "secondary" | "danger";
type Size = "default" | "small";

type ButtonProps = Omit<ButtonHTMLAttributes<HTMLButtonElement>, "children"> & {
  children: string;
  variant?: Variant;
  size?: Size;
  loading?: boolean;
};

const BASE =
  "inline-flex items-center justify-center gap-(--space-sm) rounded-(--rounded-md) font-(--text-ui-weight) leading-(--text-ui-line-height) transition-colors duration-(--motion-duration-fast) ease-(--motion-easing-standard) disabled:cursor-not-allowed disabled:bg-(--color-surface-alt) disabled:text-(color:--color-text-3) aria-busy:cursor-progress";

const VARIANT_CLASSES: Record<Variant, string> = {
  primary:
    "bg-(--color-accent) text-(color:--color-text-on-accent) enabled:hover:bg-(--color-accent-hover) enabled:active:bg-(--color-accent-active)",
  secondary:
    "border border-(--color-border) bg-(--color-surface) text-(color:--color-text) enabled:hover:bg-(--color-surface-alt)",
  danger:
    "border border-(--color-border) bg-(--color-surface) text-(color:--color-danger) enabled:hover:bg-(--color-surface-alt)",
};

const SIZE_CLASSES: Record<Size, string> = {
  default:
    "px-(--space-lg) py-(--space-sm) text-(length:--text-ui-size)",
  small:
    "px-(--space-md) py-(--space-xs) text-(length:--text-small-size)",
};

const ICON_SIZE: Record<Size, number> = { default: 16, small: 14 };

export const Button = ({
  variant = "primary",
  size = "default",
  loading = false,
  disabled,
  className = "",
  children,
  ...rest
}: ButtonProps) => (
  <button
    type="button"
    {...rest}
    disabled={disabled}
    aria-busy={loading || undefined}
    onClick={loading ? undefined : rest.onClick}
    className={`${BASE} ${VARIANT_CLASSES[variant]} ${SIZE_CLASSES[size]} ${focusRing} ${className}`}
  >
    {loading && (
      <LoaderCircle
        aria-hidden
        size={ICON_SIZE[size]}
        className="animate-spin motion-reduce:animate-none"
      />
    )}
    {variant === "danger" && !loading && (
      <Trash2 aria-hidden size={ICON_SIZE[size]} />
    )}
    {children}
  </button>
);
