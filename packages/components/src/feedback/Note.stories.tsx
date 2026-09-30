import type { Meta, StoryObj } from "@storybook/react-vite";
import { ImageOff } from "lucide-react";
import { expect, fn, userEvent, within } from "storybook/test";
import { Note } from "./Note";

const meta = {
  title: "Feedback/Note",
  component: Note,
  args: { message: "Items are stored on this device only." },
} satisfies Meta<typeof Note>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const WithAction: Story = {
  args: { icon: ImageOff, message: "1 image skipped.", actionLabel: "Paste as file", onAction: fn() },
  play: async ({ canvasElement, args }) => {
    await userEvent.click(within(canvasElement).getByRole("button", { name: "Paste as file" }));
    await expect(args.onAction).toHaveBeenCalledOnce();
  },
};
