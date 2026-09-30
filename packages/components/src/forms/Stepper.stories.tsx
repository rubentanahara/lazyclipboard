import type { Meta, StoryObj } from "@storybook/react-vite";
import { expect, fn, userEvent } from "storybook/test";
import { Stepper } from "./Stepper";
import { withStoryFrame } from "./storyFrame";

const meta = {
  title: "Forms/Stepper",
  component: Stepper,
  decorators: [withStoryFrame],
  args: {
    label: "History size",
    value: 5,
    min: 1,
    max: 10,
    onChange: fn(),
  },
} satisfies Meta<typeof Stepper>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  play: async ({ canvas, args }) => {
    await userEvent.click(canvas.getByRole("button", { name: "Increase History size" }));
    await expect(args.onChange).toHaveBeenCalledWith(6);
    await userEvent.click(canvas.getByRole("button", { name: "Decrease History size" }));
    await expect(args.onChange).toHaveBeenCalledWith(4);
  },
};

export const AtMinimum: Story = {
  args: { value: 1 },
  play: async ({ canvas }) => {
    await expect(canvas.getByRole("button", { name: "Decrease History size" })).toBeDisabled();
  },
};

export const AtMaximum: Story = {
  args: { value: 10 },
  play: async ({ canvas }) => {
    await expect(canvas.getByRole("button", { name: "Increase History size" })).toBeDisabled();
  },
};

export const Locked: Story = {
  args: { locked: true },
  play: async ({ canvas }) => {
    await expect(canvas.getByRole("button", { name: "Increase History size" })).toBeDisabled();
    await expect(canvas.getByRole("button", { name: "Decrease History size" })).toBeDisabled();
  },
};
