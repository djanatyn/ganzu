///! organize files according to rules
///!
///! testing:
///! TODO: add test directory
///! TODO: test parsing example manifest
///! TODO: test generating / parsing kdl for Meta
///! TODO: test generating / parsing kdl for FileSnapshot
///!
///! control flow:
///! TODO: fail if unable to snapshot files
///! TODO: fail if unable to match files
///!
///! output:
///! TODO: report files successfully snapshotted
///! TODO: report files matched
///! TODO: output kdl manifest
///!
pub mod config;
pub mod error;
pub mod example;
pub mod file;
pub mod filter;
mod kdl;
pub mod rule;
