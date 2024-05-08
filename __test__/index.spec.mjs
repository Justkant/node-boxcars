// @ts-check
import test from "ava";

import { readFile, readFiles } from "../index.js";

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

test("readFiles from native", (t) => {
  const res = readFiles([
    "./__test__/demos/61F74DFC4C42C4FB11F1FABECB0C561B.replay",
    "./__test__/demos/930965114253E9682DDFDE8747C9C8E2.replay",
  ]);
  const json = JSON.parse(res);
  t.truthy(json);
  t.is(json.length, 2);
});
