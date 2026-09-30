import type { Meta, StoryObj } from "@storybook/react-vite";
import { expect, fn, screen, userEvent, waitFor } from "storybook/test";
import { ConfirmDialog } from "./ConfirmDialog";

const meta = {
  title: "Feedback/Confirm Dialog",
  component: ConfirmDialog,
  args: {
    open: true,
    onOpenChange: fn(),
    onConfirm: fn(),
    title: "Delete group “Work”?",
    description: "12 items will be deleted. This cannot be undone.",
    confirmLabel: "Delete group",
  },
} satisfies Meta<typeof ConfirmDialog>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  play: async () => {
    const dialog = await screen.findByRole("alertdialog", { name: "Delete group “Work”?" });
    await expect(dialog).toHaveAccessibleDescription("12 items will be deleted. This cannot be undone.");
    await waitFor(() => expect(screen.getByRole("button", { name: "Cancel" })).toHaveFocus());
  },
};

export const Confirm: Story = {
  play: async ({ args }) => {
    await userEvent.click(await screen.findByRole("button", { name: "Delete group" }));
    await expect(args.onConfirm).toHaveBeenCalledOnce();
  },
};

export const Cancel: Story = {
  play: async ({ args }) => {
    await userEvent.click(await screen.findByRole("button", { name: "Cancel" }));
    await expect(args.onOpenChange).toHaveBeenCalledWith(false);
    await expect(args.onConfirm).not.toHaveBeenCalled();
  },
};
