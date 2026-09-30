import type { Meta, StoryObj } from "@storybook/react-vite";
import { KeyCap } from "./KeyCap";

const meta = {
  title: "Controls/Key Cap",
  component: KeyCap,
  args: { children: "⌘1" },
} satisfies Meta<typeof KeyCap>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const Chord: Story = { args: { children: "⌘⌥C" } };

export const Return: Story = { args: { children: "↵" } };
