const { spawnSync } = require("child_process");
const { readFileSync, writeFileSync, rmSync } = require("fs");
const { join, relative } = require("path");

const targetDir = {
  rust: join(__dirname, "../src/words"),
  cpp: join(__dirname, "../../cpp"),
};

const readFile = (f) => readFileSync(f, "utf8").trim().split("\n");
function writeFile(filepath, content, fmt = () => {}) {
  rmSync(filepath, { recursive: true, force: true });
  writeFileSync(filepath, content);
  console.log(`Generated ${relative(__dirname, filepath)}!`);
  try {
    fmt();
  } catch {}
}

const build = {};
const GUESSES = readFile("guesses");
const ANSWERS = readFile("answers");
GUESSES.sort();
ANSWERS.sort();

/**
 * @param {string} filename
 */
build.rust = (filename, words) => {
  const content =
    `pub const ${filename.toUpperCase()}: [&[u8; 5]; ${words.length}] = [` +
    words.map((v) => `b"${v}"`).join(",") +
    "];";
  const filepath = join(targetDir.rust, filename + ".rs");
  const fmt = () => spawnSync("rustfmt", [filepath], { cwd: targetDir.rust });
  writeFile(filepath, content, fmt);
};

build.cpp = () => {
  const frame = (guesses, answers, G, A) => `\
#include "words.h"

const char Words::ANSWERS[][6]{${answers}};
const char Words::GUESSES[][6]{${guesses}};
const int Words::ANSWER_COUNT = ${A};
const int Words::GUESS_COUNT = ${G};
`;
  const [G, A] = [GUESSES, ANSWERS].map((v) => v.length)
  let [guesses, answers] = [GUESSES, ANSWERS].map(JSON.stringify);
  guesses = guesses.replace("[", "").replace("]", "");
  answers = answers.replace("[", "").replace("]", "");
  const content = frame(guesses, answers, G, A);
  const filepath = join(targetDir.cpp, "words.cc");
  writeFile(filepath, content);
};

build.rust("guesses", GUESSES);
build.rust("answers", ANSWERS);
build.cpp();
