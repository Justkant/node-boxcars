import { Bench } from "tinybench";
import { readFile, readFileHeader } from "../index.js";

const b = new Bench();

b.add("readFile", () => {
  readFile("./__test__/demos/61F74DFC4C42C4FB11F1FABECB0C561B.replay");
});

b.add("readFileHeader", () => {
  readFileHeader("./__test__/demos/61F74DFC4C42C4FB11F1FABECB0C561B.replay");
});

await b.run();

console.table(b.table());
