import type { Meta, StoryObj } from "@storybook/react-vite";
import { expect, within } from "storybook/test";
import { CapturedStrip } from "./CapturedStrip";

const meta = {
  title: "Rows/Captured Strip",
  component: CapturedStrip,
  args: {
    kind: "text",
    text: "Use a 4pt spacing scale so components stay aligned across breakpoints and density modes.",
    source: "Safari",
  },
  decorators: [
    (Story) => (
      <div style={{ width: 520 }}>
        <Story />
      </div>
    ),
  ],
} satisfies Meta<typeof CapturedStrip>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(canvas.getByText("CAPTURED", { exact: false })).toBeVisible();
    await expect(canvas.getByText("Text · Safari")).toBeVisible();
  },
};

export const Dark: Story = {
  globals: { theme: "dark" },
};

export const Link: Story = {
  args: { kind: "link", text: "https://linear.app/method", source: "Chrome" },
};
