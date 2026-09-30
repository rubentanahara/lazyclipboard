import { TriangleAlert } from "lucide-react";
import { useId, useState, type KeyboardEvent } from "react";
import { BODY_TEXT, FIELD_PADDING, FIELD_SURFACE, FOCUS_RING, ICON_SIZE, KEY_CAP, SMALL_TEXT } from "./styles";

type ShortcutFieldProps = {
  label: string;
  value: string;
  onChange: (chord: string) => void;
  conflict?: string;
  locked?: boolean;
};

const MODIFIER_KEYS = new Set(["Control", "Alt", "Shift", "Meta"]);
const UNSET_LABEL = "Not set";
const RECORDING_LABEL = "Press a shortcut";

const PHYSICAL_KEY_PATTERN = /^(?:Key|Digit)(.)$/;

function physicalKeyName(event: KeyboardEvent): string {
  const physical = PHYSICAL_KEY_PATTERN.exec(event.code);
  if (physical) return physical[1];
  return event.key.length === 1 ? event.key.toUpperCase() : event.key;
}

function hasModifier(event: KeyboardEvent): boolean {
  return event.ctrlKey || event.altKey || event.shiftKey || event.metaKey;
}

function formatChord(event: KeyboardEvent): string {
  const key = physicalKeyName(event);
  const isMac = navigator.userAgent.includes("Mac");
  if (isMac) {
    return `${event.ctrlKey ? "⌃" : ""}${event.altKey ? "⌥" : ""}${event.shiftKey ? "⇧" : ""}${event.metaKey ? "⌘" : ""}${key}`;
  }
  const modifiers = [
    event.ctrlKey && "Ctrl",
    event.altKey && "Alt",
    event.shiftKey && "Shift",
    event.metaKey && "Win",
  ].filter(Boolean);
  return [...modifiers, key].join("+");
}

export function ShortcutField({ label, value, onChange, conflict, locked = false }: ShortcutFieldProps) {
  const [recording, setRecording] = useState(false);
  const conflictId = useId();

  const handleKeyDown = (event: KeyboardEvent<HTMLButtonElement>) => {
    if (!recording) return;
    event.preventDefault();
    if (event.key === "Escape") {
      setRecording(false);
      return;
    }
    if (MODIFIER_KEYS.has(event.key) || !hasModifier(event)) return;
    onChange(formatChord(event));
    setRecording(false);
  };

  return (
    <div className="inline-flex flex-col gap-[var(--space-xs)]">
      <button
        type="button"
        aria-label={label}
        aria-describedby={conflict ? conflictId : undefined}
        disabled={locked}
        onClick={() => setRecording(true)}
        onBlur={() => setRecording(false)}
        onKeyDown={handleKeyDown}
        className={`${BODY_TEXT} ${FIELD_SURFACE} ${FIELD_PADDING} ${FOCUS_RING} inline-flex min-w-40 cursor-pointer items-center disabled:cursor-not-allowed disabled:text-[color:var(--color-text-3)]`}
      >
        {recording ? (
          <span>{RECORDING_LABEL}</span>
        ) : value === "" ? (
          <span>{UNSET_LABEL}</span>
        ) : (
          <kbd className={KEY_CAP}>{value}</kbd>
        )}
      </button>
      {conflict && (
        <p id={conflictId} role="alert" className={`${SMALL_TEXT} inline-flex items-center gap-[var(--space-xs)]`}>
          <TriangleAlert size={ICON_SIZE} aria-hidden className="text-[color:var(--color-warning)]" />
          {conflict}
        </p>
      )}
    </div>
  );
}
