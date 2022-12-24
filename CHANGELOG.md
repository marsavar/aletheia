# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)

## [0.1.5] - 2022-12-24

### Fixed
- Fixed incorrect documentation examples for the `tag_type` and `show_blocks` functions, as well as a typo in the `datetime_to` documentation.

### Changed
- `datetime_to` and `datetime_from` can no longer panic when providing invalid values. Instead, the API call will simply ignore these query parameters and not include them in the request.
- Structs no longer implement the `Serialize` trait.
- Improved documentation by using implied shortcut reference links.
- Updated dependencies.


## [0.1.4] - 2022-12-23

### Added
- Added the `Result<T>` type. This is a wrapper around Rust's core `Result<T, E>` type that makes errors default to Aletheia's own `Error` type.

## [0.1.3] - 2022-04-18

### Added
- Added `BodyText` to the `Field` enum. This allows the article body to be displayed with no HTMLin the response (credit to @invokermain for spotting the lacking field).
