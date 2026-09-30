import type { Decorator } from "@storybook/react-vite";

export const withStoryFrame: Decorator = (Story) => (
  <div className="bg-[var(--color-surface)] p-[var(--space-lg)]">
    <Story />
  </div>
);
