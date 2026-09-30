import type { Meta, StoryObj } from "@storybook/react-vite";
import { Chip } from "./Chip";

const meta = {
  title: "Controls/Chip",
  component: Chip,
  args: { children: "12" },
} satisfies Meta<typeof Chip>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Count: Story = {};

export const Provider: Story = { args: { children: "Anthropic" } };
