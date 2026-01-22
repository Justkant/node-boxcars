import { CborDecoder } from "@jsonjoy.com/json-pack/lib/cbor";
import test from "ava";
import { readFile, readFileHeader } from "../index";

function parseResult(res: Uint8Array) {
  const dec = new CborDecoder();
  return dec.decode(res);
}

test("readFile from native", (t) => {
  const res = readFile(
    "./__test__/demos/61F74DFC4C42C4FB11F1FABECB0C561B.replay",
  );
  const json = parseResult(res);
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
    "./__test__/demos/61F74DFC4C42C4FB11F1FABECB0C561B.replay",
  );
  const json = parseResult(res);
  console.log(json);
  t.truthy(json);
  t.like(json, {
    game_type: "TAGame.Replay_Soccar_TA",
    properties: {
      TeamSize: 2,
    },
  });
});
