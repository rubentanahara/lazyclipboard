export function SelectionBar() {
  return (
    <span
      data-part="bar"
      aria-hidden="true"
      className="absolute inset-y-0 left-0 w-(--space-2xs) rounded-(--rounded-md) bg-(--color-accent)"
    />
  );
}
