import type { Meta, StoryObj } from "@storybook/react-vite";
import { expect, fn } from "storybook/test";
import { Button } from "./Button";
import { expectKeyboardFocusRing } from "./expectKeyboardFocusRing";

const meta = {
  title: "Controls/Button",
  component: Button,
  args: { children: "Paste all", onClick: fn() },
  play: async ({ canvas }) => {
    await expectKeyboardFocusRing(canvas.getByRole("button"));
  },
} satisfies Meta<typeof Button>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Primary: Story = { args: { variant: "primary" } };

export const Secondary: Story = { args: { variant: "secondary" } };

export const Danger: Story = {
  args: { variant: "danger", children: "Delete" },
  play: async ({ canvas }) => {
    const button = canvas.getByRole("button", { name: "Delete" });
    await expect(button.querySelector("svg")).not.toBeNull();
    await expectKeyboardFocusRing(button);
  },
};

export const Small: Story = {
  args: { variant: "secondary", size: "small", children: "Rename" },
};

export const SmallDanger: Story = {
  args: { variant: "danger", size: "small", children: "Delete…" },
};

export const Loading: Story = {
  args: { variant: "primary", loading: true, children: "Pasting" },
  play: async ({ canvas, args, userEvent }) => {
    const button = canvas.getByRole("button", { name: "Pasting" });
    await expect(button).toHaveAttribute("aria-busy", "true");
    await expectKeyboardFocusRing(button);
    await userEvent.click(button);
    await expect(args.onClick).not.toHaveBeenCalled();
  },
};

export const Locked: Story = {
  args: { variant: "primary", disabled: true },
  play: async ({ canvas, args, userEvent }) => {
    const button = canvas.getByRole("button");
    await expect(button).toBeDisabled();
    await userEvent.click(button);
    await expect(args.onClick).not.toHaveBeenCalled();
  },
};
