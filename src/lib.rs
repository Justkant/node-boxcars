#![deny(clippy::all)]
use boxcars::{ParseError, Replay};
use minicbor_serde;
use napi::bindgen_prelude::Uint8Array;
use napi_derive::napi;
use std::fs;

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
pub fn read_file(filename: String) -> napi::Result<Uint8Array> {
  let buffer = fs::read(filename).map_err(|e| napi::Error::from_reason(e.to_string()))?;
  let replay = parse_rl(&buffer).map_err(|e| napi::Error::from_reason(e.to_string()))?;
  let vec = minicbor_serde::to_vec(&replay).map_err(|e| napi::Error::from_reason(e.to_string()))?;
  Ok(Uint8Array::from(vec))
}

#[napi]
pub fn read_file_header(filename: String) -> napi::Result<Uint8Array> {
  let buffer = fs::read(filename).map_err(|e| napi::Error::from_reason(e.to_string()))?;
  let replay = parse_rl_header(&buffer).map_err(|e| napi::Error::from_reason(e.to_string()))?;
  let vec = minicbor_serde::to_vec(&replay).map_err(|e| napi::Error::from_reason(e.to_string()))?;
  Ok(Uint8Array::from(vec))
}
