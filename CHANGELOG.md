# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

## [0.4.0]
### Added
- Add trait `Capacity<T>`. This is used in `where` bounds now instead of `ArrayLength<T>`.

### Changed
- Un-deprecate this crate.
- Make `GenericArrayVec`/`GenericArrayString` smaller. The internal length field is now optimally sized based on capacity
instead of being hardcoded to `usize`. Unfortunately some extra `where`-bound boilerplate is needed now; see the crate
docs.
- Rename `Wrapper` to `ArrayvecStorageRaw` (though the also added type alias `ArrayvecStorage` should be used).

### Fixed
- Fix no_std compatibility.

## [0.3.1] - 2021-08-05
### Deprecated
- This crate is now deprecated.

## [0.3.0] - 2020-05-19
### Added
- Add feature reexports for arrayvec and generic-array's features allowing more array sizes.

### Changed
- Upgrade arrayvec to 0.5.
- Upgrade generic-array to 0.14.

## [0.2.0] - 2019-04-30
### Changed
- Upgrade generic-array to 0.13.

## [0.1.0] - 2018-08-29
### Changed
- Upgrade generic-array to 0.12.

## [0.0.2] - 2018-07-15

## [0.0.1] - 2018-17-14
Initial release.
