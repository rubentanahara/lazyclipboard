import type { Meta, StoryObj } from "@storybook/react-vite";
import { Folder } from "lucide-react";
import { expect, fn, userEvent, within } from "storybook/test";
import { PanelHeader, PanelHeaderDetail } from "./PanelHeader";

const meta = {
  title: "Feedback/Panel Header",
  component: PanelHeader,
  args: { icon: Folder, title: "Work" },
} satisfies Meta<typeof PanelHeader>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  play: async ({ canvasElement }) => {
    await expect(within(canvasElement).getByRole("heading", { name: "Work" })).toBeVisible();
  },
};

export const Detail: Story = {
  render: (args) => <PanelHeaderDetail title={args.title} onBack={fn()} count={<span>12</span>} />,
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(canvas.getByRole("heading", { name: "Work" })).toBeVisible();
    await expect(canvas.getByText("12")).toBeVisible();
    await userEvent.click(canvas.getByRole("button", { name: "Back" }));
  },
};
