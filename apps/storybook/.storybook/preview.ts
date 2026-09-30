import type { Decorator, Preview } from "@storybook/react-vite";
import "@lazyclipboard/tokens/tokens.css";
import { installMockIpcWhenTauriIsAbsent } from "../../ui/src/shared/mock";
import "./preview.css";

installMockIpcWhenTauriIsAbsent();

const withTheme: Decorator = (Story, context) => {
  document.documentElement.dataset.theme = context.globals.theme;
  return Story();
};

const preview: Preview = {
  decorators: [withTheme],
  globalTypes: {
    theme: {
      description: "Colour theme",
      toolbar: {
        title: "Theme",
        icon: "circlehollow",
        items: [
          { value: "light", title: "Light" },
          { value: "dark", title: "Dark" },
        ],
        dynamicTitle: true,
      },
    },
  },
  initialGlobals: { theme: "light" },
  parameters: { a11y: { test: "error" } },
};

export default preview;
