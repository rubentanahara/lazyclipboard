import type { Meta, StoryObj } from "@storybook/react-vite";

const meta = {
  title: "Harness/Sample",
  render: () => (
    <p
      style={{
        margin: 0,
        padding: "var(--space-lg)",
        background: "var(--color-surface)",
        color: "var(--color-text)",
        fontFamily: "var(--text-body-family)",
        fontSize: "var(--text-body-size)",
      }}
    >
      Storybook harness sample
    </p>
  ),
} satisfies Meta;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};
