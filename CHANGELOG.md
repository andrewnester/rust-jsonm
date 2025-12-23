# Changelog

All notable changes to this project will be documented in this file.

## [0.2.1] - 2025-12-23

### Fixed
- Fixed panic when unpacking empty objects (`{}`)
- Fixed null values being corrupted during pack/unpack cycle due to dictionary index mismatch
- Fixed boolean values not being properly memoized in unpacker

### Added
- Added tests for empty object handling
- Added tests for null and boolean value memoization

## [0.2.0] - Previous release

(Add previous release notes here if available)
