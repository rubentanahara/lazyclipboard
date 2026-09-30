import type { Meta, StoryObj } from "@storybook/react-vite";
import { expect, within } from "storybook/test";
import { FooterHint } from "./FooterHint";

const StoryKeyCap = ({ children }: { children: string }) => (
  <kbd className="rounded-(--rounded-sm) bg-(--color-surface-alt) px-(--space-sm) py-(--space-2xs) text-(length:--text-caption-size) text-(--color-text-2)">
    {children}
  </kbd>
);

const meta = {
  title: "Feedback/Footer Hint",
  component: FooterHint,
  args: { shortcut: <StoryKeyCap>↵</StoryKeyCap>, label: "Paste" },
} satisfies Meta<typeof FooterHint>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(canvas.getByText("↵")).toBeVisible();
    await expect(canvas.getByText("Paste")).toBeVisible();
  },
};

export const Chord: Story = {
  args: { shortcut: <StoryKeyCap>⌘↵</StoryKeyCap>, label: "Paste all" },
};
