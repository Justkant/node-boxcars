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

fn parse_rl_header(data: &[u8]) -> Result<Replay, ParseError> {
  boxcars::ParserBuilder::new(data)
    .never_check_crc()
    .never_parse_network_data()
    .parse()
}

#[napi]
pub fn read_file(filename: String) -> Result<String> {
  let buffer = fs::read(filename)?;
  let replay = parse_rl(&buffer)?;
  serde_json::to_string(&replay).map_err(Error::from)
}

#[napi]
pub fn read_file_header(filename: String) -> Result<String> {
  let buffer = fs::read(filename)?;
  let replay = parse_rl_header(&buffer)?;
  serde_json::to_string(&replay).map_err(Error::from)
}
