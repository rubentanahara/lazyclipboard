import { defineConfig, devices } from "@playwright/test";

const WINDOWS = ["panel", "main", "settings", "onboarding"];
const PREVIEW_PORT = 4173;
const PREVIEW_URL = `http://localhost:${PREVIEW_PORT}`;
const isCi = Boolean(process.env.CI);

export default defineConfig({
  testDir: ".",
  outputDir: "test-results",
  forbidOnly: isCi,
  reporter: isCi ? [["github"], ["list"]] : "list",
  use: { ...devices["Desktop Chrome"], trace: "retain-on-failure" },
  projects: WINDOWS.map((name) => ({
    name,
    testMatch: ["harness.spec.ts", `${name}/**/*.spec.ts`],
    use: { baseURL: `${PREVIEW_URL}/src/windows/${name}/` },
  })),
  webServer: {
    command: `pnpm exec vite build && pnpm exec vite preview --port ${PREVIEW_PORT} --strictPort`,
    cwd: "..",
    url: `${PREVIEW_URL}/src/windows/panel/index.html`,
    timeout: 120_000,
  },
});
