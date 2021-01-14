# srtparse

A library for parsing [SRT Subtitles][1].

[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/rossnomann/srtparse/CI?style=flat-square)](https://github.com/rossnomann/srtparse/actions/)
[![Downloads](https://img.shields.io/crates/d/srtparse.svg?style=flat-square)](https://crates.io/crates/srtparse/)
[![Documentation](https://img.shields.io/badge/docs-latest-yellowgreen.svg?style=flat-square)](https://docs.rs/srtparse)

## Changelog

### 0.2.0 (30.03.2020)

- Switched to 2018 edition.
- Renamed `Subtitle` to `Item`.
- Changed type of subtitle's `start_time` and `end_time` to `Time`.
  (You still able to convert it to `Duration`.)
- Renamed `parse` function to `from_str`.
- Renamed `read_from_file` function to `from_file`.
- Added `from_reader` function.
- Removed `Result` alias.
- `Error` struct replaced by a bunch of different structs.

### 0.1.1 (04.12.2016)

- Fixed time parsing.

### 0.1.0 (03.12.2016)

- First release.

## LICENSE

The MIT License (MIT)

[1]: https://www.matroska.org/technical/subtitles.html#srt-subtitles
