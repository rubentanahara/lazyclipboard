import AxeBuilder from "@axe-core/playwright";
import { expect, test as base } from "@playwright/test";

const BLOCKING_IMPACTS = ["serious", "critical"];
const WCAG_TAGS = ["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"];

type Fixtures = {
  expectNoSeriousAxeViolations: () => Promise<void>;
};

export const test = base.extend<Fixtures>({
  expectNoSeriousAxeViolations: async ({ page }, use) => {
    await use(async () => {
      const { violations } = await new AxeBuilder({ page }).withTags(WCAG_TAGS).analyze();
      const blocking = violations.filter(({ impact }) => BLOCKING_IMPACTS.includes(impact ?? ""));
      expect(blocking).toEqual([]);
    });
  },
});

export { expect };
