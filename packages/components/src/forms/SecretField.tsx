import { CircleAlert } from "lucide-react";
import { useId, useState } from "react";
import { BODY_TEXT, ERROR_TEXT, FIELD_PADDING, FIELD_SURFACE, FOCUS_RING, ICON_SIZE, SECONDARY_BUTTON, SMALL_TEXT } from "./styles";

export const MASKED_PLACEHOLDER = "••••••••••••••••";

type SecretFieldProps = {
  label: string;
  hasSavedKey: boolean;
  onSave: (key: string) => void;
  saving?: boolean;
  error?: string;
  locked?: boolean;
};

export function SecretField({ label, hasSavedKey, onSave, saving = false, error, locked = false }: SecretFieldProps) {
  const [replacing, setReplacing] = useState(false);
  const [draft, setDraft] = useState("");
  const inputId = useId();
  const errorId = useId();
  const showsSavedKey = hasSavedKey && !replacing && !error;

  if (showsSavedKey) {
    return (
      <div className="inline-flex items-center gap-[var(--space-sm)]">
        <span className={`${BODY_TEXT} ${FIELD_SURFACE} ${FIELD_PADDING}`}>
          <span aria-hidden>{MASKED_PLACEHOLDER}</span>
          <span className="sr-only">{label} saved</span>
        </span>
        <button type="button" className={SECONDARY_BUTTON} disabled={locked} onClick={() => setReplacing(true)}>
          Replace key
        </button>
      </div>
    );
  }

  return (
    <div className="inline-flex flex-col gap-[var(--space-xs)]">
      <form
        className="inline-flex items-center gap-[var(--space-sm)]"
        onSubmit={(event) => {
          event.preventDefault();
          onSave(draft);
          setDraft("");
          setReplacing(false);
        }}
      >
        <label htmlFor={inputId} className="sr-only">
          {label}
        </label>
        <input
          id={inputId}
          type="password"
          autoComplete="off"
          spellCheck={false}
          placeholder="Paste your key"
          value={draft}
          disabled={saving || locked}
          aria-invalid={error ? true : undefined}
          aria-describedby={error ? errorId : undefined}
          onChange={(event) => setDraft(event.target.value)}
          className={`${BODY_TEXT} ${FIELD_SURFACE} ${FIELD_PADDING} ${FOCUS_RING} disabled:text-[color:var(--color-text-3)]`}
        />
        <button type="submit" className={SECONDARY_BUTTON} disabled={saving || locked || draft === ""}>
          {saving ? "Saving…" : "Save"}
        </button>
      </form>
      {error && (
        <p id={errorId} role="alert" className={`${SMALL_TEXT} ${ERROR_TEXT} inline-flex items-center gap-[var(--space-xs)]`}>
          <CircleAlert size={ICON_SIZE} aria-hidden />
          {error}
        </p>
      )}
    </div>
  );
}
