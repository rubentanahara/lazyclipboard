import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { test } from "node:test";
import { renderTokensCss } from "./render-tokens-css.js";

const MINI_SPEC = `---
name: mini
colors:
  light:
    surface: "#FFFFFF"
    text: "#16181D"
  dark:
    surface: "#171A21"
    text: "#E6E8EC"
typography:
  body: { fontFamily: Inter, fontSize: 13px, fontWeight: 500, lineHeight: 1.4 }
spacing: { 2xs: 2px, md: 12px }
rounded: { sm: 6px, pill: 9999px }
elevation: { menu-y: 12, menu-blur: 32 }
motion:
  duration: { fast: 100ms }
  easing: { standard: "cubic-bezier(0.2, 0, 0, 1)" }
---

# Body is ignored
`;

const MINI_CSS = `:root {
  --color-surface: #FFFFFF;
  --color-text: #16181D;
  --text-body-family: Inter;
  --text-body-size: 13px;
  --text-body-weight: 500;
  --text-body-line-height: 1.4;
  --space-2xs: 2px;
  --space-md: 12px;
  --rounded-sm: 6px;
  --rounded-pill: 9999px;
  --elevation-menu-y: 12px;
  --elevation-menu-blur: 32px;
  --motion-duration-fast: 100ms;
  --motion-easing-standard: cubic-bezier(0.2, 0, 0, 1);
}

[data-theme="dark"] {
  --color-surface: #171A21;
  --color-text: #E6E8EC;
}
`;

const designMarkdown = readFileSync(new URL("../../../DESIGN.md", import.meta.url), "utf8");

function blockOf(css, selector) {
  const start = css.indexOf(`${selector} {`);
  return css.slice(start, css.indexOf("}", start));
}

test("renders a literal spec to the exact CSS", () => {
  assert.equal(renderTokensCss(MINI_SPEC), MINI_CSS);
});

test("renders the real design spec with light and dark values that differ", () => {
  const css = renderTokensCss(designMarkdown);
  const light = blockOf(css, ":root");
  const dark = blockOf(css, '[data-theme="dark"]');

  assert.match(light, /--color-surface: #FFFFFF;/);
  assert.match(dark, /--color-surface: #171A21;/);
  assert.match(light, /--color-accent: #4F46E5;/);
  assert.match(dark, /--color-accent: #8B87FF;/);
  assert.match(light, /--space-md: 12px;/);
  assert.match(light, /--rounded-xl: 14px;/);
  assert.match(light, /--text-body-size: 13px;/);
  assert.match(light, /--elevation-panel-blur: 40px;/);
  assert.match(light, /--motion-duration-fast: 100ms;/);
  assert.match(light, /--motion-duration-base: 160ms;/);
  assert.match(light, /--motion-duration-slow: 240ms;/);
  assert.match(light, /--motion-easing-standard: cubic-bezier\(0\.2, 0, 0, 1\);/);
});

test("declares every light colour in dark too", () => {
  const css = renderTokensCss(designMarkdown);
  const colorNames = (selector) =>
    [...blockOf(css, selector).matchAll(/--color-([a-z0-9-]+):/g)].map(([, name]) => name);

  assert.deepEqual(colorNames('[data-theme="dark"]'), colorNames(":root"));
});

test("renders a spec with Windows line endings", () => {
  assert.equal(renderTokensCss(MINI_SPEC.replaceAll("\n", "\r\n")), MINI_CSS);
});

test("fails when the spec has no front matter", () => {
  assert.throws(() => renderTokensCss("# no front matter"), /front matter/);
});
