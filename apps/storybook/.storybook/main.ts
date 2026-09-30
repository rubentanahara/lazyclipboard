import type { StorybookConfig } from "@storybook/react-vite";

const config: StorybookConfig = {
  framework: "@storybook/react-vite",
  stories: [
    "../../../packages/components/src/**/*.stories.@(ts|tsx)",
    "../../ui/src/windows/**/*.stories.@(ts|tsx)",
    "../src/**/*.stories.@(ts|tsx)",
  ],
  addons: ["@storybook/addon-a11y", "@storybook/addon-vitest"],
};

export default config;
