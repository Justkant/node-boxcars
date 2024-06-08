// @ts-check
import test from "ava";

import { readFile, readFileHeader } from "../index.js";

test("readFile from native", (t) => {
  const res = readFile(
    "./__test__/demos/61F74DFC4C42C4FB11F1FABECB0C561B.replay"
  );
  const json = JSON.parse(res);
  console.log(json);
  t.truthy(json);
  t.like(json, {
    game_type: "TAGame.Replay_Soccar_TA",
    properties: {
      TeamSize: 2,
    },
  });
});

test("readFileHeader from native", (t) => {
  const res = readFileHeader(
    "./__test__/demos/61F74DFC4C42C4FB11F1FABECB0C561B.replay"
  );
  const json = JSON.parse(res);
  console.log(json);
  t.truthy(json);
  t.like(json, {
    game_type: "TAGame.Replay_Soccar_TA",
    properties: {
      TeamSize: 2,
    },
  });
});
