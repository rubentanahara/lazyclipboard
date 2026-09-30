import { CircleAlert, CircleCheck, Loader } from "lucide-react";
import { BODY_TEXT, ERROR_TEXT, ICON_SIZE, SECONDARY_BUTTON, SMALL_TEXT } from "./styles";

type TestStatusRowProps = { onTest: () => void } & (
  | { status: "idle" }
  | { status: "testing" }
  | { status: "failed"; message: string }
  | { status: "succeeded" }
  | { status: "blocked"; reason: string }
);

const IDLE_MESSAGE = "Not tested yet";
const TESTING_MESSAGE = "Testing…";
const SUCCEEDED_MESSAGE = "Connection works";

export function TestStatusRow(props: TestStatusRowProps) {
  const { status, onTest } = props;
  const buttonLabel = {
    idle: "Test connection",
    testing: TESTING_MESSAGE,
    failed: "Retry",
    succeeded: "Test connection",
    blocked: "Test connection",
  }[status];

  return (
    <div className="flex items-center justify-between gap-[var(--space-lg)] py-[var(--space-sm)]">
      <p role="status" className={`${BODY_TEXT} inline-flex items-center gap-[var(--space-sm)]`}>
        {status === "idle" && <span className={SMALL_TEXT}>{IDLE_MESSAGE}</span>}
        {status === "testing" && (
          <>
            <Loader size={ICON_SIZE} aria-hidden className="animate-spin motion-reduce:animate-none" />
            {TESTING_MESSAGE}
          </>
        )}
        {status === "failed" && (
          <span className={`${ERROR_TEXT} inline-flex items-center gap-[var(--space-sm)]`}>
            <CircleAlert size={ICON_SIZE} aria-hidden />
            {props.message}
          </span>
        )}
        {status === "succeeded" && (
          <span className="inline-flex items-center gap-[var(--space-sm)] text-[color:var(--color-success)]">
            <CircleCheck size={ICON_SIZE} aria-hidden />
            {SUCCEEDED_MESSAGE}
          </span>
        )}
        {status === "blocked" && <span className={SMALL_TEXT}>{props.reason}</span>}
      </p>
      <button
        type="button"
        className={SECONDARY_BUTTON}
        disabled={status === "testing" || status === "blocked"}
        onClick={onTest}
      >
        {buttonLabel}
      </button>
    </div>
  );
}
