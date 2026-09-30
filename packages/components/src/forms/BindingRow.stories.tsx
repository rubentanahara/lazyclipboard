import type { Meta, StoryObj } from "@storybook/react-vite";
import { expect } from "storybook/test";
import { BindingRow } from "./BindingRow";
import { withStoryFrame } from "./storyFrame";

const meta = {
  title: "Forms/Binding Row",
  component: BindingRow,
  decorators: [withStoryFrame],
  args: { action: "Next item", vim: "Ctrl+J", always: "↓", vimEnabled: true },
} satisfies Meta<typeof BindingRow>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const VimOff: Story = {
  args: { vimEnabled: false },
  play: async ({ canvas }) => {
    await expect(canvas.getByText("Ctrl+J")).toHaveAccessibleDescription("Vim binding off");
  },
};
