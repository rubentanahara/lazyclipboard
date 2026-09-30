import type { Meta, StoryObj } from "@storybook/react-vite";
import { expect, fireEvent, fn, userEvent } from "storybook/test";
import { ShortcutField } from "./ShortcutField";
import { withStoryFrame } from "./storyFrame";

const meta = {
  title: "Forms/Shortcut Field",
  component: ShortcutField,
  decorators: [withStoryFrame],
  args: {
    label: "Copy to group shortcut",
    value: "⌘⌥C",
    onChange: fn(),
  },
} satisfies Meta<typeof ShortcutField>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const Unset: Story = {
  args: { value: "" },
  play: async ({ canvas }) => {
    await expect(canvas.getByRole("button", { name: "Copy to group shortcut" })).toHaveTextContent("Not set");
  },
};

export const Recording: Story = {
  play: async ({ canvas }) => {
    await userEvent.click(canvas.getByRole("button", { name: "Copy to group shortcut" }));
    await expect(canvas.getByRole("button", { name: "Copy to group shortcut" })).toHaveTextContent(
      "Press a shortcut",
    );
  },
};

export const RecordsChord: Story = {
  play: async ({ canvas, args }) => {
    await userEvent.click(canvas.getByRole("button", { name: "Copy to group shortcut" }));
    await userEvent.keyboard("{Control>}{Alt>}k{/Alt}{/Control}");
    await expect(args.onChange).toHaveBeenCalledTimes(1);
    await expect(args.onChange).toHaveBeenCalledWith(expect.stringMatching(/K$/));
  },
};

export const RecordsPhysicalKeyWhenOptionChangesTheCharacter: Story = {
  play: async ({ canvas, args }) => {
    const field = canvas.getByRole("button", { name: "Copy to group shortcut" });
    await userEvent.click(field);
    await fireEvent.keyDown(field, { key: "˚", code: "KeyK", altKey: true, ctrlKey: true });
    await expect(args.onChange).toHaveBeenCalledWith(expect.stringMatching(/K$/));
  },
};

export const IgnoresKeyWithoutModifier: Story = {
  play: async ({ canvas, args }) => {
    await userEvent.click(canvas.getByRole("button", { name: "Copy to group shortcut" }));
    await userEvent.keyboard("k");
    await expect(args.onChange).not.toHaveBeenCalled();
    await expect(canvas.getByRole("button", { name: "Copy to group shortcut" })).toHaveTextContent(
      "Press a shortcut",
    );
  },
};

export const EscapeCancelsRecording: Story = {
  play: async ({ canvas, args }) => {
    await userEvent.click(canvas.getByRole("button", { name: "Copy to group shortcut" }));
    await userEvent.keyboard("{Escape}");
    await expect(args.onChange).not.toHaveBeenCalled();
    await expect(canvas.getByRole("button", { name: "Copy to group shortcut" })).toHaveTextContent("⌘⌥C");
  },
};

export const Conflict: Story = {
  args: { conflict: "Already used by Spotlight." },
  play: async ({ canvas }) => {
    await expect(canvas.getByRole("alert")).toHaveTextContent("Already used by Spotlight.");
  },
};

export const Locked: Story = {
  args: { locked: true },
  play: async ({ canvas }) => {
    await expect(canvas.getByRole("button", { name: "Copy to group shortcut" })).toBeDisabled();
  },
};
