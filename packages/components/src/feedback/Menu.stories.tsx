import type { Meta, StoryObj } from "@storybook/react-vite";
import { Copy, Pencil } from "lucide-react";
import { expect, fn, screen, userEvent } from "storybook/test";
import { Menu, MenuItem } from "./Menu";

const meta = {
  title: "Feedback/Menu",
  component: Menu,
  args: {
    open: true,
    trigger: <button type="button">Actions</button>,
    children: null,
  },
} satisfies Meta<typeof Menu>;

export default meta;
type Story = StoryObj<typeof meta>;

const onCopy = fn();
const onDelete = fn();

export const Default: Story = {
  args: {
    children: (
      <>
        <MenuItem icon={Pencil} label="Rename" onSelect={fn()} />
        <MenuItem icon={Copy} label="Copy" onSelect={onCopy} />
        <MenuItem variant="danger" label="Delete group…" onSelect={onDelete} />
      </>
    ),
  },
  play: async () => {
    await screen.findByRole("menu");
    await userEvent.keyboard("{ArrowDown}{ArrowDown}");
    await expect(screen.getByRole("menuitem", { name: "Copy" })).toHaveAttribute("data-highlighted");
    await userEvent.keyboard("{Enter}");
    await expect(onCopy).toHaveBeenCalledOnce();
  },
};

export const Danger: Story = {
  args: { children: <MenuItem variant="danger" label="Delete group…" onSelect={onDelete} /> },
  play: async () => {
    const item = await screen.findByRole("menuitem", { name: "Delete group…" });
    await expect(item.querySelector("svg")).not.toBeNull();
  },
};
