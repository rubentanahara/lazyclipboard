import { parse } from "yaml";

const FRONT_MATTER = /^---\n([\s\S]*?)\n---/;
const TYPOGRAPHY_PROPERTIES = {
  fontFamily: "family",
  fontSize: "size",
  fontWeight: "weight",
  lineHeight: "line-height",
};

export function renderTokensCss(designMarkdown) {
  const match = FRONT_MATTER.exec(designMarkdown);
  if (!match) throw new Error("DESIGN.md has no front matter");
  const spec = parse(match[1]);

  const lightBlock = block(":root", [
    ...colorDeclarations(spec.colors.light),
    ...typographyDeclarations(spec.typography),
    ...scaleDeclarations("space", spec.spacing),
    ...scaleDeclarations("rounded", spec.rounded),
    ...scaleDeclarations("elevation", spec.elevation),
  ]);
  const darkBlock = block('[data-theme="dark"]', colorDeclarations(spec.colors.dark));

  return `${lightBlock}\n${darkBlock}`;
}

function block(selector, declarations) {
  return `${selector} {\n${declarations.map((line) => `  ${line}\n`).join("")}}\n`;
}

function colorDeclarations(colors) {
  return Object.entries(colors).map(([name, value]) => `--color-${name}: ${value};`);
}

function typographyDeclarations(typography) {
  return Object.entries(typography).flatMap(([role, properties]) =>
    Object.entries(TYPOGRAPHY_PROPERTIES).map(
      ([property, suffix]) => `--text-${role}-${suffix}: ${properties[property]};`,
    ),
  );
}

function scaleDeclarations(prefix, scale) {
  return Object.entries(scale).map(([name, value]) => `--${prefix}-${name}: ${withPixelUnit(value)};`);
}

function withPixelUnit(value) {
  return typeof value === "number" ? `${value}px` : value;
}
