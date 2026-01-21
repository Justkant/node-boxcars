#![deny(clippy::all)]
use boxcars::{ParseError, Replay};
use std::fs;
use napi_derive::napi;

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
pub fn read_file(filename: String) -> napi::Result<String> {
  let buffer = fs::read(filename).map_err(|e| napi::Error::from_reason(e.to_string()))?;
  let replay = parse_rl(&buffer).map_err(|e| napi::Error::from_reason(e.to_string()))?;
  serde_json::to_string(&replay).map_err(|e| napi::Error::from_reason(e.to_string()))
}

#[napi]
pub fn read_file_header(filename: String) -> napi::Result<String> {
  let buffer = fs::read(filename).map_err(|e| napi::Error::from_reason(e.to_string()))?;
  let replay = parse_rl_header(&buffer).map_err(|e| napi::Error::from_reason(e.to_string()))?;
  serde_json::to_string(&replay).map_err(|e| napi::Error::from_reason(e.to_string()))
}
