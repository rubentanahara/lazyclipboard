import { expect, userEvent, waitFor } from "storybook/test";

const FOCUS_RING_WIDTH = "2px";

const resolveColorToken = (token: string) => {
  const probe = document.createElement("span");
  probe.style.color = `var(${token})`;
  document.body.append(probe);
  const color = getComputedStyle(probe).color;
  probe.remove();
  return color;
};

const describeRing = (control: HTMLElement) => {
  const { outlineStyle, outlineWidth, outlineColor } = getComputedStyle(control);
  return `${outlineStyle} ${outlineWidth} ${outlineColor}`;
};

export const expectKeyboardFocusRing = async (control: HTMLElement) => {
  await userEvent.tab();
  await expect(control).toHaveFocus();
  const expectedRing = `solid ${FOCUS_RING_WIDTH} ${resolveColorToken("--color-border-focus")}`;
  await waitFor(() => {
    if (describeRing(control) !== expectedRing) {
      throw new Error(`focus ring is "${describeRing(control)}", expected "${expectedRing}"`);
    }
  });
};
