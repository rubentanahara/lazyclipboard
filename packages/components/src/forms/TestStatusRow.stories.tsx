import type { Meta, StoryObj } from "@storybook/react-vite";
import { expect, fn, userEvent } from "storybook/test";
import { TestStatusRow } from "./TestStatusRow";
import { withStoryFrame } from "./storyFrame";

const meta = {
  title: "Forms/Test Status Row",
  component: TestStatusRow,
  decorators: [withStoryFrame],
  args: { status: "idle", onTest: fn() },
} satisfies Meta<typeof TestStatusRow>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Idle: Story = {
  play: async ({ canvas, args }) => {
    await userEvent.click(canvas.getByRole("button", { name: "Test connection" }));
    await expect(args.onTest).toHaveBeenCalledTimes(1);
  },
};

export const Testing: Story = {
  args: { status: "testing" },
  play: async ({ canvas }) => {
    await expect(canvas.getByRole("button", { name: "Testing…" })).toBeDisabled();
    await expect(canvas.getByRole("status")).toHaveTextContent("Testing…");
  },
};

export const Failed: Story = {
  args: { status: "failed", message: "The provider rejected this key." },
  play: async ({ canvas, args }) => {
    await expect(canvas.getByRole("status")).toHaveTextContent("The provider rejected this key.");
    await userEvent.click(canvas.getByRole("button", { name: "Retry" }));
    await expect(args.onTest).toHaveBeenCalledTimes(1);
  },
};

export const Succeeded: Story = {
  args: { status: "succeeded" },
  play: async ({ canvas }) => {
    await expect(canvas.getByRole("status")).toHaveTextContent("Connection works");
  },
};

export const MissingKey: Story = {
  args: { status: "blocked", reason: "Add an API key to test the connection." },
  play: async ({ canvas }) => {
    await expect(canvas.getByRole("button", { name: "Test connection" })).toBeDisabled();
    await expect(canvas.getByRole("status")).toHaveTextContent("Add an API key to test the connection.");
  },
};
