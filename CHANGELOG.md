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
