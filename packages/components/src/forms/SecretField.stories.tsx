import type { Meta, StoryObj } from "@storybook/react-vite";
import { expect, fn, userEvent } from "storybook/test";
import { MASKED_PLACEHOLDER, SecretField } from "./SecretField";
import { withStoryFrame } from "./storyFrame";

const meta = {
  title: "Forms/Secret Field",
  component: SecretField,
  decorators: [withStoryFrame],
  args: {
    label: "Anthropic API key",
    hasSavedKey: false,
    onSave: fn(),
  },
} satisfies Meta<typeof SecretField>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Empty: Story = {
  play: async ({ canvas, args }) => {
    await userEvent.type(canvas.getByLabelText("Anthropic API key"), "sk-test-123");
    await userEvent.click(canvas.getByRole("button", { name: "Save" }));
    await expect(args.onSave).toHaveBeenCalledWith("sk-test-123");
  },
};

export const DraftStaysOutOfTheMarkup: Story = {
  play: async ({ canvas }) => {
    const input = canvas.getByLabelText("Anthropic API key");
    await userEvent.type(input, "sk-test-123");
    await expect(input).toHaveValue("sk-test-123");
    await expect(input.getAttribute("value")).toBeNull();
  },
};

export const Saved: Story = {
  args: { hasSavedKey: true },
  play: async ({ canvas, canvasElement }) => {
    await expect(canvas.getByText(MASKED_PLACEHOLDER)).toBeVisible();
    await expect(canvasElement.querySelector("input")).toBeNull();
    await userEvent.click(canvas.getByRole("button", { name: "Replace key" }));
    await expect(canvas.getByLabelText("Anthropic API key")).toHaveValue("");
  },
};

export const Saving: Story = {
  args: { saving: true },
  play: async ({ canvas }) => {
    await expect(canvas.getByLabelText("Anthropic API key")).toBeDisabled();
    await expect(canvas.getByRole("button", { name: "Saving…" })).toBeDisabled();
  },
};

export const Error: Story = {
  args: { error: "The provider rejected this key." },
  play: async ({ canvas }) => {
    await expect(canvas.getByRole("alert")).toHaveTextContent("The provider rejected this key.");
  },
};

export const Locked: Story = {
  args: { hasSavedKey: true, locked: true },
  play: async ({ canvas }) => {
    await expect(canvas.getByRole("button", { name: "Replace key" })).toBeDisabled();
  },
};
