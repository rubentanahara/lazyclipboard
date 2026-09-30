import type { Meta, StoryObj } from "@storybook/react-vite";
import { SegmentedControl } from "./SegmentedControl";
import { SettingsRow } from "./SettingsRow";
import { Stepper } from "./Stepper";
import { withStoryFrame } from "./storyFrame";

const meta = {
  title: "Forms/Settings Row",
  component: SettingsRow,
  decorators: [withStoryFrame],
  args: { label: "History size", children: null },
} satisfies Meta<typeof SettingsRow>;

export default meta;

type Story = StoryObj<typeof meta>;

export const WithStepper: Story = {
  args: {
    children: <Stepper label="History size" value={5} min={1} max={10} onChange={() => undefined} />,
  },
};

export const WithDescription: Story = {
  args: {
    label: "Theme",
    description: "System follows your operating system.",
    children: (
      <SegmentedControl
        label="Theme"
        value="system"
        options={[
          { value: "light", label: "Light" },
          { value: "dark", label: "Dark" },
          { value: "system", label: "System" },
        ]}
        onValueChange={() => undefined}
      />
    ),
  },
};

export const Locked: Story = {
  args: {
    description: "Managed by the group settings.",
    children: <Stepper label="History size" value={5} min={1} max={10} onChange={() => undefined} locked />,
  },
};
