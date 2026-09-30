import type { Meta, StoryObj } from "@storybook/react-vite";
import { expect, fn } from "storybook/test";
import { Toggle } from "./Toggle";
import { expectKeyboardFocusRing } from "./expectKeyboardFocusRing";

const meta = {
  title: "Controls/Toggle",
  component: Toggle,
  args: { label: "Never send to AI", onChange: fn() },
  play: async ({ canvas }) => {
    await expectKeyboardFocusRing(canvas.getByRole("switch"));
  },
} satisfies Meta<typeof Toggle>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Off: Story = {
  args: { checked: false },
  play: async ({ canvas, args, userEvent }) => {
    const toggle = canvas.getByRole("switch");
    await expect(toggle).toHaveAttribute("aria-checked", "false");
    await expectKeyboardFocusRing(toggle);
    await userEvent.keyboard(" ");
    await expect(args.onChange).toHaveBeenCalledWith(true);
  },
};

export const On: Story = {
  args: { checked: true },
  play: async ({ canvas, args, userEvent }) => {
    const toggle = canvas.getByRole("switch");
    await expect(toggle).toHaveAttribute("aria-checked", "true");
    await expectKeyboardFocusRing(toggle);
    await userEvent.keyboard("{Enter}");
    await expect(args.onChange).toHaveBeenCalledWith(false);
  },
};

export const Locked: Story = {
  args: { checked: false, disabled: true },
  play: async ({ canvas, args, userEvent }) => {
    const toggle = canvas.getByRole("switch");
    await expect(toggle).toBeDisabled();
    await userEvent.click(toggle);
    await expect(args.onChange).not.toHaveBeenCalled();
  },
};
