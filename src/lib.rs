use anyhow::{Error, Result};
use boxcars::{ParseError, Replay};
use std::fs;

#[macro_use]
extern crate napi_derive;

fn parse_rl(data: &[u8]) -> Result<Replay, ParseError> {
  boxcars::ParserBuilder::new(data)
    .must_parse_network_data()
    .parse()
}

#[napi]
pub fn read_file(filename: String) -> Result<String> {
  let buffer = fs::read(filename)?;
  let replay = parse_rl(&buffer)?;
  serde_json::to_string(&replay).map_err(Error::from)
}

#[napi]
pub fn read_files(filenames: Vec<String>) -> Result<String> {
  let mut replays: Vec<Replay> = Vec::new();

  for filename in &filenames {
    let buffer = fs::read(filename)?;
    replays.push(parse_rl(&buffer)?);
  }
  serde_json::to_string(&replays).map_err(Error::from)
}
