import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { renderTokensCss } from "./render-tokens-css.js";

const designPath = new URL("../../../DESIGN.md", import.meta.url);
const outputDirectory = new URL("../dist/", import.meta.url);

mkdirSync(outputDirectory, { recursive: true });
writeFileSync(new URL("tokens.css", outputDirectory), renderTokensCss(readFileSync(designPath, "utf8")));
