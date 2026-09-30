import type { Meta, StoryObj } from "@storybook/react-vite";
import { Folder, Plus } from "lucide-react";
import { expect, within } from "storybook/test";
import { expectSelectedState, tokenColour } from "./expectSelectedState";
import { NavItem } from "./NavItem";

const meta = {
  title: "Rows/Nav Item",
  component: NavItem,
  args: { icon: Folder, label: "Design refs", count: 12 },
  decorators: [
    (Story) => (
      <nav aria-label="Groups" style={{ width: 240, background: "var(--color-surface-alt)" }}>
        <Story />
      </nav>
    ),
  ],
} satisfies Meta<typeof NavItem>;

export default meta;

type Story = StoryObj<typeof meta>;

const selectedItemShowsBarAndAccentIcon: Story["play"] = async ({ canvasElement }) => {
  await expectSelectedState(within(canvasElement).getByRole("button", { current: true }));
};

export const Default: Story = {};

export const SelectedLight: Story = {
  args: { selected: true },
  globals: { theme: "light" },
  play: selectedItemShowsBarAndAccentIcon,
};

export const SelectedDark: Story = {
  args: { selected: true },
  globals: { theme: "dark" },
  play: selectedItemShowsBarAndAccentIcon,
};

export const WithoutCount: Story = {
  args: { count: undefined },
};

export const NewGroup: Story = {
  args: { icon: Plus, label: "New group", count: undefined, tone: "accent" },
  play: async ({ canvasElement }) => {
    const label = within(canvasElement).getByText("New group");
    await expect(getComputedStyle(label).color).toBe(tokenColour(label, "--color-accent"));
  },
};
