import type { Meta, StoryObj } from "@storybook/react-vite";
import { expect, within } from "storybook/test";
import { GroupTag } from "./GroupTag";

const meta = {
  title: "Rows/Group Tag",
  component: GroupTag,
  args: { name: "Design refs" },
} satisfies Meta<typeof GroupTag>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  play: async ({ canvasElement }) => {
    await expect(within(canvasElement).getByText("Design refs")).toBeVisible();
  },
};

export const Dark: Story = {
  globals: { theme: "dark" },
};
