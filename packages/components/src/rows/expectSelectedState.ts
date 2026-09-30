import { expect } from "storybook/test";

function resolveColour(colour: string) {
  const probe = document.createElement("span");
  probe.style.color = colour;
  document.body.append(probe);
  const resolved = getComputedStyle(probe).color;
  probe.remove();
  return resolved;
}

export function tokenColour(element: Element, name: string) {
  return resolveColour(getComputedStyle(element).getPropertyValue(name).trim());
}

export async function expectSelectedState(row: HTMLElement) {
  await expect(getComputedStyle(row).backgroundColor).toBe(tokenColour(row, "--color-accent-soft"));

  const bar = row.querySelector("[data-part='bar']");
  await expect(bar).not.toBeNull();
  await expect(getComputedStyle(bar as Element).width).toBe("2px");
  await expect(getComputedStyle(bar as Element).backgroundColor).toBe(tokenColour(row, "--color-accent"));

  const icon = row.querySelector("svg");
  await expect(getComputedStyle(icon as Element).color).toBe(tokenColour(row, "--color-accent"));
}
