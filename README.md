# `@kant/node-boxcars`

![https://github.com/Justkant/node-boxcars/actions](https://github.com/Justkant/node-boxcars/workflows/CI/badge.svg)

Node.js bindings for the [boxcars](https://github.com/nickbabcock/boxcars) parser. Read Rocket League `.replay` files and get back JSON.

## Installation

```bash
yarn add @kant/node-boxcars
# or
npm install @kant/node-boxcars
```

## Usage

```js
import { readFile, readFileHeader } from "@kant/node-boxcars";

// Parse the full replay (including network frames)
const replay = JSON.parse(readFile("./path/to/match.replay"));
console.log(replay.game_type);

// Parse only the header for quick metadata access
const header = JSON.parse(readFileHeader("./path/to/match.replay"));
console.log(header.properties.TeamSize);
```

Both functions synchronously return a JSON string that matches the `boxcars::Replay` structure.

## Development

- Install the latest Rust toolchain
- Install Node.js 10+ (Node-API) and yarn/corepack
- Build: `yarn build`
- Test: `yarn test`

## Release

Set `NPM_TOKEN` in the repository secrets before tagging a release.

```bash
npm version [patch|minor|major]
git push
```

GitHub Actions will publish the prebuilt binaries for supported platforms.
