import type { Meta, StoryObj } from "@storybook/react-vite";
import { expect, within } from "storybook/test";
import { SourceRow } from "./SourceRow";

const meta = {
  title: "Rows/Source Row",
  component: SourceRow,
  args: {
    position: 1,
    text: "Use a 4pt spacing scale so components stay aligned...",
    fate: { kind: "point", number: 1 },
  },
  decorators: [
    (Story) => (
      <ul style={{ width: 520, margin: 0, padding: 0, listStyle: "none", background: "var(--color-surface)" }}>
        <Story />
      </ul>
    ),
  ],
} satisfies Meta<typeof SourceRow>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Point: Story = {
  play: async ({ canvasElement }) => {
    await expect(within(canvasElement).getByText("Point 1")).toBeVisible();
  },
};

export const Dropped: Story = {
  args: { position: 5, text: "~/Desktop/brand/logo-final.svg", fate: { kind: "dropped" } },
  play: async ({ canvasElement }) => {
    await expect(within(canvasElement).getByText("Dropped")).toBeVisible();
  },
};

export const PointDark: Story = {
  globals: { theme: "dark" },
};

export const DroppedDark: Story = {
  ...Dropped,
  globals: { theme: "dark" },
};
