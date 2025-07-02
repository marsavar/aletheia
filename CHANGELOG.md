# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)

## [1.0.0] - 2025-07-02

### Changed
- [**BREAKING CHANGE**] `GuardianContentClient` no longer needs to be mutable. To account for this change, requests must be built with the `.build_request()` method to be called on the client. An example can be found in the `README.md` file.
The motivation for this change is to be able to share the client between multiple threads without resorting to synchronisation primitives.
- The client no longer prints to stderr if the Guardian Content API responds with an error. The client now returns an `Error` variant depending on the type of error, so that downstream consumers can deal with the errors as they wish.
- Removed the `aletheia::Result<T>` type.
- Upgraded several dependencies.

### Added
- Added MSRV to the project

### Fixed
- The `pages` field in the `SearchResponse` struct is now of type `Option<isize>` (previously `Option<u32>`) to account for the API responding with `-1` when setting the page size to `0` on the client.

## [0.1.6] - 2022-12-28

### Added
- Structs and enums now implement the `Clone` trait.

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
