const { spawnSync } = require("child_process");
const { readFileSync, writeFileSync, rmSync } = require("fs");
const { join, relative } = require("path");

const TARGET_DIR = join(__dirname, "../src/words");

const build = {};

/**
 * @param {string} filename
 */
build.rust = (filename) => {
  const words = readFileSync(filename, "utf8").trim().split("\n");
  const content =
    `pub const ${filename.toUpperCase()}: [&[u8; 5]; ${words.length}] = [` +
    words.map((v) => `b"${v}"`).join(",") +
    "];";
  const filepath = join(TARGET_DIR, filename + ".rs");
  rmSync(filepath, { recursive: true, force: true });
  writeFileSync(filepath, content);
  console.log(`Generated ${relative(__dirname, filepath)}!`);
  try {
    spawnSync("rustfmt", [filepath], { cwd: TARGET_DIR });
  } catch {}
};

build.cpp = () => {};

build.rust("guesses");
build.rust("answers");
