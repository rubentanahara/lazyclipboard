import type { Meta, StoryObj } from "@storybook/react-vite";
import { expect, within } from "storybook/test";
import { StateBox } from "./StateBox";

const meta = {
  title: "Feedback/State Box",
  component: StateBox,
  args: { variant: "empty", title: "Nothing here yet", message: "Copy something to see it here." },
} satisfies Meta<typeof StateBox>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Empty: Story = {};

export const Error: Story = {
  args: { variant: "error", title: "Could not load items", message: "Check your connection and retry." },
  play: async ({ canvasElement }) => {
    const box = within(canvasElement).getByRole("alert");
    await expect(within(box).getByText("Could not load items")).toBeVisible();
    await expect(box.querySelector("svg")).not.toBeNull();
  },
};

export const NoKey: Story = {
  args: { variant: "no-key", title: "No API key", message: "Add a key in Settings to use AI." },
};
