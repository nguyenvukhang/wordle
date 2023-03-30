const { readFileSync, writeFileSync } = require("fs");

/**
 * @param {string} filename
 */
function build(filename) {
  const words = readFileSync(filename, "utf8").trim().split("\n");
  const len = words.length;
  const pre = `pub const ${filename.toUpperCase()}: [&[u8; 5]; ${len}] = [`;
  const content = pre + words.map((v) => `b"${v}"`).join(",") + "];";
  writeFileSync(filename + ".rs", content);
}

build("guesses");
build("answers");
