import type { Meta, StoryObj } from "@storybook/react-vite";
import { expect, fn, userEvent, within } from "storybook/test";
import { Select } from "./Select";
import { withStoryFrame } from "./storyFrame";

const meta = {
  title: "Forms/Select",
  component: Select,
  decorators: [withStoryFrame],
  args: {
    label: "Paste order",
    placeholder: "Choose an order",
    options: [
      { value: "newest", label: "Newest first" },
      { value: "oldest", label: "Oldest first" },
    ],
    onValueChange: fn(),
  },
} satisfies Meta<typeof Select>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  play: async ({ canvasElement, args }) => {
    const page = within(canvasElement.ownerDocument.body);
    await userEvent.click(page.getByRole("combobox", { name: "Paste order" }));
    await userEvent.click(await page.findByRole("option", { name: "Oldest first" }));
    await expect(args.onValueChange).toHaveBeenCalledWith("oldest");
  },
};

export const Selected: Story = {
  args: { value: "newest" },
  play: async ({ canvas }) => {
    await expect(canvas.getByRole("combobox", { name: "Paste order" })).toHaveTextContent("Newest first");
  },
};

export const Empty: Story = {
  args: { options: [] },
  play: async ({ canvas }) => {
    const trigger = canvas.getByRole("combobox", { name: "Paste order" });
    await expect(trigger).toBeDisabled();
    await expect(trigger).toHaveTextContent("No options");
  },
};

export const Locked: Story = {
  args: { value: "newest", locked: true },
  play: async ({ canvas }) => {
    await expect(canvas.getByRole("combobox", { name: "Paste order" })).toBeDisabled();
  },
};
