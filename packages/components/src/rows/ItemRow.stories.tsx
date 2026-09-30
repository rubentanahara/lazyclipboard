import type { Meta, StoryObj } from "@storybook/react-vite";
import { within } from "storybook/test";
import { expectSelectedState } from "./expectSelectedState";
import { ItemRow } from "./ItemRow";

const meta = {
  title: "Rows/Item Row",
  component: ItemRow,
  args: {
    kind: "text",
    title: "Use a 4pt spacing scale so components stay aligned across breakpoints.",
    meta: "Safari · 2m ago",
  },
  decorators: [
    (Story) => (
      <div role="listbox" aria-label="Items" style={{ width: 640, background: "var(--color-surface)" }}>
        <Story />
      </div>
    ),
  ],
} satisfies Meta<typeof ItemRow>;

export default meta;

type Story = StoryObj<typeof meta>;

const selectedRowShowsBarAndAccentIcon: Story["play"] = async ({ canvasElement }) => {
  await expectSelectedState(within(canvasElement).getByRole("option", { selected: true }));
};

export const Default: Story = {
  render: (args) => <ItemRow {...args} />,
};

export const SelectedLight: Story = {
  args: { selected: true },
  globals: { theme: "light" },
  play: selectedRowShowsBarAndAccentIcon,
};

export const SelectedDark: Story = {
  args: { selected: true },
  globals: { theme: "dark" },
  play: selectedRowShowsBarAndAccentIcon,
};

export const Link: Story = {
  args: { kind: "link", title: "https://linear.app/method", meta: "Chrome · 1h ago" },
};

export const File: Story = {
  args: { kind: "file", title: "~/Desktop/brand/logo-final.svg", meta: "Finder · Yesterday" },
};

export const Formatted: Story = {
  args: { formatted: true },
};

export const Thumbnail: Story = {
  args: {
    kind: "image",
    title: "Screenshot 2026-09-30 at 12.11.04",
    meta: "Figma · 14m ago",
    thumbnailSrc: "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='1' height='1'/%3E",
  },
};
