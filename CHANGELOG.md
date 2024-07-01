# Mutant-Kraken Changelog

All notable changes to this project will be documented in this file.

For guidance on how to write a good changelog, see [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

For quick reference:

Types of changes

- **Added** for new features.
- **Changed** for changes in existing functionality.
- **Deprecated** for soon-to-be removed features.
- **Removed** for now removed features.
- **Fixed** for any bug fixes.
- **Security** in case of vulnerabilities.

## [0.1.4] - 2024-07-01

### Fixed

- Fixed assets not being properly copied to the target directory when building the project.

## [0.1.3] - 2024-07-01

### Security

- Updated `tree-sitter` to 0.22.6
- Updated `tree-sitter-kotlin` to 0.3.6

### Changed

- Changed how the `KotlinTypes` enum is created by utilizing Rust's Procedural macros
- Updated the structure of the project by created a workspace and moving the project into a `mutant-kraken` subdirectory
- Updated `config.yml` to use the new structure of the project

### Added

- Added a new crate called `mutant-kraken-macros` which contains the procedural macros used to generate the `KotlinTypes` enum

### Removed

- Removed the `KotlinTypes::LineStringLiteral` variant that was used throughout the project. This type no longer exists in the `kotlin-tree-sitter` AST.

## [0.1.2] - 2024-07-01

### Added

- Added publishing for Linux using homebrew
- Added publishing to cargo crates.io within ci

## [0.1.1-beta] - 2024-09-05

### Fixed

- Fixed bug where tool would crash because of missing kotlin types in the AST. This was caused by the tool being moved to the most recent version of `kotlin-tree-sitter` without updating the AST types in the mutation operators. The version of `kotlin-tree-sitter` has been downgraded to the previous version to fix this issue. Will come back to this issue in a future release.
- Fixed a bug where the tool would crash because of unwrapping a null value while trying to fine `LabelRemoveOperator` mutations. This was cause by a bad condition in an if statement.
- Fixed an issue where string literals were not mutated correctly.

## [0.1.0-beta] - 2024-09-04

Welcome to the first release of Mutant-Kraken! This is a beta release, so please report any issues you find.

### Added

- Initial release of Mutant-Kraken
- 7 new Kotlin-specific mutation operators
- 8 traditional mutation operators
- Ability to configure the tool using a json file
- Updated README with installation and usage instructions
- ... and more!
