import type { Meta, StoryObj } from "@storybook/react-vite";
import { expect, fn, userEvent } from "storybook/test";
import { SegmentedControl } from "./SegmentedControl";
import { withStoryFrame } from "./storyFrame";

const meta = {
  title: "Forms/Segmented Control",
  component: SegmentedControl,
  decorators: [withStoryFrame],
  args: {
    label: "Theme",
    value: "system",
    options: [
      { value: "light", label: "Light" },
      { value: "dark", label: "Dark" },
      { value: "system", label: "System" },
    ],
    onValueChange: fn(),
  },
} satisfies Meta<typeof SegmentedControl>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  play: async ({ canvas, args }) => {
    await userEvent.click(canvas.getByRole("radio", { name: "Dark" }));
    await expect(args.onValueChange).toHaveBeenCalledWith("dark");
  },
};

export const SelectedStaysSelected: Story = {
  play: async ({ canvas, args }) => {
    await userEvent.click(canvas.getByRole("radio", { name: "System" }));
    await expect(args.onValueChange).not.toHaveBeenCalled();
    await expect(canvas.getByRole("radio", { name: "System" })).toBeChecked();
  },
};

export const Locked: Story = {
  args: { locked: true },
  play: async ({ canvas }) => {
    await expect(canvas.getByRole("radio", { name: "Dark" })).toBeDisabled();
  },
};
