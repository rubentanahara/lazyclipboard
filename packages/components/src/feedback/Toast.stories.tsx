import type { Meta, StoryObj } from "@storybook/react-vite";
import { useState } from "react";
import { expect, fn, userEvent, within } from "storybook/test";
import { Toast } from "./Toast";

const meta = {
  title: "Feedback/Toast",
  component: Toast,
  args: { open: true, message: "Item deleted", onOpenChange: fn() },
  decorators: [
    (Story) => (
      <div className="relative h-40">
        <Story />
      </div>
    ),
  ],
} satisfies Meta<typeof Toast>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Undo: Story = {
  args: { actionLabel: "Undo", onAction: fn(), detail: "6s" },
  play: async ({ canvasElement, args }) => {
    await userEvent.click(within(canvasElement).getByRole("button", { name: "Undo" }));
    await expect(args.onAction).toHaveBeenCalledOnce();
  },
};

export const Paused: Story = {
  args: { actionLabel: "Undo", detail: "6s" },
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.hover(canvas.getByText("Item deleted"));
    await expect(await canvas.findByText("Paused")).toBeVisible();
  },
};

export const Info: Story = {
  args: { message: "11 items removed", detail: "Cleanup" },
};

export const ReopenedAfterPause: Story = {
  args: { actionLabel: "Undo", detail: "6s" },
  render: (args) => {
    const [open, setOpen] = useState(true);
    return (
      <>
        <button type="button" onClick={() => setOpen(true)}>
          Reopen
        </button>
        <Toast {...args} open={open} onOpenChange={setOpen} />
      </>
    );
  },
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.hover(canvas.getByText("Item deleted"));
    await expect(await canvas.findByText("Paused")).toBeVisible();
    await userEvent.click(canvas.getByRole("button", { name: "Undo" }));
    await userEvent.click(canvas.getByRole("button", { name: "Reopen" }));
    await expect(await canvas.findByText("6s")).toBeVisible();
    await expect(canvas.queryByText("Paused")).toBeNull();
  },
};
