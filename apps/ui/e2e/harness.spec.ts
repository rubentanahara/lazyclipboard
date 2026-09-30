import { expect, test } from "./fixtures";

const SEEDED_GROUP_COUNT = 20;

test("the window loads on the IPC mock and passes the axe scan", async ({ page, expectNoSeriousAxeViolations }) => {
  await page.goto("");

  const groups = await page.evaluate(() => {
    const internals = (window as unknown as { __TAURI_INTERNALS__: { invoke: (command: string) => Promise<unknown[]> } })
      .__TAURI_INTERNALS__;
    return internals.invoke("groups_list");
  });

  expect(groups).toHaveLength(SEEDED_GROUP_COUNT);
  await expectNoSeriousAxeViolations();
});

test.fail("the axe fixture rejects a critical violation", async ({ page, expectNoSeriousAxeViolations }) => {
  await page.setContent('<!doctype html><html lang="en"><title>t</title><main><img src="missing.png"></main></html>');

  await expectNoSeriousAxeViolations();
});
