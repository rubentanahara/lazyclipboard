export const BODY_TEXT =
  "font-[family-name:var(--text-body-family)] text-[length:var(--text-body-size)] font-[number:var(--text-body-weight)] leading-[var(--text-body-line-height)] text-[color:var(--color-text)]";

export const SMALL_TEXT =
  "font-[family-name:var(--text-small-family)] text-[length:var(--text-small-size)] font-[number:var(--text-small-weight)] leading-[var(--text-small-line-height)] text-[color:var(--color-text-2)]";

export const FOCUS_RING =
  "focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[color:var(--color-border-focus)]";

export const FIELD_SURFACE =
  "bg-[var(--color-surface-alt)] border border-[color:var(--color-border)] rounded-[var(--rounded-md)]";

export const FIELD_PADDING = "px-[var(--space-md)] py-[var(--space-sm)]";

export const SECONDARY_BUTTON = `${BODY_TEXT} ${FIELD_SURFACE} ${FIELD_PADDING} ${FOCUS_RING} cursor-pointer hover:bg-[var(--color-surface-raised)] disabled:cursor-not-allowed disabled:text-[color:var(--color-text-3)]`;

export const KEY_CAP =
  "inline-flex items-center rounded-[var(--rounded-sm)] bg-[var(--color-surface-alt)] px-[var(--space-sm)] py-[var(--space-2xs)] font-[family-name:var(--text-caption-family)] text-[length:var(--text-caption-size)] font-[number:var(--text-body-weight)] leading-[var(--text-caption-line-height)] text-[color:var(--color-text)]";

export const ICON_SIZE = 16;

export const ERROR_TEXT = "text-[color:var(--color-danger)]";
