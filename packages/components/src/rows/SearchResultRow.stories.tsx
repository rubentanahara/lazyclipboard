import type { Meta, StoryObj } from "@storybook/react-vite";
import { expect, within } from "storybook/test";
import { expectSelectedState } from "./expectSelectedState";
import { SearchResultRow } from "./SearchResultRow";

const meta = {
  title: "Rows/Search Result Row",
  component: SearchResultRow,
  args: {
    kind: "text",
    title: "Use a 4pt spacing scale so components...",
    match: "spacing",
    meta: "2m ago",
    group: "Design refs",
  },
  decorators: [
    (Story) => (
      <div role="listbox" aria-label="Search results" style={{ width: 640, background: "var(--color-surface)" }}>
        <Story />
      </div>
    ),
  ],
} satisfies Meta<typeof SearchResultRow>;

export default meta;

type Story = StoryObj<typeof meta>;

const selectedRowShowsBarAndAccentIcon: Story["play"] = async ({ canvasElement }) => {
  await expectSelectedState(within(canvasElement).getByRole("option", { selected: true }));
};

export const Default: Story = {
  play: async ({ canvasElement }) => {
    const match = within(canvasElement).getByText("spacing");
    await expect(match.tagName).toBe("MARK");
    await expect(getComputedStyle(match).textDecorationLine).toBe("underline");
  },
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

export const NoMatchInTitle: Story = {
  args: { match: "absent" },
};

export const Link: Story = {
  args: { kind: "link", title: "...material.io/foundations/layout/spacing", group: "Links to read", meta: "Yesterday" },
};
