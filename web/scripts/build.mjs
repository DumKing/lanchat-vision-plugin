import { cp, mkdir, rm } from "node:fs/promises";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const dist = join(root, "dist");
await rm(dist, { recursive: true, force: true });
await mkdir(dist, { recursive: true });
await cp(join(root, "public", "index.html"), join(dist, "index.html"));
await cp(join(root, "assets"), join(dist, "assets"), { recursive: true });
for (const file of ["main.js", "styles.css"]) await cp(join(root, "src", file), join(dist, file));
console.log(`视觉插件页面已构建: ${dist}`);
